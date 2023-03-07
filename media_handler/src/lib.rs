pub mod media_config;
pub mod media_handler;

use image::DynamicImage;
use image::GenericImageView;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub path: PathBuf,
    pub data: DynamicImage,
}

impl Frame {
    pub fn new(p: PathBuf) -> Self {
        let data = image::open(&p)
            .expect(&format!(
                "Image couldn't be open by 'image' package: {:?}",
                p
            ))
            .flipv();
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
        let timer = Instant::now();
        let f = self.data.clone().into_rgba8().into_raw();
        println!(
            "LOG: Convert image to raw time: {:?}",
            timer.elapsed().as_millis()
        );
        f
    }
}
