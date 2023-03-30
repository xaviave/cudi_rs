use graphic_handler::graphic_config::GraphicConfig;
use graphic_handler::GraphicContext;
use media_handler::frame::Frame;
use media_handler::media_config::MediaConfig;
use media_handler::MediaHandler;

use std::sync::mpsc;

fn main() {
    // // media to graphic communication
    // let (tx_mg, rx_mg) = mpsc::channel::<Frame>();
    // // graphic to media communication
    // let (tx_gm, rx_gm) = mpsc::channel::<u8>();

    let media_config = MediaConfig::new("confs/media.yaml");
    let media_handler = MediaHandler::new(media_config);

    let graphic_config = GraphicConfig::new("confs/graphic.yaml");
    let g = GraphicContext::new(graphic_config);
    g.launch_graphic(media_handler);
}
