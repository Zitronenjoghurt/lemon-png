use lemon_png::decode::png::{PngDecoder, PngDecoderConfig};
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./../test.png");
    let bytes = std::fs::read(&path).unwrap();

    let decode_config = PngDecoderConfig::new().skip_erroneous_chunks();
    let png = PngDecoder::new(&bytes, &decode_config).decode().unwrap();
    println!("{png:#?}");
}
