pub struct Boid {
    pub x: f32,
    pub y: f32,

    pub speedx: f32,
    pub speedy: f32,
}

#[derive(Default, Clone)]
pub struct Forces {
    pub sepx: f32,
    pub sepy: f32,

    pub xspeed_avg: f32,
    pub yspeed_avg: f32,

    pub xpos_avg: f32,
    pub ypos_avg: f32,

    pub neighbour_amount: usize,
}

impl Default for Boid {
    fn default() -> Boid {
        Boid {
            x: 20.0,
            y: 20.0,
            speedx: 10.0,
            speedy: 10.0,
        }
    }
}

impl Boid {
    pub fn step(&mut self, forces: &Forces) {
        self.x += self.speedx;
        self.y += self.speedy;
    }

    pub fn get_distance(&self, other: &Boid) -> f32 {
        return (((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y))
            as f32).sqrt();
    }

    pub fn calc_separation(&self, neighbours: &[&Boid]) -> (f32, f32) {
        (
            neighbours.iter().map(|b| self.x - b.x).sum(),
            neighbours.iter().map(|b| self.y - b.y).sum(),
        )
    }

    pub fn calc_alignment(&self, neighbours: &[&Boid]) -> (f32, f32) {
        if neighbours.len() == 0 {
            return (0.0, 0.0);
        }

        (
            neighbours.iter().map(|b| b.speedx).sum::<f32>() / (neighbours.len() as f32),
            neighbours.iter().map(|b| b.speedy).sum::<f32>() / (neighbours.len() as f32),
        )
    }

    pub fn calc_cohesion(&self, neighbours: &[&Boid]) -> (f32, f32) {
        if neighbours.len() == 0 {
            return (0.0, 0.0);
        }

        (
            neighbours.iter().map(|b| b.x).sum::<f32>() / (neighbours.len() as f32),
            neighbours.iter().map(|b| b.y).sum::<f32>() / (neighbours.len() as f32),
        )
    }
}
