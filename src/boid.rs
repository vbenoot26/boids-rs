use crate::context::Context;
use rand::RngExt;

pub struct Boid {
    pub x: f32,
    pub y: f32,

    pub speedx: f32,
    pub speedy: f32,
}

pub fn new(ctx: &Context) -> Boid {
    let mut rng = rand::rng();
    Boid {
        x: rng.random::<f32>() * (ctx.width as f32),
        y: rng.random::<f32>() * (ctx.height as f32),
        speedx: rng.random::<f32>() * 10.0 - 5.0,
        speedy: rng.random::<f32>() * 10.0 - 5.0,
    }
}

impl Boid {
    pub fn get_distance(&self, other: &Boid) -> f32 {
        return (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
            as f32).sqrt();
    }
}
