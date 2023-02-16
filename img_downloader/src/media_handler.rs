use crate::media_config::MediaConfig;

/*
Add a strategy with a trait to handle different API or local downlloading
https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html
 */

pub struct MediaHandler {
    pub config: MediaConfig,
}

impl MediaHandler {
    pub fn new(config: MediaConfig) -> Self {
        MediaHandler { config }
    }
}
