use crate::boid::{self, Boid};
use crate::context::Context;
use crate::grid::Grid;
use crate::{context, grid};

#[derive(Copy, Clone)]
pub struct BoidId(pub usize);

#[derive(Default, Clone)]
struct Forces {
    pub sepx: f32,
    pub sepy: f32,

    pub xspeed_avg: f32,
    pub yspeed_avg: f32,

    pub xpos_avg: f32,
    pub ypos_avg: f32,

    pub neighbour_amount: usize,
}

pub struct World {
    pub boids: Vec<boid::Boid>,
    width: i32,
    height: i32,
    grid: Grid,
    ctx: context::Context,
}

pub fn init(ctx: context::Context) -> World {
    let boids: Vec<Boid> = (0..ctx.boid_amount).map(|_| boid::new(&ctx)).collect();

    return World {
        boids: boids,
        width: ctx.width,
        height: ctx.height,
        grid: grid::init(&ctx),
        ctx: ctx,
    };
}

impl World {
    pub fn step(&mut self) {
        self.grid.distribute(&self.boids);

        let forces: Vec<Forces> = self
            .boids
            .iter()
            .map(|b| {
                let (sepx, sepy) = self.get_separation_forces(b, self.ctx.close_distance);
                let mut other_forces = self.get_neighbour_forces(b, self.ctx.viewing_distance);

                other_forces.sepx = sepx;
                other_forces.sepy = sepy;

                other_forces
            })
            .collect();

        for (i, b) in self.boids.iter_mut().enumerate() {
            forces[i].apply_to(&self.ctx, b);

            b.x = b.x.rem_euclid(self.width as f32);
            b.y = b.y.rem_euclid(self.height as f32);
        }
    }

    fn get_neighbour_forces(&self, boid: &boid::Boid, dist: f32) -> Forces {
        let mut xspeed_cum = 0.0;
        let mut yspeed_cum = 0.0;

        let mut xpos_cum = 0.0;
        let mut ypos_cum = 0.0;

        let mut neighbour_count = 0_usize;

        for b in self
            .grid
            .get_possible_neighbours(&boid)
            .map(|id| &self.boids[id.0])
            .filter(|b| {
                let bdist = b.get_distance(boid);
                bdist < dist && bdist > 0.0
            })
        {
            xspeed_cum += b.speedx;
            yspeed_cum += b.speedy;
            xpos_cum += b.x;
            ypos_cum += b.y;

            neighbour_count += 1;
        }

        Forces {
            sepx: 0.0,
            sepy: 0.0,
            xspeed_avg: xspeed_cum / neighbour_count as f32,
            yspeed_avg: yspeed_cum / neighbour_count as f32,
            xpos_avg: xpos_cum / neighbour_count as f32,
            ypos_avg: ypos_cum / neighbour_count as f32,
            neighbour_amount: neighbour_count,
        }
    }

    fn get_separation_forces(&self, boid: &boid::Boid, dist: f32) -> (f32, f32) {
        let (mut sepx, mut sepy) = (0.0, 0.0);

        for b in self
            .grid
            .get_possible_neighbours(boid)
            .map(|id| &self.boids[id.0])
            .filter(|b| {
                let bdist = b.get_distance(boid);
                bdist < dist && bdist > 0.0
            })
        {
            sepx += boid.x - b.x;
            sepy += boid.y - b.y;
        }

        (sepx, sepy)
    }
}

impl Forces {
    fn apply_to(&self, ctx: &Context, boid: &mut Boid) {
        let (speedx, speedy) = self.calc_speeds(ctx, boid);
        boid.speedx = speedx;
        boid.speedy = speedy;

        boid.x += boid.speedx;
        boid.y += boid.speedy;
    }

    fn calc_speeds(&self, ctx: &Context, boid: &Boid) -> (f32, f32) {
        let mut new_speedx = boid.speedx + self.sepx * ctx.avoid_factor;
        let mut new_speedy = boid.speedy + self.sepy * ctx.avoid_factor;

        if self.neighbour_amount == 0 {
            return (new_speedx, new_speedy);
        }

        new_speedx += (self.xpos_avg - boid.x) * ctx.centering_factor;
        new_speedy += (self.ypos_avg - boid.y) * ctx.centering_factor;

        new_speedx += (self.xspeed_avg - boid.speedx) * ctx.matching_factor;
        new_speedy += (self.yspeed_avg - boid.speedy) * ctx.matching_factor;

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
}
