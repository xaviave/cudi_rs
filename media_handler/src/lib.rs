pub mod media_config;
pub mod media_handler;

use image;
use image::DynamicImage;
use image::GenericImageView;
use std::path::PathBuf;

pub struct Frame {
    pub path: PathBuf,
    pub data: DynamicImage,
}

impl Frame {
    pub fn new(p: PathBuf) -> Self {
        let data = image::open(&p).unwrap();
        Self { path: p, data }
    }

    pub fn print_debug(&self) {
        println!(
            "File description:\npath: {:?}\nsize: {:?}",
            self.path,
            self.data.dimensions()
        );
    }
}
