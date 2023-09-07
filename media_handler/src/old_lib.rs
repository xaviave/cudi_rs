pub mod frame;
pub mod media_config;
pub mod media_source_api;
pub mod new_lib;
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

    media_asked: Arc<Mutex<i32>>,
    thread_counter: Arc<Mutex<i32>>,
    // channel to communicate with
    tx_graphic: Sender<Frame>,
    rx_graphic: Receiver<u8>,

    // channel to communicate with
    tx_downloader: Sender<Frame>,
    rx_downloader: Receiver<Frame>,

    // channel to communicate with
    tx_path_handler: Sender<Vec<PathBuf>>,
    rx_path_handler: Receiver<Vec<PathBuf>>,
}

impl MediaHandler {
    fn get_next_media(thread_counter: Arc<Mutex<i32>>, tx: Sender<Frame>, media_path: PathBuf) {
        thread::spawn(move || {
            let mut num = thread_counter.lock().unwrap();
            *num += 1;
            let f = Frame::new(media_path);
            // println!("Before sending: {}", f);
            tx.send(f).unwrap();
            // println!("After sending");
        });
    }

    fn query_path_queue(
        tx: Sender<Vec<PathBuf>>,
        media_source: Arc<MediaSource>,
        config: Arc<MediaConfig>,
    ) {
        /*
        Spawn a thread to request a new media list than will extend the current one
        */
        let c = Arc::clone(&config);
        let ms = Arc::clone(&media_source);

        println!("avant");
        thread::spawn(move || {
            tx.send(ms.get_media_list(c)).unwrap();
        });
        println!("apres");
    }

    fn get_async_path_queue(&self) -> Vec<PathBuf> {
        let min_paths = 2 * ((self.config.max_threads as usize) - self.media_queue.len());
        if self.path_queue.len() < min_paths {
            Self::query_path_queue(
                self.tx_path_handler.clone(),
                Arc::clone(&self.media_source),
                Arc::clone(&self.config),
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
        media_source: Arc<MediaSource>,
        config: Arc<MediaConfig>,
    ) -> Vec<PathBuf> {
        Self::query_path_queue(tx, media_source, config);
        match rx.recv() {
            Ok(p) => p,
            Err(_) => vec![],
        }
    }

    pub fn new(config: MediaConfig, tx_graphic: Sender<Frame>, rx_graphic: Receiver<u8>) -> Self {
        let media_source = Arc::new(MediaSource::Local(LocalMedia::new(&config)));
        // let mut media_source = MediaSource::DB(PostgreSQLMedia::new(&config));

        let c = Arc::new(config);
        let media_asked = Arc::new(Mutex::new(1));
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
                    Arc::clone(&media_source),
                    Arc::clone(&c),
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
            Arc::clone(&media_source),
            Arc::clone(&c),
        ));
        MediaHandler {
            config: c,
            media_source,
            path_queue,
            media_queue,
            media_asked,
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
        // update the number of media asked, will be the 'size' of the queue
        let mut media_asked = self.media_asked.lock().unwrap();
        *media_asked = *media_asked + signal as i32;
        println!("media asked: {signal:?} | total need {:?}", *media_asked);

        // for _ in 0..signal {
        //     println!("size: {}", self.media_queue.len());
        //     if let Some(f) = self.media_queue.pop() {
        //         match self.tx_graphic.send(f) {
        //             Ok(_) => (),
        //             Err(e) => println!("{}", e),
        //         }
        //     }
        // }
    }

    fn fill_path_queue(&mut self) {
        if self.path_queue.len() < self.config.max_threads as usize {
            self.path_queue.extend(self.get_async_path_queue());
        }
    }

    fn fill_media_queue(&mut self, mut total: i64) -> i64 {
        let binding = Arc::clone(&self.media_asked);
        let mut media_asked = binding.lock().unwrap();

        if *media_asked == 0 {
            return total;
        }

        for _ in 0..*media_asked {
            if let Some(f) = self.media_queue.pop() {
                match self.tx_graphic.send(f) {
                    Ok(_) => {
                        *media_asked -= 1;
                        total += 1;
                    }
                    Err(e) => println!("{}", e),
                }
            } else {
                let max_media = std::cmp::min(
                    self.path_queue.len(),
                    (self.config.max_threads as usize) - self.media_queue.len(),
                );
                for _ in 0..max_media {
                    let p = self.path_queue.pop().unwrap();
                    Self::get_next_media(
                        Arc::clone(&self.thread_counter),
                        self.tx_downloader.clone(),
                        p,
                    );
                    match self.rx_downloader.try_recv() {
                        Ok(f) => {
                            let mut num = self.thread_counter.lock().unwrap();
                            *num -= 1;
                            self.media_queue.push(f)
                        }
                        Err(_) => {
                            print!(".");
                        }
                    };
                }
                self.fill_path_queue();
            }
        }
        total

        // // launch media_needed threads that will handle one Frame processing
        // for _ in 0..max_media {
        //     let p = self.path_queue.pop().unwrap();
        //     Self::get_next_media(
        //         Arc::clone(&self.thread_counter),
        //         self.tx_downloader.clone(),
        //         p,
        //     );
        // }

        // // Try to received all Frames from the processing threads launched before
        // for _ in 0..max_media {
        //     match self.rx_downloader.try_recv() {
        //         Ok(f) => {
        //             let mut num = self.thread_counter.lock().unwrap();
        //             *num -= 1;
        //             self.media_queue.push(f)
        //         }
        //         Err(_) => {
        //             println!("wainting for new image");
        //         }
        //     };
        // }
    }

    pub fn run(&mut self) {
        let mut total = 0;
        loop {
            println!("loop debut");
            if let Ok(v) = self.rx_graphic.try_recv() {
                // Kill the thread
                if v == 0 {
                    return;
                }
                self.handle_signal(v);
            };

            self.fill_path_queue();
            total = self.fill_media_queue(total);
            println!("total after one iteration: {total:?}");
        }
    }
}
