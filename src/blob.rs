use nannou::prelude::*;

#[derive(Debug)]
pub struct Blob {
    pub pos: Vec2,
    pub r: f32,
    hw: f32,
    hh: f32,
    vel: Vec2,
}

impl Blob {
    pub fn new(x: f32, y: f32, w: u32, h: u32) -> Blob {
        Blob {
            pos: Vec2::new(x, y),
            r: 40.,
            vel: vec2(random_range(1.,7.), random_range(1., 7.)),
            hw: w as f32 / 2.,
            hh: h as f32 / 2.,
        }
    }

    pub fn update(&mut self) {
        if self.pos.x > self.hw || self.pos.x < -self.hw {
            self.vel.x *= -1.0;
        }

        if self.pos.y > self.hh || self.pos.y < -self.hh {
            self.vel.y *= -1.0;
        }

        self.pos += self.vel;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.pos)
            .radius(self.r as f32)
            .no_fill()
            .stroke_weight(4.)
            .stroke(BLACK);
    }
}
