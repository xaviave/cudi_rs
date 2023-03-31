pub mod frame;
pub mod media_config;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Instant;
use std::{fs, thread};

use frame::Frame;
use media_config::MediaConfig;

/*
Add a strategy with a trait to handle different API or local downloading
also for video, gif or image for the next_media iterator

https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html
 */

#[derive(Debug)]
pub struct MediaHandler {
    pub config: MediaConfig,
    pub media_paths: Vec<PathBuf>,
    pub path_queue: Vec<PathBuf>,
    pub media_queue: Vec<Frame>,
    tx_graphic: Sender<Frame>,
    rx_graphic: Receiver<u8>,
    tx_downloader: Sender<Frame>,
    rx_downloader: Receiver<Frame>,
}

impl MediaHandler {
    fn shuffle_vec(mut x: Vec<PathBuf>) -> Vec<PathBuf> {
        let mut rng = thread_rng();
        x.shuffle(&mut rng);
        x
    }

    fn get_next_media(tx: Sender<Frame>, media_path: PathBuf) {
        thread::spawn(move || {
            tx.send(Frame::new(media_path)).unwrap();
        });
    }

    pub fn new(config: MediaConfig, tx_graphic: Sender<Frame>, rx_graphic: Receiver<u8>) -> Self {
        let mp: Vec<PathBuf> = fs::read_dir(&config.data_folder)
            .unwrap()
            .map(|p| p.unwrap().path())
            .filter(|f| f.is_file())
            .collect();

        let (tx_downloader, rx_downloader) = mpsc::channel();
        let mut path_queue = Self::shuffle_vec(mp.clone());
        let mut media_queue: Vec<Frame> = vec![];
        for _ in 0..config.max_threads {
            if path_queue.len() < 1 {
                path_queue = Self::shuffle_vec(mp.clone());
            }
            let p = path_queue.pop().unwrap();
            Self::get_next_media(tx_downloader.clone(), p);
        }

        for _ in 0..config.max_threads {
            match rx_downloader.recv() {
                Ok(f) => media_queue.push(f),
                Err(_) => (),
            };
        }
        MediaHandler {
            config,
            media_paths: mp,
            path_queue,
            media_queue,
            tx_graphic,
            rx_graphic,
            tx_downloader,
            rx_downloader,
        }
    }

    pub fn run(&mut self) {
        loop {
            let signal: u8 = match self.rx_graphic.try_recv() {
                Ok(v) => v,
                Err(_) => 0,
            };

            if signal == 1 {
                println!("Received {}", signal);
                self.tx_graphic
                    .send(self.media_queue.pop().unwrap())
                    .unwrap();
            }

            if self.path_queue.len() < ((self.config.max_threads as usize) - self.media_queue.len())
            {
                self.path_queue = Self::shuffle_vec(self.media_paths.clone());
            }
            for _ in 0..(self.config.max_threads - (self.media_queue.len() as u32)) {
                let p = self.path_queue.pop().unwrap();
                Self::get_next_media(self.tx_downloader.clone(), p);
            }
            for _ in 0..(self.config.max_threads - (self.media_queue.len() as u32)) {
                match self.rx_downloader.try_recv() {
                    Ok(f) => self.media_queue.push(f),
                    Err(_) => (),
                };
            }
        }
    }
}
