pub mod frame;
pub mod media_config;
pub mod media_source_api;
pub mod schema;
pub mod sql_models;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::frame::Frame;
use crate::media_config::MediaConfig;
use crate::media_source_api::{LocalMedia, MediaSource, PostgreSQLMedia};

pub struct MediaHandler {
    pub config: Arc<MediaConfig>,
    pub media_source: Arc<MediaSource>,

    pub path_queue: Vec<PathBuf>,

    timer: Instant,
    media_asked: Arc<Mutex<i32>>,
    thread_counter: Arc<Mutex<i32>>,
    thread_handlers: HashMap<Duration, JoinHandle<()>>,
    // channel to communicate with
    tx_graphic: Sender<Frame>,
    rx_graphic: Receiver<u8>,

    // channel to communicate with
    tx_threads_cleaner: Sender<Duration>,
    rx_threads_cleaner: Receiver<Duration>,

    // channel to communicate with
    tx_path_handler: Sender<Vec<PathBuf>>,
    rx_path_handler: Receiver<Vec<PathBuf>>,
}

impl MediaHandler {
    pub fn new(config: MediaConfig, tx_graphic: Sender<Frame>, rx_graphic: Receiver<u8>) -> Self {
        let media_source = Arc::new(MediaSource::Local(LocalMedia::new(&config)));
        // let mut media_source = MediaSource::DB(PostgreSQLMedia::new(&config));

        let timer = Instant::now();
        let c = Arc::new(config);
        let media_asked = Arc::new(Mutex::new(-2));
        let thread_counter = Arc::new(Mutex::new(3));
        let (tx_threads_cleaner, rx_threads_cleaner) = mpsc::channel();
        let (tx_path_handler, rx_path_handler) = mpsc::channel();

        let mut thread_handlers: HashMap<Duration, JoinHandle<()>> = HashMap::new();
        Self::query_media_path(
            timer,
            media_source.clone(),
            tx_path_handler.clone(),
            tx_threads_cleaner.clone(),
            &mut thread_handlers,
        );
        let mut path_queue = rx_path_handler.recv().unwrap();

        for _ in 0..c.max_threads {
            if let Some(p) = path_queue.pop() {
                Self::process_media(
                    p,
                    timer,
                    tx_graphic.clone(),
                    tx_threads_cleaner.clone(),
                    &mut thread_handlers,
                );
            } else {
                match rx_path_handler.try_recv() {
                    Ok(paths) => {
                        path_queue.extend(paths);
                    }
                    Err(_) => {
                        print!("Error in Thread query_media_path.");
                    }
                };
                Self::query_media_path(
                    timer,
                    media_source.clone(),
                    tx_path_handler.clone(),
                    tx_threads_cleaner.clone(),
                    &mut thread_handlers,
                );
            }
        }

        Self {
            config: c,
            media_source,

            timer,
            media_asked,
            thread_counter,
            thread_handlers,

            path_queue,

            tx_graphic,
            rx_graphic,
            tx_threads_cleaner,
            rx_threads_cleaner,
            tx_path_handler,
            rx_path_handler,
        }
    }

    fn process_media(
        media_path: PathBuf,
        timer: Instant,
        tx_graphic: Sender<Frame>,
        tx_threads_cleaner: Sender<Duration>,
        thread_handlers: &mut HashMap<Duration, JoinHandle<()>>,
    ) {
        let hashed = timer.elapsed();
        thread_handlers.insert(
            hashed,
            thread::spawn(move || {
                // open and process media
                let f = Frame::new(media_path);
                // add the frame object to the queue
                tx_graphic.send(f).unwrap();
                tx_threads_cleaner.send(hashed).unwrap();
            }),
        );
    }

    fn query_media_path(
        timer: Instant,
        media_source: Arc<MediaSource>,
        tx_path_handler: Sender<Vec<PathBuf>>,
        tx_thread_cleaner: Sender<Duration>,
        thread_handlers: &mut HashMap<Duration, JoinHandle<()>>,
    ) {
        let hashed = timer.elapsed();
        thread_handlers.insert(
            hashed,
            thread::spawn(move || {
                tx_path_handler.send(media_source.get_media_list()).unwrap();
                tx_thread_cleaner.send(hashed).unwrap();
            }),
        );
    }

    fn handle_signal(&mut self, signal: u8) {
        let media_asked_arc = self.media_asked.clone();
        // update the number of media asked, will be the 'size' of the queue
        let mut media_asked = media_asked_arc.lock().unwrap();
        *media_asked += signal as i32;
        println!("received | media_asked: {}", *media_asked);
    }

    fn handle_media_query(&mut self, max_threads: i32) {
        let media_asked_arc = self.media_asked.clone();
        let mut ma = media_asked_arc.lock().unwrap();
        let thread_counter_arc = self.thread_counter.clone();
        let mut thread_number = thread_counter_arc.lock().unwrap();

        // launch the max number of threads available
        while *ma > 0 && *thread_number < max_threads {
            // try to get a path to launch the media processing
            if let Some(p) = self.path_queue.pop() {
                Self::process_media(
                    p,
                    self.timer,
                    self.tx_graphic.clone(),
                    self.tx_threads_cleaner.clone(),
                    &mut self.thread_handlers,
                );
                *ma -= 1;
                *thread_number += 1;
            } else {
                // no path available, fill again the queue
                match self.rx_path_handler.try_recv() {
                    Ok(paths) => {
                        self.path_queue.extend(paths);
                        Self::query_media_path(
                            self.timer,
                            self.media_source.clone(),
                            self.tx_path_handler.clone(),
                            self.tx_threads_cleaner.clone(),
                            &mut self.thread_handlers,
                        );
                        *thread_number += 1;
                    }
                    Err(_) => {
                        print!("[handle_media_query] - Can't find paths");
                    }
                };
            }
        }

        // println!("[handle_media_query] - media_asked: {media_asked:?} | media_processed: {media_processed:?}");
        // extend media path to prepare the next query and try to avoid query while processing
        if *ma as usize > self.path_queue.len() {
            Self::query_media_path(
                self.timer,
                self.media_source.clone(),
                self.tx_path_handler.clone(),
                self.tx_threads_cleaner.clone(),
                &mut self.thread_handlers,
            );
            *thread_number += 1;
        }
    }

    fn clean(&mut self, max_threads: i32) {
        // println!("[clean]");
        for _ in 0..max_threads {
            match self.rx_threads_cleaner.try_recv() {
                Ok(d) => {
                    let binding = self.thread_counter.clone();
                    let mut thread_number = binding.lock().unwrap();
                    *thread_number -= 1;
                    self.thread_handlers.remove(&d);
                }
                Err(_) => (),
            }
        }
    }

    pub fn run(&mut self) {
        let max_threads = self.config.max_threads as i32;
        loop {
            if let Ok(v) = self.rx_graphic.try_recv() {
                // Kill signal
                if v == 0 {
                    // self.hard_clean();
                    break;
                }
                self.handle_signal(v);
            };
            self.handle_media_query(max_threads);
            self.clean(max_threads);
        }
    }
}
