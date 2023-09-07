use image::DynamicImage;
use image::GenericImageView;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    // width:height ratio
    pub ratio: f32,
    pub path: PathBuf,
    pub data: DynamicImage,
}

impl std::fmt::Display for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File description:\npath: {}\nsize: {}:{} | {:p}",
            self.path.display(),
            self.width,
            self.height,
            self
        )
    }
}

impl Frame {
    pub fn new(p: PathBuf) -> Self {
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

    pub fn get_raw_image(&self) -> Vec<u8> {
        self.data.clone().into_rgba8().into_raw()
    }
}
