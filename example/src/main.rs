use lemon_png::png::Png;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./../test.png");
    let png = Png::load_from_path(&path).unwrap();
    println!("{:?}", png);
}
