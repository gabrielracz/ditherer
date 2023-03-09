use image::{Rgba, ImageBuffer};
use std::{env, process::exit};
use std::path::Path;
use std::ffi::OsStr;

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


fn ordered_dither(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, level: i32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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
            let threshhold = 1.03f32 - kernel[x as usize%kernel.len()][y as usize %kernel.len()];

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
    let path_string: &str;
    let usage_string = "usage: dither [image] [detail level] [darkness]";
    
    if args.len() > 1{
        path_string = &args[1];
    } else {
        println!("{}", usage_string);
        exit(0);
    }
    if args.len() > 2 {
        bayer_level = args[2].parse::<i32>().expect(usage_string);
    }

    let path = Path::new(path_string);
    let file_stem = path.file_stem().unwrap();
    let extension = path.extension().unwrap();

    let source_image = image::open(path_string).expect("Failed to open input image").into_rgba8();
    let dithered_image = ordered_dither(&source_image, bayer_level);

    let dithered_filename: String = file_stem.to_str().unwrap().to_owned() + "-dithered." + extension.to_str().unwrap();
    dithered_image.save(&dithered_filename).expect("Failed to save image");
    println!("Saved dithered image to: {}", dithered_filename);
}
