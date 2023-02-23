use clap::Parser;
use media_handler::Frame;
use media_handler::media_config::MediaConfig;
use media_handler::media_handler::MediaHandler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value_t = 30)]
    fps: u8,
    data_folder: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let cfg = MediaConfig::new(cli.fps, cli.data_folder);

    let mut media_handler = MediaHandler::new(cfg);

    let img = Frame::new(media_handler.get_next_media());
    img.print_debug();
}
