use crate::boid::Forces;
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

        let forces: Vec<Forces> = self
            .boids
            .iter()
            .map(|b| {
                let too_close = self.find_neighbours(b, self.ctx.close_distance);
                let neighbours = self.find_neighbours(b, self.ctx.viewing_distance);

                let (sep_x, sep_y) = b.calc_separation(&too_close[..]);
                let (align_x, align_y) = b.calc_alignment(&neighbours[..]);
                let (coh_x, coh_y) = b.calc_cohesion(&neighbours[..]);

                Forces {
                    sepx: sep_x,
                    sepy: sep_y,
                    xspeed_avg: align_x,
                    yspeed_avg: align_y,
                    xpos_avg: coh_x,
                    ypos_avg: coh_y,
                    neighbour_amount: neighbours.len(),
                }
            })
            .collect();

        for (i, b) in self.boids.iter_mut().enumerate() {
            b.step(&self.ctx, &forces[i]);

            b.x = b.x.rem_euclid(self.width as f32);
            b.y = b.y.rem_euclid(self.height as f32);
        }
    }

    fn find_neighbours(&self, boid: &boid::Boid, dist: f32) -> Vec<&boid::Boid> {
        self.grid
            .get_possible_neighbours(&boid)
            .iter()
            .map(|id| &self.boids[id.0])
            .filter(|b| {
                let bdist = b.get_distance(boid);
                bdist < dist && bdist > 0.0
            })
            .collect()
    }
}
