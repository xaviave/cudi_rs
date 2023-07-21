pub mod frame;
pub mod media_config;
pub mod media_source_api;
pub mod schema;
pub mod sql_models;

use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use frame::Frame;
use media_config::MediaConfig;
use media_source_api::{LocalMedia, MediaSource, PostgreSQLMedia};

pub struct MediaHandler {
    pub config: Arc<MediaConfig>,
    pub media_source: Arc<MediaSource>,
    pub path_queue: Vec<PathBuf>,
    pub media_queue: Vec<Frame>,

    thread_counter: Arc<Mutex<i32>>,
    tx_graphic: Sender<Frame>,
    rx_graphic: Receiver<u8>,
    tx_downloader: Sender<Frame>,
    rx_downloader: Receiver<Frame>,
    tx_path_handler: Sender<Vec<PathBuf>>,
    rx_path_handler: Receiver<Vec<PathBuf>>,
}

impl MediaHandler {
    fn get_next_media(thread_counter: Arc<Mutex<i32>>, tx: Sender<Frame>, media_path: PathBuf) {
        thread::spawn(move || {
            let mut num = thread_counter.lock().unwrap();
            *num += 1;
            tx.send(Frame::new(media_path)).unwrap();
        });
    }

    fn query_path_queue(
        tx: Sender<Vec<PathBuf>>,
        media_source: &Arc<MediaSource>,
        config: &Arc<MediaConfig>,
    ) {
        /*
        Spawn a thread to request a new media list than will extend the current one
        */
        let c = Arc::clone(&config);
        let ms = Arc::clone(&media_source);

        thread::spawn(move || {
            tx.send(ms.get_media_list(&*c)).unwrap();
        });
    }

    fn get_async_path_queue(&self) -> Vec<PathBuf> {
        let min_paths = 2 * ((self.config.max_threads as usize) - self.media_queue.len());
        if self.path_queue.len() < min_paths {
            Self::query_path_queue(
                self.tx_path_handler.clone(),
                &self.media_source,
                &self.config,
            );
        }
        match self.rx_path_handler.try_recv() {
            Ok(p) => p,
            Err(_) => vec![],
        }
    }

    fn get_sync_path_queue(
        tx: Sender<Vec<PathBuf>>,
        rx: &Receiver<Vec<PathBuf>>,
        media_source: &Arc<MediaSource>,
        config: &Arc<MediaConfig>,
    ) -> Vec<PathBuf> {
        Self::query_path_queue(tx.clone(), &media_source, config);
        match rx.recv() {
            Ok(p) => p,
            Err(_) => vec![],
        }
    }

    pub fn new(config: MediaConfig, tx_graphic: Sender<Frame>, rx_graphic: Receiver<u8>) -> Self {
        let media_source = Arc::new(MediaSource::Local(LocalMedia::new(&config)));
        // let mut media_source = MediaSource::DB(PostgreSQLMedia::new(&config));

        let c = Arc::new(config);
        let thread_counter = Arc::new(Mutex::new(0));
        let (tx_downloader, rx_downloader) = mpsc::channel();
        let (tx_path_handler, rx_path_handler) = mpsc::channel();

        let mut media_queue: Vec<Frame> = vec![];
        let mut path_queue = vec![];
        for _ in 0..c.max_threads {
            if path_queue.len() == 0 {
                path_queue.extend(Self::get_sync_path_queue(
                    tx_path_handler.clone(),
                    &rx_path_handler,
                    &media_source,
                    &c,
                ));
            }
            Self::get_next_media(
                Arc::clone(&thread_counter),
                tx_downloader.clone(),
                path_queue.pop().unwrap(),
            );
        }
        for _ in 0..c.max_threads {
            match rx_downloader.recv() {
                Ok(f) => {
                    let mut num = thread_counter.lock().unwrap();
                    *num -= 1;
                    media_queue.push(f)
                }
                Err(_) => (),
            };
        }

        path_queue.extend(Self::get_sync_path_queue(
            tx_path_handler.clone(),
            &rx_path_handler,
            &media_source,
            &c,
        ));
        MediaHandler {
            config: c,
            media_source,
            path_queue,
            media_queue,
            thread_counter,
            tx_graphic,
            rx_graphic,
            tx_downloader,
            rx_downloader,
            tx_path_handler,
            rx_path_handler,
        }
    }

    fn handle_signal(&mut self, signal: u8) {
        for _ in 0..signal {
            self.tx_graphic
                .send(self.media_queue.pop().unwrap())
                .unwrap();
        }
    }

    fn fill_media_queue(&mut self) {
        let num = *Arc::clone(&self.thread_counter).lock().unwrap();
        let media_needed = std::cmp::min(
            self.path_queue.len(),
            (self.config.max_threads as usize) - self.media_queue.len(),
        );
        // if too many threads or no need of new media
        if num > self.config.max_threads as i32 || media_needed == 0 {
            return;
        }

        for _ in 0..media_needed {
            let p = self.path_queue.pop().unwrap();
            Self::get_next_media(
                Arc::clone(&self.thread_counter),
                self.tx_downloader.clone(),
                p,
            );
        }
        for _ in 0..media_needed {
            match self.rx_downloader.recv() {
                Ok(f) => {
                    let mut num = self.thread_counter.lock().unwrap();
                    *num -= 1;
                    self.media_queue.push(f)
                }
                Err(_) => (),
            };
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.rx_graphic.try_recv() {
                Ok(v) => self.handle_signal(v),
                Err(_) => (),
            };
            if self.path_queue.len() < 2 * (self.config.max_threads as usize) {
                self.path_queue.extend(self.get_async_path_queue());
            }
            self.fill_media_queue();
        }
    }
}
