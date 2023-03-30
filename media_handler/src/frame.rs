use image::DynamicImage;
use image::GenericImageView;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    // height:width ratio
    pub ratio: f32,
    pub path: PathBuf,
    pub data: DynamicImage,
}

impl Frame {
    pub fn new(p: PathBuf) -> Self {
        // add a image header checker to handle bad image format
        // https://docs.rs/image/0.24.6/image/io/struct.Reader.html
        let data = image::open(&p).expect(&format!(
            "Image couldn't be open by 'image' package: {:?}",
            p
        ));
        let (width, height) = data.dimensions();
        Self {
            width,
            height,
            ratio: width as f32 / height as f32,
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
