#[derive(Debug)]
pub struct GraphicConfig {
	pub height: i32,
	pub width: i32,
	pub app_name: String,
	pub window_name: String,
}

impl GraphicConfig {
	pub fn new(height: i32, width: i32, app_name: &str, window_name: &str) -> Self {
		// coulf use an arg call "window_config" with foating or fullscreen to set the window size
		Self { height, width, app_name: String::from(app_name), window_name: String::from(window_name) }
	}
}