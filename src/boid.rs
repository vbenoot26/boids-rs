use crate::context::Context;
use rand::RngExt;

pub struct Boid {
    pub x: f32,
    pub y: f32,

    pub speedx: f32,
    pub speedy: f32,
}

#[derive(Default, Clone)]
pub struct Forces {
    pub sepx: f32,
    pub sepy: f32,

    pub xspeed_avg: f32,
    pub yspeed_avg: f32,

    pub xpos_avg: f32,
    pub ypos_avg: f32,

    pub neighbour_amount: usize,
}

impl Default for Boid {
    fn default() -> Boid {
        Boid {
            x: 20.0,
            y: 20.0,
            speedx: 10.0,
            speedy: 10.0,
        }
    }
}

pub fn new(ctx: &Context) -> Boid {
    let mut rng = rand::rng();
    Boid {
        x: rng.random::<f32>() * (ctx.width as f32),
        y: rng.random::<f32>() * (ctx.height as f32),
        speedx: rng.random::<f32>() * 10.0,
        speedy: rng.random::<f32>() * 10.0,
    }
}

impl Boid {
    pub fn step(&mut self, ctx: &Context, forces: &Forces) {
        let (speedx, speedy) = self.calc_speeds(ctx, forces);
        self.speedx = speedx;
        self.speedy = speedy;

        self.x += self.speedx;
        self.y += self.speedy;
    }

    fn calc_speeds(&self, ctx: &Context, forces: &Forces) -> (f32, f32) {
        let mut new_speedx = self.speedx + forces.sepx * ctx.avoid_factor;
        let mut new_speedy = self.speedy + forces.sepy * ctx.avoid_factor;

        if forces.neighbour_amount == 0 {
            return (new_speedx, new_speedy);
        }

        new_speedx += (forces.xpos_avg - self.x) * ctx.centering_factor;
        new_speedy += (forces.ypos_avg - self.y) * ctx.centering_factor;

        new_speedx += (forces.xspeed_avg - self.speedx) * ctx.matching_factor;
        new_speedy += (forces.yspeed_avg - self.speedy) * ctx.matching_factor;

        (new_speedx, new_speedy)
    }

    pub fn get_distance(&self, other: &Boid) -> f32 {
        return (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
            as f32).sqrt();
    }

    pub fn calc_separation(&self, neighbours: &[&Boid]) -> (f32, f32) {
        (
            neighbours.iter().map(|b| self.x - b.x).sum(),
            neighbours.iter().map(|b| self.y - b.y).sum(),
        )
    }

    pub fn calc_alignment(&self, neighbours: &[&Boid]) -> (f32, f32) {
        if neighbours.len() == 0 {
            return (0.0, 0.0);
        }

        (
            neighbours.iter().map(|b| b.speedx).sum::<f32>() / (neighbours.len() as f32),
            neighbours.iter().map(|b| b.speedy).sum::<f32>() / (neighbours.len() as f32),
        )
    }

    pub fn calc_cohesion(&self, neighbours: &[&Boid]) -> (f32, f32) {
        if neighbours.len() == 0 {
            return (0.0, 0.0);
        }

        (
            neighbours.iter().map(|b| b.x).sum::<f32>() / (neighbours.len() as f32),
            neighbours.iter().map(|b| b.y).sum::<f32>() / (neighbours.len() as f32),
        )
    }
}
