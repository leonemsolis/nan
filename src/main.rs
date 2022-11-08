use nannou::prelude::*;
use nannou::image::{RgbImage, DynamicImage, Rgb};

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 400;

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
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model{}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let mut img = RgbImage::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }


    let img = DynamicImage::ImageRgb8(img);
    let texture = wgpu::Texture::from_image(app, &img);
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}
