use crate::media_config::MediaConfig;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::PathBuf;

/*
Add a strategy with a trait to handle different API or local downlloading
also for video, gif or image for the next_media iterator

https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html
 */

fn vector_to_shuffle_iter(mut x: Vec<PathBuf>) -> Box<dyn Iterator<Item = PathBuf>> {
    let mut rng = thread_rng();
    x.shuffle(&mut rng);
    Box::new(x.into_iter())
}

// #[derive(Debug)]
pub struct MediaHandler {
    pub config: MediaConfig,
    pub media_list: Vec<PathBuf>,
    pub media_iter: Box<dyn Iterator<Item = PathBuf>>,
}

impl MediaHandler {
    pub fn new(config: MediaConfig) -> Self {
        let ml: Vec<PathBuf> = fs::read_dir(&config.data_folder)
            .unwrap()
            .map(|p| p.unwrap().path())
            .collect();
        MediaHandler {
            config,
            media_list: ml.clone(),
            media_iter: vector_to_shuffle_iter(ml),
        }
    }

    pub fn get_next_media(&mut self) -> PathBuf {
        match self.media_iter.next() {
            Some(media) => media,
            None => {
                self.media_iter = vector_to_shuffle_iter(self.media_list.clone());
                self.get_next_media()
            }
        }
    }
}
