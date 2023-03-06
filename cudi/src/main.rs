use clap::Parser;
use graphic_handler::graphic_config::GraphicConfig;
use graphic_handler::GraphicContext;
use media_handler::media_config::MediaConfig;
use media_handler::media_handler::MediaHandler;
use media_handler::Frame;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 30)]
    fps: u8,
    data_folder: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let media_config = MediaConfig::new(cli.fps, cli.data_folder);

    let mut media_handler = MediaHandler::new(media_config);

    let img = Frame::new(media_handler.get_next_media());
    img.print_debug();

    let graphic_config = GraphicConfig::new(
        1000,
        1000,
        "CUDI",
        "data/init/loading.jpg",
        "Running panorama",
        "graphic_handler/shaders/default.vs",
        "graphic_handler/shaders/default.fs",
    );
    let g = GraphicContext::new(graphic_config);
    g.launch_graphic();
}
