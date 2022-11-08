mod blob;
use blob::{Blob};

use nannou::prelude::*;
use nannou::image::{RgbImage, DynamicImage, Rgb};
use nannou::color::RgbHue;


const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const BLOBS: usize = 15;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    blobs: [Blob; BLOBS],
}

impl Model {
    fn update(&mut self) {
        for i in 0..self.blobs.len() {
            self.blobs[i].update();
        }
    }

    fn draw(&self, context: &Draw) {
        for i in 0..self.blobs.len() {
            self.blobs[i].draw(context);
        }
    }

    fn get_distance_sum(&self, point: Vec2) -> u8 {
        let mut d = 0.;

        for i in 0..self.blobs.len() {
            let b = point.distance(self.blobs[i].pos);
            let b = 60. * self.blobs[i].r / b;
            d += b;
        }

        clamp(d, 0., 255.) as u8
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let mut blobs = Vec::new();
    let hw = (WIDTH / 2) as f32;
    let hh = (HEIGHT / 2) as f32;
    for i in 0..BLOBS {
        blobs.push(Blob::new(random_range(-hw, hw), random_range(-hh,hh), WIDTH, HEIGHT));
    }

    Model {
        blobs: blobs.try_into().unwrap(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let mut img = RgbImage::new(WIDTH, HEIGHT);
    draw_pixels(&mut img, model);

    let img = DynamicImage::ImageRgb8(img);
    let texture = wgpu::Texture::from_image(app, &img);
    draw.texture(&texture);

    //model.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_pixels(img: &mut RgbImage, model: &Model) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let point = screen_point(x, y);
            let d = model.get_distance_sum(point);

            let color = Rgb([d, d, d]);
            img.put_pixel(x, y, color);
        }
    }
}

fn screen_point(x: u32, y: u32) -> Vec2 {
    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    Vec2::new(-w / 2. + x as f32, h / 2. - y as f32)
}

