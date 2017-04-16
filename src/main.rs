extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    // To use encoder.set()
    use png::HasParameters;

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 2, 1); // Width is 2 pixels and height is 1.
    encoder
        .set(png::ColorType::RGBA)
        .set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    let data = [255, 0, 0, 255, 0, 0, 0, 255];
    writer.write_image_data(&data).unwrap(); // Save
}
