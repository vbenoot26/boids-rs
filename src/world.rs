use crate::boid;
use crate::context;

pub struct World {
    pub boids: Vec<boid::Boid>,
    width: i32,
    height: i32,

    ctx: context::Context,
}

pub fn init(ctx: context::Context) -> World {
    let boid_singleton = vec![boid::Boid::default()];

    return World {
        boids: boid_singleton,
        width: ctx.width,
        height: ctx.height,
        ctx: ctx,
    };
}

impl World {
    pub fn step(&mut self) {
        for b in &mut self.boids {
            b.step();

            b.x = b.x.rem_euclid(self.width as f32);
            b.y = b.y.rem_euclid(self.height as f32);
        }
    }

    fn find_neighbours(&self, boid: &boid::Boid) -> Vec<&boid::Boid> {
        self.boids
            .iter()
            .filter(|b| b.get_distance(boid) < (self.ctx.viewing_distance))
            .collect()
    }
}
