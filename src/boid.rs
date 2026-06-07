use crate::context::Context;
use rand::RngExt;

pub struct Boid {
    pub x: f32,
    pub y: f32,

    speedx: f32,
    speedy: f32,
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

        let sqrd_speed = (new_speedx * new_speedx) + (new_speedy * new_speedy);

        if sqrd_speed < ctx.min_speed * ctx.min_speed {
            let factor = ((ctx.min_speed * ctx.min_speed) / sqrd_speed).sqrt();

            new_speedx = factor * new_speedx;
            new_speedy = factor * new_speedy;
        }

        if sqrd_speed > ctx.max_speed * ctx.max_speed {
            let factor = ((ctx.max_speed * ctx.max_speed) / sqrd_speed).sqrt();

            new_speedx = factor * new_speedx;
            new_speedy = factor * new_speedy;
        }

        (new_speedx, new_speedy)
    }

    pub fn get_distance(&self, other: &Boid) -> f32 {
        return (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
            as f32).sqrt();
    }

    pub fn calc_forces(
        &self,
        neighbours: impl Iterator<Item = &Boid>,
        too_close: impl Iterator<Item = &Boid>,
    ) -> Forces {
        let forces_neighbours = neighbours.map(|b| Forces {
            xspeed_avg: b.speedx,
            yspeed_avg: b.speedy,

            xpos_avg: b.x,
            ypos_avg: b.y,

            neighbour_amount: 1,

            sepx: 0.0,
            sepy: 0.0,
        });

        let forces_close = too_close.map(|b| Forces {
            sepx: self.x - b.x,
            sepy: self.y - b.y,

            xspeed_avg: 0.0,
            yspeed_avg: 0.0,
            xpos_avg: 0.0,
            ypos_avg: 0.0,
            neighbour_amount: 0,
        });

        let forces = forces_neighbours.chain(forces_close).fold(
            Forces {
                sepx: 0.0,
                sepy: 0.0,
                xspeed_avg: 0.0,
                yspeed_avg: 0.0,
                xpos_avg: 0.0,
                ypos_avg: 0.0,
                neighbour_amount: 0,
            },
            |cum, force| Forces {
                sepx: cum.sepx + force.sepx,
                sepy: cum.sepy + force.sepy,
                xspeed_avg: cum.xspeed_avg + force.xspeed_avg,
                yspeed_avg: cum.yspeed_avg + force.yspeed_avg,
                xpos_avg: cum.xpos_avg + force.xpos_avg,
                ypos_avg: cum.ypos_avg + force.ypos_avg,
                neighbour_amount: cum.neighbour_amount + force.neighbour_amount,
            },
        );

        forces.xspeed_avg = forces.xspeed_avg / forces.neighbour_amount;
        forces.yspeed_avg = forces.yspeed_avg / forces.neighbour_amount;

        forces.xpos_avg = forces.xpos_avg / forces.neighbour_amount;
        forces.ypos_avg = forces.ypos_avg / forces.neighbour_amount;

        forces
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
