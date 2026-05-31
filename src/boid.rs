pub struct Boid {
    pub x: i32,
    pub y: i32,

    pub speedx: i32,
    pub speedy: i32,

    close_dx: i32,
    close_dy: i32,
}

impl Boid {
    pub fn step(&mut self) {
        self.x += self.speedx;
        self.y += self.speedy;
    }

    pub fn get_distance(&self, other: &Boid) -> f32 {
        return (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
            as f32).sqrt();
    }

    pub fn calc_separation(mut self, neighbours: Vec<&Boid>) {
        self.close_dx = 0;
        self.close_dy = 0;
        for boid in neighbours {
            self.close_dx += self.x - boid.x;
            self.close_dy += self.y - boid.y;
        }
    }
}
