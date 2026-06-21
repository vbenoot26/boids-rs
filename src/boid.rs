use std::time::Duration;

use crate::context::Context;
use rand::RngExt;

#[derive(Copy, Clone)]
pub struct Boid {
    pub x: f32,
    pub y: f32,

    pub speedx: f32,
    pub speedy: f32,
}

pub fn new<R: RngExt>(ctx: &Context, rng: &mut R) -> Boid {
    Boid {
        x: rng.random::<f32>() * (ctx.width as f32),
        y: rng.random::<f32>() * (ctx.height as f32),
        speedx: rng.random::<f32>() * 10.0 - 5.0,
        speedy: rng.random::<f32>() * 10.0 - 5.0,
    }
}

fn scale_to_duration(speedx: f32, speedy: f32, delta_t: Duration) -> (f32, f32) {
    let second = Duration::from_millis(17);
    let ratio = delta_t.as_nanos() as f32 / second.as_nanos() as f32;

    return (speedx * ratio, speedy * ratio);
}

impl Boid {
    pub fn get_distance_sqrd(&self, other: &Boid) -> f32 {
        return ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
            as f32;
    }

    pub fn step(&mut self, new_speed_x: f32, new_speed_y: f32, delta_t: Duration) {
        self.speedx = new_speed_x;
        self.speedy = new_speed_y;

        let (scaledx, scaledy) = scale_to_duration(self.speedx, self.speedy, delta_t);

        self.x += scaledx;
        self.y += scaledy;
    }
}
