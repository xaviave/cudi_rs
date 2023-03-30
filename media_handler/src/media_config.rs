use std::fs;
use std::path::PathBuf;

use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct MediaConfig {
    pub data_folder: PathBuf,
    pub max_threads: u32,
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
            Err(_) => {
                println!(
                    "Data folder path: {:?} doesn't exist, using default 'data' folder",
                    path
                );
                Self::create_default_folder()
            }
        }
    }

    pub fn new(config_file_path: &str) -> Self {
        /*
        Could add an option to get all args from a config file or environ
        */
        let raw_cfg =
            fs::read_to_string(config_file_path).expect("Unable to read media config file");
        let cfg = &YamlLoader::load_from_str(&raw_cfg).unwrap()[0];
        Self {
            data_folder: Self::folder_exist(cfg["data_folder"].as_str().unwrap()),
            max_threads: cfg["max_threads"].as_i64().unwrap() as u32,
        }
    }
}
