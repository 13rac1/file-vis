// File Visualizer
// 2017 Brad Erickson
// Warning: Here be my first Rust program.

use std::process::exit;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;

extern crate clap;
use clap::{Arg, App};

extern crate palette;
use palette::{Rgb, RgbHue, Hsv, FromColor};

extern crate image;

fn main() {
    // Setup stderr
    let mut stderr = std::io::stderr();

    // Parse cli args
    let matches = App::new("File Visualizer")
        .version("1.0")
        .author("Brad Erickson https://github.com/eosrei/file-vis")
        .about("Create PNG images visualizing binary files")
        .arg(Arg::with_name("INPUT")
                 .help("Sets the input file")
                 .required(true)
                 .index(1))
        .arg(Arg::with_name("output")
                 .short("o")
                 .long("output")
                 .value_name("FILE")
                 .help("Sets the output PNG file path")
                 .takes_value(true))
        .get_matches();

    // Check the input file
    let input_arg = matches.value_of("INPUT").unwrap();
    let input_path = Path::new(input_arg);
    let input_display = input_path.display();
    println!("input: {}", input_display);

    if !input_path.exists() {
        writeln!(stderr, "file not found: {}", input_display).unwrap();
        exit(1);
    }
    let mut file = match File::open(&input_path) {
        Err(why) => panic!("couldn't open {}: {}", input_display, why.description()),
        Ok(file) => file,
    };

    // Check the output file
    let default_output = format!("{}.png", input_arg);
    let output_arg = matches.value_of("output").unwrap_or(&default_output);
    let output_path = Path::new(output_arg);
    let output_display = output_path.display();

    if output_path.exists() {
        println!("overwriting: {}", output_display);
    } else {
        println!("output: {}", output_display);
    }

    // Read the input file
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Find height/width of square ratio image
    let byte_count = buffer.len() as f64;
    println!("byte count: {}", byte_count);
    let height_width = byte_count.sqrt().ceil() as u32;
    println!("image size: {}x{}", height_width, height_width);

    // Pad file byte array with black to fit image
    let missing_count = height_width.pow(2) - byte_count as u32;
    println!("padding bytes: {}", missing_count);
    let mut bytes_missing = vec![0; missing_count as usize];

    buffer.append(&mut bytes_missing);
    println!("padded byte count: {}", buffer.len());

    // Procedural, doesn't require Rgb Iterators
    // let mut out: Vec<u8> = Vec::new();
    // for i in &buffer {
    //     let color = Rgb::new(*i);
    //     out.push(color.red);
    //     out.push(color.green);
    //     out.push(color.blue);
    // }

    // Do it all Functional... http://stackoverflow.com/a/30220832
    // Find RGB color for each byte and flatten to Vec<u8>
    let out = buffer
        .iter()
        .map(|&x| Vrgb::new_byte(x))
        .flat_map(|x| x)
        .collect::<Vec<u8>>();

    // Output a PNG
    image::save_buffer(output_path, &out, height_width, height_width, image::RGB(8)).unwrap();
}

#[derive(Debug)]
struct Vrgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Vrgb {
    fn new(red: u8, green: u8, blue: u8) -> Vrgb {
        Vrgb {
            red: red,
            green: green,
            blue: blue,
        }
    }

    fn new_byte(byte: u8) -> Vrgb {
        let colors = match byte {
            1...32 => Vrgb::get_control(byte - 1), // Control Characters
            33...126 => Vrgb::get_printable(byte - 33), // Printable Characters
            127 => Vrgb::get_control(32), // Delete Character
            128...255 => Vrgb::get_extended(byte - 128), // Extended Characters
            _ => (0, 0, 0), // Null
        };

        Vrgb::new(colors.0, colors.1, colors.2)
    }

    fn get_colors(hsv: Hsv<f64>) -> (u8, u8, u8) {
        // HSV Colors: 0 = red, 120 = green, 240 = blue.
        let rgb = Rgb::from_hsv(hsv);
        ((rgb.red * 255f64) as u8, (rgb.green * 255f64) as u8, (rgb.blue * 255f64) as u8)
    }

    fn get_control(value: u8) -> (u8, u8, u8) {
        // Expects 0..32, converts to a 32 range.
        let range = value as f64;
        let hue = 180.0 + range - 16.0;
        let hsv = Hsv::new(RgbHue::from(hue), 1.0, 0.5);
        Vrgb::get_colors(hsv)
    }

    fn get_printable(value: u8) -> (u8, u8, u8) {
        // Expects 0..93, converts to a 32 range.
        let range = value as f64 / 93.0 * 32.0;
        let hue = 240.0 + range - 16.0;
        let hsv = Hsv::new(RgbHue::from(hue), 1.0, 0.5);
        Vrgb::get_colors(hsv)
    }

    fn get_extended(value: u8) -> (u8, u8, u8) {
        // Expects 0..127, converts to a 32 range.
        let range = value as f64 / 127.0 * 32.0;
        let hue = 300.0 + range - 16.0;
        let hsv = Hsv::new(RgbHue::from(hue), 1.0, 0.5);
        Vrgb::get_colors(hsv)
    }
}

impl IntoIterator for Vrgb {
    type Item = u8;
    type IntoIter = VrgbIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        VrgbIntoIterator {
            rgb: self,
            index: 0,
        }
    }
}

struct VrgbIntoIterator {
    rgb: Vrgb,
    index: u8,
}

impl Iterator for VrgbIntoIterator {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let result = match self.index {
            0 => Some(self.rgb.red),
            1 => Some(self.rgb.green),
            2 => Some(self.rgb.blue),
            _ => return None,
        };
        self.index += 1;
        result
    }
}
