extern crate sdl2;
use sdl2::{pixels::Color, video};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use image::{Rgba, ImageBuffer};
use sdl2::Sdl;

pub struct View {
    pub canvas: Canvas<Window>,
    pub context: Sdl
}

impl View {
    fn new(ctx: Sdl, canv: Canvas<Window>) -> View {
        View {
            canvas: canv,
            context: ctx
        }
    }

    pub fn draw_image(self: &mut View, image: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let mut points: Vec<Point> = vec![];
        for x in 0..image.width() {
            for y in 0..image.height() {
                if image.get_pixel(x, y)[0] != 0 {
                    let p = Point::new(x as i32, y as i32);
                    points.push(p);
                }
            }
        }
        self.canvas.draw_points(points.as_slice()).unwrap();
        self.canvas.present();
    }
}

pub fn draw_image(view: &mut View, image: ImageBuffer<Rgba<u8>, Vec<u8>>) {

}

pub fn create() -> View {

    let mut view: View;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let view = View::new(sdl_context, canvas);
    return view;
}