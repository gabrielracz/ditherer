#![allow(dead_code, unused_variables)]
mod dither;
use dither::ordered_dither;

mod view;

use image::{Rgba, ImageBuffer, imageops, SubImage, GenericImageView};
use std::iter::Filter;
use std::{env, process::exit};
use std::path::Path;
use sdl2::event::{Event};

use sdl2::keyboard::Keycode;
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut bayer_level: i32 = 2;
    let mut darkness: f32 = 0.0;
    let input_path_string: &str;
    let output_path_string: &str;
    let usage_string = "usage: dither [input] [output] [detail level] [darkness]";

    let mut view = view::create();
    
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
    let (w, h) = original.dimensions();
    


    let resampled = &original;
    let dithered_image = ordered_dither(&resampled, bayer_level, darkness);
    let output = imageops::resize(&dithered_image, original.width(), original.height(), imageops::FilterType::Nearest);
    // let dithered_filename: String = file_stem.to_str().unwrap().to_owned() + "-dithered." + extension.to_str().unwrap();
    output.save(output_path_string).expect("Failed to save image");

    let zf = 8;
    let cropped = GenericImageView::view(&original, w/zf, h/zf, w*(zf-2)/zf, h*(zf-2)/zf).to_image();
    let cropped_resize = imageops::resize(&cropped, w + w/zf, h + h/zf, imageops::FilterType::Nearest);
    ordered_dither(&cropped_resize, bayer_level, darkness).save("cropped.png").expect("fail crop");


    let mut event_pump = view.context.event_pump().unwrap();
    let mut z = 16;
    let mut t = 0;
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { timestamp , window_id, keycode, scancode, keymod, repeat } => {
                    let key = keycode.unwrap();
                    println!("{:?}", keycode);
                    if key == Keycode::J {
                        z -= 2;
                    } else if key == Keycode::K {
                        z += 2;
                    }
                },
                Event::Quit { timestamp }=> {
                    exit(0);
                }
                _ => {}
            }
        }
        
        t += 1;
        z = ((t as f32/60.0).sin() * 150.0) as u32;
        let cropped = GenericImageView::view(&original, z, z, w - z , h - z).to_image();
        let cropped_resize = imageops::resize(&cropped, original.width(), original.height(), imageops::FilterType::Nearest);
        view.draw_image(&ordered_dither(&cropped_resize, bayer_level, darkness));
       
        // std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("\n\x1b[92msaved dithered image to: {} \x1b[0m", output_path_string);
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
