use media_handler::Frame;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GraphicConfig {
    // u128 to work with Instant millis
    pub fps: u128,
    pub width: u32,
    pub height: u32,
    pub app_name: String,
    pub loading_media: Frame,
    pub vertex_path: PathBuf,
    pub fragment_path: PathBuf,
}

impl GraphicConfig {
    fn file_exist(p: &str) -> PathBuf {
        let path = PathBuf::from(p);
        match path.try_exists() {
            Ok(_) => path,
            Err(_) => panic!("File doesn't exist. Check file path or use default."),
        }
    }

    pub fn new(
        fps: u128,
        width: u32,
        height: u32,
        app_name: &str,
        loading_media_path: &str,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Self {
        /*
            could use an arg call "window_config" with floating or fullscreen
            to set the window size
        */

        Self {
            fps: 1000 / fps,
            width,
            height,
            app_name: String::from(app_name),
            loading_media: Frame::new(Self::file_exist(loading_media_path)),
            vertex_path: Self::file_exist(vertex_path),
            fragment_path: Self::file_exist(fragment_path),
        }
    }
}
