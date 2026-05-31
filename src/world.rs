use crate::boid;
use crate::context;

pub struct World {
    pub boids: Vec<boid::Boid>,
    width: i32,
    height: i32,

    ctx: context::Context,
}

pub fn init(ctx: context::Context) -> World {
    let boid_singleton = vec![boid::Boid {
        x: 12,
        y: 12,
        speedx: 10,
        speedy: 5,
    }];

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
            b.x += b.speedx;
            b.y += b.speedy;

            b.x = b.x.rem_euclid(self.width);
            b.y = b.y.rem_euclid(self.height);
        }
    }

    fn find_neighbours(&self, boid: &boid::Boid) -> Vec<&boid::Boid> {
        self.boids
            .iter()
            .filter(|b| b.get_distance(boid) < (self.ctx.viewing_distance as f32))
            .collect()
    }
}
