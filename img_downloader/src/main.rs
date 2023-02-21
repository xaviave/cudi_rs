mod media_config;
mod media_handler;

use crate::media_config::MediaConfig;
use crate::media_handler::MediaHandler;
use clap::Parser;
use image;
use image::DynamicImage;
use image::GenericImageView;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 30)]
    fps: u8,
    data_folder: Option<String>,
}

struct Image {
    path: PathBuf,
    data: DynamicImage,
}

impl Image {
    fn new(p: PathBuf) -> Image {
        let data = image::open(&p).unwrap();
        Image { path: p, data }
    }
}

fn main() {
    let cli = Cli::parse();
    let cfg = MediaConfig::new(cli.fps, cli.data_folder);

    let mut media_handler = MediaHandler::new(cfg);

    let img = Image::new(media_handler.get_next_media());
    println!(
        "File description:\npath: {:?}\nsize: {:?}",
        img.path,
        img.data.dimensions()
    );
}
