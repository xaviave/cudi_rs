use std::fs;
use std::path::PathBuf;

pub struct MediaConfig {
    pub fps: u8,
    pub data_folder: PathBuf,
}

impl MediaConfig {
    fn create_default_folder() -> PathBuf {
        let default_path = match fs::canonicalize(PathBuf::from("data")) {
            Ok(p) => p,
            Err(_) => panic!("Absolute path couldn't be created"),
        };
        match default_path.try_exists() {
            Ok(true) => default_path,
            Ok(false) => {
                fs::create_dir(&default_path).ok();
                println!("Path '{:?}' will be use as data folder.", &default_path);
                default_path.clone()
            }
            Err(_) => {
                panic!("OS error, check permissions.");
            }
        }
    }

    fn folder_exist(p: &str) -> PathBuf {
        let path = PathBuf::from(p);
        match path.try_exists() {
            Ok(_) => path,
            Err(_) => panic!("Folder doesn;t exist. Check folder path or use default."),
        }
    }

    pub fn new(fps: u8, df: Option<String>) -> Self {
        let data_folder = match df {
            Some(p) => Self::folder_exist(&p),
            None => Self::create_default_folder(),
        };
        MediaConfig { fps, data_folder }
    }
}
