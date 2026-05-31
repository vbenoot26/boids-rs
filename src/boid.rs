pub struct Boid {
    pub x: i32,
    pub y: i32,

    pub speedx: f32,
    pub speedy: f32,

    close_dx: f32,
    close_dy: f32,

    xspeed_avg: f32,
    yspeed_avg: f32,
}

impl Boid {
    pub fn step(&mut self) {
        self.x += self.speedx as i32;
        self.y += self.speedy as i32;
    }

    pub fn get_distance(&self, other: &Boid) -> f32 {
        return (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
            as f32).sqrt();
    }

    pub fn calc_separation(mut self, neighbours: Vec<&Boid>) {
        self.close_dx = 0.0;
        self.close_dy = 0.0;
        for boid in neighbours {
            self.close_dx += (self.x - boid.x) as f32;
            self.close_dy += (self.y - boid.y) as f32;
        }
    }

    pub fn calc_alignment(mut self, neighbours: Vec<&Boid>) {
        self.xspeed_avg = 0.0;
        self.yspeed_avg = 0.0;

        let len = neighbours.len();

        if len == 0 {
            return;
        }

        for boid in neighbours {
            self.xspeed_avg += boid.speedx;
            self.yspeed_avg += boid.speedy;
        }

        self.xspeed_avg /= len as f32;
        self.yspeed_avg /= len as f32;
    }
}
