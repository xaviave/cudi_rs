use crate::media_config::MediaConfig;
use crate::Frame;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

/*
Add a strategy with a trait to handle different API or local downlloading
also for video, gif or image for the next_media iterator

https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html
 */

// #[derive(Debug)]
pub struct MediaHandler {
    pub config: MediaConfig,
    pub media_list: Vec<PathBuf>,
    pub media_iter: Box<dyn Iterator<Item = PathBuf>>,
}

impl MediaHandler {
    fn vector_to_shuffle_iter(mut x: Vec<PathBuf>) -> Box<dyn Iterator<Item = PathBuf>> {
        let mut rng = thread_rng();
        x.shuffle(&mut rng);
        Box::new(x.into_iter())
    }
    pub fn new(config: MediaConfig) -> Self {
        let ml: Vec<PathBuf> = fs::read_dir(&config.data_folder)
            .unwrap()
            .map(|p| p.unwrap().path())
            .filter(|f| f.is_file())
            .collect();
        MediaHandler {
            config,
            media_list: ml.clone(),
            media_iter: Self::vector_to_shuffle_iter(ml),
        }
    }

    pub fn get_next_media(&mut self) -> Frame {
        match self.media_iter.next() {
            Some(media) => {
                let timer = Instant::now();
                let f = Frame::new(media);
                println!("LOG: opening image time: {:?}", timer.elapsed().as_millis());
                f
            }
            None => {
                self.media_iter = Self::vector_to_shuffle_iter(self.media_list.clone());
                self.get_next_media()
            }
        }
    }
}
