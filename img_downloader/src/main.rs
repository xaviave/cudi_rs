mod media_config;
mod media_handler;

use crate::media_config::MediaConfig;
use crate::media_handler::MediaHandler;
use clap::Parser;
use image;
use image::DynamicImage;
use image::GenericImageView;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 30)]
    fps: u8,
    data_folder: Option<String>,
}

fn create_folder() -> PathBuf {
    let default_path = PathBuf::from("data");
    match default_path.try_exists() {
        Ok(true) => default_path,
        Ok(false) => {
            fs::create_dir(&default_path).ok();
            println!("Path {:?} will be use as data folder.", &default_path);
            default_path.clone()
        }
        Err(_) => {
            panic!("OS error, check permission");
        }
    }
}

fn folder_exist(p: &str) -> PathBuf {
    let path = PathBuf::from(p);
    match path.try_exists() {
        Ok(_) => path,
        Err(_) => create_folder(),
    }
}

fn cli_to_config(cli: &Cli) -> MediaConfig {
    let path = match &cli.data_folder {
        Some(p) => folder_exist(&p),
        None => create_folder(),
    };
    MediaConfig::new(cli.fps, path)
}

struct Image {
    path: PathBuf,
    data: DynamicImage,
}

impl Image {
    fn new(p: PathBuf) -> Image {
        match p.try_exists() {
            Ok(_) => {
                let data = image::open(&p).unwrap();
                Image { path: p, data }
            }
            Err(_) => panic!("File: {:?} doesn't exist", p.as_path()),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let media_handler = MediaHandler::new(cli_to_config(&cli));
    println!(
        "fps: {:?} | folder path {:?}",
        media_handler.config.fps, media_handler.config.data_folder
    );

    let img = Image::new(media_handler.config.data_folder.join("img.jpg"));
    println!(
        "File description:\npath: {:?}\nsize: {:?}",
        img.path,
        img.data.dimensions()
    );
}
