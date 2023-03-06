pub mod media_config;
pub mod media_handler;

use image::DynamicImage;
use image::GenericImageView;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub path: PathBuf,
    pub data: DynamicImage,
}

impl Frame {
    pub fn new(p: PathBuf) -> Self {
        let data = image::open(&p).unwrap().flipv();
        let (width, height) = data.dimensions();
        Self {
            width,
            height,
            path: p,
            data,
        }
    }

    pub fn print_debug(&self) {
        println!(
            "File description:\npath: {:?}\nsize: {:?}",
            self.path,
            (self.width, self.height)
        );
    }

    pub fn get_raw_image(&self) -> Vec<u8> {
        self.data.clone().into_rgba8().into_raw()
    }
}
