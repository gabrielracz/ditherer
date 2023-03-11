#![allow(dead_code)]

use image::{Rgba, ImageBuffer, imageops};
use std::{env, process::exit};
use std::path::Path;
// use std::ffi::OsStr;

#[non_exhaustive]
struct Colors;
impl Colors {
    pub const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
    pub const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
}

macro_rules! _constfor {
    ($limit:expr, $body:block) => {
        let mut i = 0;
        while i < $limit {
            // execute the code inside the loop
            $body;
            i += 1;
        }
    };
}

struct Bayer;
impl Bayer {
    pub const _L1: &'static[&'static[f32]] = &[
        &[0.0/4.0, 2.0/4.0],
        &[3.0/4.0, 1.0/4.0]
    ];
    pub const _L2: &'static[&'static[f32]] = &[
        &[ 0.0/16.0,  8.0/16.0,  2.0/16.0, 10.0/16.0],
        &[12.0/16.0,  4.0/16.0, 14.0/16.0,  6.0/16.0],
        &[ 3.0/16.0, 11.0/16.0,  1.0/16.0,  9.0/16.0],
        &[15.0/16.0,  7.0/16.0, 13.0/16.0,  5.0/16.0]
    ];
    pub const _L3: &'static[&'static[f32]] = &[
        &[ 0.0/64.0, 32.0/64.0,  8.0/64.0, 40.0/64.0,  2.0/64.0, 34.0/64.0, 10.0/64.0, 42.0/64.0],
        &[48.0/64.0, 16.0/64.0, 56.0/64.0, 24.0/64.0, 50.0/64.0, 18.0/64.0, 58.0/64.0, 26.0/64.0],
        &[12.0/64.0, 44.0/64.0,  4.0/64.0, 36.0/64.0, 14.0/64.0, 46.0/64.0,  6.0/64.0, 38.0/64.0],
        &[60.0/64.0, 28.0/64.0, 52.0/64.0, 20.0/64.0, 62.0/64.0, 30.0/64.0, 54.0/64.0, 22.0/64.0],
        &[ 3.0/64.0, 35.0/64.0, 11.0/64.0, 43.0/64.0,  1.0/64.0, 33.0/64.0,  9.0/64.0, 41.0/64.0],
        &[51.0/64.0, 19.0/64.0, 59.0/64.0, 27.0/64.0, 49.0/64.0, 17.0/64.0, 57.0/64.0, 25.0/64.0],
        &[15.0/64.0, 47.0/64.0,  7.0/64.0, 49.0/64.0, 13.0/64.0, 45.0/64.0,  5.0/64.0, 37.0/64.0],
        &[63.0/64.0, 31.0/64.0, 55.0/64.0, 23.0/64.0, 61.0/64.0, 29.0/64.0, 53.0/64.0, 21.0/64.0]
    ];
}



fn ordered_dither(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, level: i32, darkness: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let kernel: &[&[f32]];
    match level {
        1 => {kernel = Bayer::_L1},
        2 => {kernel = Bayer::_L2},
        3 => {kernel = Bayer::_L3},
        _ => {kernel = Bayer::_L2}
    }

    let mut result: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(image.width(), image.height());
    for x in 0..image.width() {
        for y in 0..image.height() {
            let p = image.get_pixel(x, y);
            let intensity: f32 = (p[0] as u32 + p[1] as u32 + p[2] as u32) as f32/(3.0f32*255.0f32);
            let threshhold = kernel[x as usize%kernel.len()][y as usize %kernel.len()] + darkness;

            let dithered: Rgba<u8>;
            if intensity > threshhold {
                dithered = Colors::WHITE;
            } else {
                dithered = Colors::BLACK;
            }
            result.put_pixel(x, y, dithered);
        }
    }
    return result;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut bayer_level: i32 = 2;
    let mut darkness: f32 = 0.0;
    let input_path_string: &str;
    let output_path_string: &str;
    let usage_string = "usage: dither [input] [output] [detail level] [darkness]";
    
    if args.len() > 2{
        input_path_string = &args[1];
        output_path_string = &args[2];
    } else {
        println!("{}", usage_string);
        exit(0);
    }
    if args.len() > 3 {
        bayer_level = args[3].parse::<i32>().expect("invalid detail level");
    }
    if args.len() > 4 {
        darkness = args[4].parse::<f32>().expect("invalid darkness value");
    }

    let path = Path::new(input_path_string);
    let file_stem = path.file_stem().unwrap();
    let extension = path.extension().unwrap();

    let original = image::open(input_path_string).expect("Failed to open input image").into_rgba8();
    // let resampled = imageops::resize(&original, original.width()/2, original.height()/2, imageops::FilterType::Nearest);
    let resampled = &original;
    let dithered_image = ordered_dither(&resampled, bayer_level, darkness);
    let output = imageops::resize(&dithered_image, original.width(), original.height(), imageops::FilterType::Nearest);

    let dithered_filename: String = file_stem.to_str().unwrap().to_owned() + "-dithered." + extension.to_str().unwrap();
    output.save(output_path_string).expect("Failed to save image");
    println!("Saved dithered image to: {}", dithered_filename);
}

// https://www.includehelp.com/rust/reverse-bits-of-a-binary-number.aspx
fn bit_reverse(n: u8) -> u8 {
    let mut val: u8 = 0;
    let mut tmp: u8;
    let mut rev: u8 = 0;

    while val < 8 {
        tmp = n & (1 << val);
        if tmp>0
        {
            rev = rev | (1 << ((8 - 1) - val));
        }
        val = val + 1;
    }
    return rev;
}

// https://graphics.stanford.edu/~seander/bithacks.html#InterleaveBMN
fn bit_interleave(i: u32, j: u32) -> u32 {
    const B: [u32; 4] = [0x55555555, 0x33333333, 0x0F0F0F0F, 0x00FF00FF];
    const S: [u32; 4] = [1, 2, 4, 8];

    let mut x: u32 = i;
    let mut y: u32 = j;
    let z: u32;

    x = (x | (x << S[3])) & B[3];
    x = (x | (x << S[2])) & B[2];
    x = (x | (x << S[1])) & B[1];
    x = (x | (x << S[0])) & B[0];

    y = (y | (y << S[3])) & B[3];
    y = (y | (y << S[2])) & B[2];
    y = (y | (y << S[1])) & B[1];
    y = (y | (y << S[0])) & B[0];

    z = x | (y << 1);

    return z;
}
