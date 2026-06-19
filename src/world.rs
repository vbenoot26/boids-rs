use std::slice::Chunks;
use std::thread::{available_parallelism, scope};

use crate::boid::{self, Boid};
use crate::grid::Grid;
use crate::{context, grid};

#[derive(Copy, Clone)]
pub struct BoidId(pub usize);

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

        let threads = match available_parallelism() {
            Ok(thr) => thr.get(),
            Err(_) => 1,
        };

        let ch_size = (self.boids.len() as f32 / threads as f32).ceil() as usize;

        let mut speeds = vec![(0.0, 0.0); self.boids.len()];

        scope(|s| {
            for (b_ch, speeds_ch) in self.boids.chunks(ch_size).zip(speeds.chunks_mut(ch_size)) {
                s.spawn(|| {
                    self.calc_speeds(b_ch, speeds_ch);
                });
            }
        });

        for (i, b) in self.boids.iter_mut().enumerate() {
            let (speedx, speedy) = speeds[i];
            b.step(speedx, speedy);

            b.x = b.x.rem_euclid(self.width as f32);
            b.y = b.y.rem_euclid(self.height as f32);
        }
    }

    fn calc_speeds(&self, b_ch: &[Boid], speeds_ch: &mut [(f32, f32)]) {
        for (i, b) in b_ch.iter().enumerate() {
            let (new_speed_x, new_speed_y) = self.calc_new_speeds(b);
            speeds_ch[i] = self.clamp_speeds(b.speedx + new_speed_x, b.speedy + new_speed_y);
        }
    }

    fn calc_new_speeds(&self, boid: &boid::Boid) -> (f32, f32) {
        let sep_dist = self.ctx.close_distance;
        let view_dist = self.ctx.viewing_distance;

        let mut sepx = 0.0;
        let mut sepy = 0.0;

        let mut xspeed_cum = 0.0;
        let mut yspeed_cum = 0.0;

        let mut xpos_cum = 0.0;
        let mut ypos_cum = 0.0;

        let mut neighbour_count = 0_usize;

        for b in self
            .grid
            .get_possible_neighbours(&boid)
            .map(|id| &self.boids[id.0])
        {
            let bdist = b.get_distance_sqrd(boid);

            if bdist <= 0.0 {
                continue;
            }

            if bdist >= view_dist * view_dist {
                continue;
            }

            xspeed_cum += b.speedx;
            yspeed_cum += b.speedy;
            xpos_cum += b.x;
            ypos_cum += b.y;

            neighbour_count += 1;
            if bdist < sep_dist * sep_dist {
                sepx += boid.x - b.x;
                sepy += boid.y - b.y;
            }
        }

        if neighbour_count == 0 {
            return (0.0, 0.0);
        }

        let xpos_avg = xpos_cum / neighbour_count as f32;
        let ypos_avg = ypos_cum / neighbour_count as f32;

        let xspeed_avg = xspeed_cum / neighbour_count as f32;
        let yspeed_avg = yspeed_cum / neighbour_count as f32;

        let mut new_speedx = sepx * self.ctx.avoid_factor;
        let mut new_speedy = sepy * self.ctx.avoid_factor;

        new_speedx += (xpos_avg - boid.x) * self.ctx.centering_factor;
        new_speedy += (ypos_avg - boid.y) * self.ctx.centering_factor;

        new_speedx += (xspeed_avg - boid.speedx) * self.ctx.matching_factor;
        new_speedy += (yspeed_avg - boid.speedy) * self.ctx.matching_factor;

        (new_speedx, new_speedy)
    }

    fn clamp_speeds(&self, speedx: f32, speedy: f32) -> (f32, f32) {
        let mut new_speedx = speedx;
        let mut new_speedy = speedy;
        let sqrd_speed = (speedx * speedx) + (speedy * speedy);

        if sqrd_speed < self.ctx.min_speed * self.ctx.min_speed {
            let factor = ((self.ctx.min_speed * self.ctx.min_speed) / sqrd_speed).sqrt();

            new_speedx = factor * speedx;
            new_speedy = factor * speedy;
        }

        if sqrd_speed > self.ctx.max_speed * self.ctx.max_speed {
            let factor = ((self.ctx.max_speed * self.ctx.max_speed) / sqrd_speed).sqrt();

            new_speedx = factor * speedx;
            new_speedy = factor * speedy;
        }

        (new_speedx, new_speedy)
    }
}
