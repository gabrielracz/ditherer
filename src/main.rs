use image::{Rgba, ImageBuffer};


fn main() {
    let src = image::open("jungle-opening.png").unwrap().into_rgba8();
    let mut new: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(src.width(), src.height());

    const KSEED: [[u8; 4]; 4] = [
        [0, 8, 2, 10],
        [12, 4, 14, 6],
        [3, 11, 1, 9],
        [15, 7, 13, 5]
    ];
    // const KSEED: [[u8; 2]; 2] = [
    //     [0, 2],
    //     [3, 1]
    // ];
    const N: usize = KSEED.len();

    let mut kernel = [[0f32; N]; N];
    for x in 0..N {
        for y in 0..N{
            kernel[x][y] = KSEED[x][y] as f32 * (1f32/(N*N) as f32);
        }
    }

    const color: Rgba<u8> = Rgba([255, 255, 255, 255]);
    const black: Rgba<u8> = Rgba([0, 0, 0, 255]);


    for x in 0..src.width() {
        for y in 0..src.height() {
            let p = src.get_pixel(x, y);
            let intensity: f32 = (p[0] as u32 +p[1] as u32 +p[2] as u32) as f32/(3f32*255f32);
            let dithered: Rgba<u8>;
            let threshhold = 1f32 - kernel[x as usize%kernel.len()][y as usize %kernel.len()];
            // println!("{} {}", threshhold, intensity);
            if intensity > threshhold {
                // B8BB26
                dithered = color;
            } else {
                dithered = black;
            }
            // let npix = Rgba([255 - p[0], 255 - p[1], 255 - p[2], 255]);
            new.put_pixel(x, y, dithered);
        }
    }

    new.save("modified.png").expect("Failed to save image");
    
}
