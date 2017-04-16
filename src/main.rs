// File Visualizer
// 2017 Brad Erickson

extern crate png;
extern crate clap;

use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::io::BufWriter;

use clap::{Arg, App};
use png::HasParameters;

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
    let display = path.display();

    if !path.exists() {
        writeln!(&mut stderr, "file not found: {}", display).unwrap();
        exit(1);
    }
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    println!("filename: {}", display);
    let mut buffer = vec![0];
    file.read_to_end(&mut buffer).unwrap();

    // Find height/width of square ratio image
    let byte_count = buffer.len() as f64;
    println!("byte count: {}", byte_count);
    let height_width = byte_count.sqrt().ceil() as u32;
    println!("image size: {}x{}", height_width, height_width);

    // Pad file byte array to fit image
    let missing_count = height_width.pow(2) - byte_count as u32;
    println!("padding bytes: {}", missing_count);
    let mut bytes_missing = vec![0; missing_count as usize];

    buffer.append(&mut bytes_missing);
    println!("final byte count: {}", buffer.len());

    // TODO: Find RGB color for each byte

    let path = Path::new("image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, height_width, height_width);
    encoder
        .set(png::ColorType::Grayscale)
        .set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&buffer).unwrap();
}
