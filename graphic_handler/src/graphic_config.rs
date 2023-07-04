use media_handler::frame::Frame;
use obj::{load_obj, raw, Obj};
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub struct GraphicConfig {
    // u128 to work with Instant millis
    pub fps: u128,
    pub width: u32,
    pub height: u32,
    pub app_name: String,
    pub renderer_size: u8,
    pub loading_media: Frame,
    pub vertex_path: PathBuf,
    pub fragment_path: PathBuf,
    pub fbo_vertex_path: PathBuf,
    pub fbo_fragment_path: PathBuf,
    // pub scenes: Vec<Obj>,
    pub scenes: Vec<String>,
}

impl GraphicConfig {
    fn file_exist(p: &str) -> PathBuf {
        let path = PathBuf::from(p);
        match path.try_exists() {
            Ok(_) => path,
            Err(_) => panic!("File doesn't exist. Check file path or use default."),
        }
    }

    pub fn new(config_file_path: &str) -> Self {
        /*
            could use an arg call "window_config" with floating or fullscreen
            to set the window size
        */
        let cfg = &YamlLoader::load_from_str(
            &fs::read_to_string(config_file_path).expect("Unable to read graphic config file"),
        )
        .unwrap()[0];

        Self {
            fps: (1000 / cfg["fps"].as_i64().unwrap()) as u128,
            width: cfg["width"].as_i64().unwrap() as u32,
            height: cfg["height"].as_i64().unwrap() as u32,
            app_name: String::from(cfg["window_name"].as_str().unwrap()),
            loading_media: Frame::new(Self::file_exist(cfg["loading_media"].as_str().unwrap())),
            vertex_path: Self::file_exist(cfg["engine_shader"][0].as_str().unwrap()),
            fragment_path: Self::file_exist(cfg["engine_shader"][1].as_str().unwrap()),
            fbo_vertex_path: Self::file_exist(cfg["framebuffer_shader"][0].as_str().unwrap()),
            fbo_fragment_path: Self::file_exist(cfg["framebuffer_shader"][1].as_str().unwrap()),
            renderer_size: cfg["renderer_size"].as_i64().unwrap() as u8,
            // will panic if the file is not founded or the Obj file is bad.
            // scenes: cfg["scenes"]
            //     .as_vec()
            //     .unwrap()
            //     .iter()
            //     .map(|f| {
            //         load_obj(BufReader::new(File::open(f.as_str().unwrap()).unwrap())).unwrap()
            //     })
            //     .collect(),
            scenes: cfg["scenes"]
                .as_vec()
                .unwrap()
                .iter()
                .map(|f| String::from(f.as_str().unwrap()))
                .collect(),
        }
    }
}
