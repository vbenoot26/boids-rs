use crate::boid;

pub struct World {
    pub boids: Vec<boid::Boid>,
    width: i32,
    height: i32,
}

pub fn init(width: i32, height: i32) -> World {
    let boid_singleton = vec![boid::Boid {
        x: 12,
        y: 12,
        speedx: 10,
        speedy: 5,
    }];

    return World {
        boids: boid_singleton,
        width,
        height,
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
}
