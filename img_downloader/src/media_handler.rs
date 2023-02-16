use crate::media_config::MediaConfig;
use std::fs;
use std::path::PathBuf;

/*
Add a strategy with a trait to handle different API or local downlloading
https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html
 */

pub struct MediaHandler {
    pub config: MediaConfig,
    pub media_list: Vec<PathBuf>,
}

impl MediaHandler {
    pub fn new(config: MediaConfig) -> Self {
        let ml = fs::read_dir("./").unwrap();
        MediaHandler {
            config,
            media_list: ml,
        }
    }

    // pub fn get_next_image() -> DynamicImage {
    //     yield img;
    // }
}
