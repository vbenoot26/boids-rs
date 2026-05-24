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
}
