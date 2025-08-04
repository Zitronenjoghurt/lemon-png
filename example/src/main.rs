use lemon_png::codec::decode::png::{PngDecoder, PngDecoderConfig};
use lemon_png::rendering::PngRenderer;
use minifb::{Scale, Window, WindowOptions};
use std::path::PathBuf;

mod crc_table;

fn main() {
    let path = PathBuf::from("./../test_images/f03n2c08.png");
    let bytes = std::fs::read(&path).unwrap();

    let decode_config = PngDecoderConfig::new().skip_erroneous_chunks();
    let png = PngDecoder::new(&bytes, &decode_config).decode().unwrap();
    println!("{png:#?}");

    let image_buffer = PngRenderer::new().render(&png).unwrap();
    let argb_buffer = image_buffer.get_argb_buffer();
    drop(image_buffer);
    let mut window = Window::new(
        "Test Render",
        png.width as usize,
        png.height as usize,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap();

    while window.is_open() {
        window
            .update_with_buffer(&argb_buffer, png.width as usize, png.height as usize)
            .unwrap();
    }
}
