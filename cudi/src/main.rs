use clap::Parser;
use graphic_handler::graphic_config::GraphicConfig;
use graphic_handler::GraphicContext;
use media_handler::media_config::MediaConfig;
use media_handler::media_handler::MediaHandler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    data_folder: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let media_config = MediaConfig::new(cli.data_folder);
    let media_handler = MediaHandler::new(media_config);

    let graphic_config = GraphicConfig::new(
        20,
        250,
        250,
        "CUDI",
        "data/init/loading.jpg",
        "graphic_handler/shaders/cudi.vs",
        "graphic_handler/shaders/cudi.fs",
        "graphic_handler/shaders/framebuffer.vs",
        "graphic_handler/shaders/framebuffer.fs",
    );
    let g = GraphicContext::new(graphic_config);
    g.launch_graphic(media_handler);
}
