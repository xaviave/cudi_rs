use std::path::PathBuf;

pub struct MediaConfig {
    pub fps: u8,
    pub data_folder: PathBuf,
}

impl MediaConfig {
    pub fn new(fps: u8, data_folder: PathBuf) -> Self {
        MediaConfig { fps, data_folder }
    }
}
