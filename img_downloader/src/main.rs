use std::path::PathBuf;
use image;
use image::DynamicImage;
use image::GenericImageView;

struct Image {
    path: PathBuf,
    data: DynamicImage
}

impl Image {
    fn new(path: &str) -> Image {
        let p = PathBuf::from(path);
        match p.try_exists() {
            Ok(_) => {
                let data = image::open(&p).unwrap();
                Image {path: p, data: data }
            }
            Err(_) => panic!("File: {:?} doens't exist", p.as_path())
        }
        
    }
}

fn main() {
    let img = Image::new("img.jpg");
    println!("File description:\npath: {:?}\nsize: {:?}", img.path, img.data.dimensions());
}
