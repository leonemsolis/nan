use nannou::prelude::*;
use nannou::image::{RgbImage, DynamicImage, Rgb};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model{}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let mut img = RgbImage::new(WIDTH, HEIGHT);

    let center = Vec2::new((WIDTH / 2) as f32, (HEIGHT / 2) as f32);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let point = Vec2::new(x as f32, y as f32);
            let d = center.distance(point);
            let d = clamp(d, 0.0, 255.0) as u8;

            let color = Rgb([d, d, d]);
            img.put_pixel(x, y, color);
        }
    }


    let img = DynamicImage::ImageRgb8(img);
    let texture = wgpu::Texture::from_image(app, &img);
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}
