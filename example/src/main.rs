use lemon_png::codec::decode::png::{PngDecoder, PngDecoderConfig};
use std::path::PathBuf;

mod crc_table;

fn main() {
    let path = PathBuf::from("./../test_images/basn3p04.png");
    let bytes = std::fs::read(&path).unwrap();

    let decode_config = PngDecoderConfig::new().skip_erroneous_chunks();
    let png = PngDecoder::new(&bytes, &decode_config).decode().unwrap();
    println!("{png:#?}");
}
