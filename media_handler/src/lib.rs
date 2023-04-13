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
    pub config: MediaConfig,
    pub media_source: MediaSource,
    pub path_queue: Vec<PathBuf>,
    pub media_queue: Vec<Frame>,

    thread_counter: Arc<Mutex<i32>>,
    tx_graphic: Sender<Frame>,
    rx_graphic: Receiver<u8>,
    tx_downloader: Sender<Frame>,
    rx_downloader: Receiver<Frame>,
}

impl MediaHandler {
    fn get_next_media(thread_counter: Arc<Mutex<i32>>, tx: Sender<Frame>, media_path: PathBuf) {
        thread::spawn(move || {
            let mut num = thread_counter.lock().unwrap();
            *num += 1;
            tx.send(Frame::new(media_path)).unwrap();
        });
    }

    pub fn new(config: MediaConfig, tx_graphic: Sender<Frame>, rx_graphic: Receiver<u8>) -> Self {
        // move media_paths to media_source
        let mut media_source = MediaSource::Local(LocalMedia::new(&config));
        // let mut media_source = MediaSource::DB(PostgreSQLMedia::new(&config));

        let thread_counter = Arc::new(Mutex::new(0));
        let mut media_queue: Vec<Frame> = vec![];
        let (tx_downloader, rx_downloader) = mpsc::channel();
        let mut path_queue = media_source.get_media_list(&config);

        for _ in 0..config.max_threads {
            if path_queue.len() < 1 {
                path_queue = media_source.get_media_list(&config);
            }
            let p = path_queue.pop().unwrap();
            let c = Arc::clone(&thread_counter);
            Self::get_next_media(c, tx_downloader.clone(), p);
        }

        for _ in 0..config.max_threads {
            match rx_downloader.recv() {
                Ok(f) => {
                    let mut num = thread_counter.lock().unwrap();
                    *num -= 1;
                    media_queue.push(f)
                }
                Err(_) => (),
            };
        }
        MediaHandler {
            config,
            media_source,
            path_queue,
            media_queue,
            thread_counter,
            tx_graphic,
            rx_graphic,
            tx_downloader,
            rx_downloader,
        }
    }

    fn handle_signal(&mut self, signal: u8) {
        if signal == 1 {
            self.tx_graphic
                .send(self.media_queue.pop().unwrap())
                .unwrap();
        }
    }

    fn fill_media_queue(&mut self) {
        let num = *Arc::clone(&self.thread_counter).lock().unwrap();
        let media_needed = std::cmp::min(
            self.config.max_threads,
            self.config.max_threads - (self.media_queue.len() as u32),
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

            if self.path_queue.len() < ((self.config.max_threads as usize) - self.media_queue.len())
            {
                self.path_queue = self.media_source.get_media_list(&self.config);
            }
            self.fill_media_queue();
        }
    }
}
