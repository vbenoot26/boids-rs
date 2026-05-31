pub struct Boid {
    pub x: i32,
    pub y: i32,

    pub speedx: f32,
    pub speedy: f32,

    forces: Forces,
}

struct Forces {
    close_dx: f32,
    close_dy: f32,

    xspeed_avg: f32,
    yspeed_avg: f32,

    xpos_avg: f32,
    ypos_avg: f32,
}

impl Default for Boid {
    fn default() -> Boid {
        let forces = Forces {
            close_dx: 0.0,
            close_dy: 0.0,
            xspeed_avg: 0.0,
            yspeed_avg: 0.0,
            xpos_avg: 0.0,
            ypos_avg: 0.0,
        };
        Boid {
            x: 20,
            y: 20,
            speedx: 10.0,
            speedy: 10.0,
            forces: forces,
        }
    }
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

    pub fn calc_separation(&mut self, neighbours: &[&Boid]) {
        self.forces.close_dx = neighbours
            .iter()
            .map(|b| self.x - b.x)
            .map(|b| b as f32)
            .sum();
        self.forces.close_dy = neighbours
            .iter()
            .map(|b| self.y - b.y)
            .map(|b| b as f32)
            .sum();
    }

    pub fn calc_alignment(&mut self, neighbours: &[&Boid]) {
        self.forces.xspeed_avg = 0.0;
        self.forces.yspeed_avg = 0.0;

        if neighbours.len() == 0 {
            return;
        }

        self.forces.xspeed_avg =
            neighbours.iter().map(|b| b.speedx).sum::<f32>() / (neighbours.len() as f32);
        self.forces.yspeed_avg =
            neighbours.iter().map(|b| b.speedy).sum::<f32>() / (neighbours.len() as f32);
    }

    pub fn calc_cohesion(&mut self, neighbours: &[&Boid]) {
        self.forces.xpos_avg = 0.0;
        self.forces.ypos_avg = 0.0;

        if neighbours.len() == 0 {
            return;
        }

        self.forces.xpos_avg = neighbours
            .iter()
            .map(|b| b.x)
            .map(|x| x as f32)
            .sum::<f32>()
            / (neighbours.len() as f32);
        self.forces.ypos_avg = neighbours
            .iter()
            .map(|b| b.y)
            .map(|y| y as f32)
            .sum::<f32>()
            / (neighbours.len() as f32);
    }
}
