extern crate image;
extern crate clap;

use std::str::from_utf8;
use std::path::Path;
use clap::{Arg, App};

fn intensity_to_ascii(value: &u8, invert: bool) -> &str {
    // changes an intensity into an ascii character
    // this is a central step in creating the ascii art
    let ascii_chars  = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*",
        "#", "&", "%", "B", "@", "$"
    ];
    
    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {   
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx]
        }
    }

    ascii_chars[ (n_chars - 1) as usize ]
}

fn main() {
    // setup clap argument parser for input file
    let matches = App::new("asciify")
                    .version("0.0.1")
                    .author("edelsonc")
                    .about("Turn an image into ascii art!")
                    .args( &[
                    Arg::from_usage("<INPUT> 'Sets the image file'"),
                    Arg::from_usage("[resize] -s, --resize [width] [height] 'Rescale image; does not preserve aspect ratio'"),
                    ])
                    .get_matches();

    // open image as new dynamic image
    let image_name = matches.value_of("INPUT").unwrap();
    let img = match image::open(&Path::new(image_name)) {
        Ok(p) => p,
        Err(e) => panic!("Not a valid image path or could no open image"),
    };

    // resize image as an option if its very large...defualts to screen width
    let dims = match matches.values_of_lossy("resize") {
        Some(v) => v.iter().map(|s| s.parse::<u32>().unwrap()).collect(),
        None => vec![80u32, 40u32],
    };

    let img = img.resize_exact(dims[0], dims[1], image::FilterType::Nearest);

    // convert to LUMA and change each greyscale pixel into a character
    let mut imgbuf = img.to_luma();
    let mut ascii_art = String::new();
    for pixel in imgbuf.pixels() {
        let pixel_ascii = intensity_to_ascii(&pixel.data[0], false);
        ascii_art = ascii_art + pixel_ascii;
    }

    let subs = ascii_art.as_bytes()
        .chunks(imgbuf.width() as usize)
        .map(from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();
    for s in subs {
        println!("{}", s);
    }
}

