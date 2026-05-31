pub struct Boid {
    pub x: i32,
    pub y: i32,

    pub speedx: i32,
    pub speedy: i32,
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

    pub fn apply_separation(mut self, neighbours: Vec<&Boid>, factor: i32) {
        let mut dx = 0;
        let mut dy = 0;
        for boid in neighbours {
            dx += self.x - boid.x;
            dy += self.y - boid.y;
        }

        self.speedx += factor * dx;
        self.speedy += factor * dy;
    }
}
