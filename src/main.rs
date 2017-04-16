// File Visualizer
// 2017 Brad Erickson
// https://github.com/eosrei/file-vis

extern crate png;
extern crate clap;

use std::process::exit;
use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::io::BufWriter;

use clap::{Arg, App};

fn main() {
    // Setup stderr
    let mut stderr = std::io::stderr();

    // Parse cli args
    let matches = App::new("File Visualizer")
        .version("1.0")
        .author("Brad Erickson https://github.com/eosrei/file-vis")
        .about("Create PNG images from binary files")
        .arg(Arg::with_name("INPUT")
                 .help("Sets the input file")
                 .required(true)
                 .index(1))
        .get_matches();

    // Read the input file
    let input_file = matches.value_of("INPUT").unwrap();
    let path = Path::new(input_file);
    if !path.exists() {
        writeln!(&mut stderr, "File not found: {}", input_file).unwrap();
        exit(1);
    }

    // Find height/width of square ratio image
    // Pad file byte array to fit image
    // Find RGB color for each byte

    // To use encoder.set()
    use png::HasParameters;

    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 2, 1);
    encoder
        .set(png::ColorType::Grayscale)
        .set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = [255, 255];
    writer.write_image_data(&data).unwrap();
}
