use crate::{boid::Boid, context::Context, world::BoidId};

struct Rectangle {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,

    boids: Vec<BoidId>,
}

pub struct Grid {
    rectangles: Vec<Vec<Rectangle>>,
    width: i32,
}

pub fn init(ctx: &Context) -> Grid {
    let width = ctx.viewing_distance;
    let width_pixels = ctx.viewing_distance as i32;

    let cols = (ctx.width as f32 / width).ceil() as usize;
    let rows = (ctx.height as f32 / width).ceil() as usize;

    let rects: Vec<Vec<Rectangle>> = (0..cols)
        .map(|i| -> Vec<Rectangle> {
            (0..rows)
                .map(|j| -> Rectangle {
                    Rectangle {
                        min_x: (i as i32) * width_pixels,
                        max_x: ((i + 1) as i32) * width_pixels,
                        min_y: (j as i32) * width_pixels,
                        max_y: ((j + 1) as i32) * width_pixels,
                        boids: Vec::with_capacity(ctx.boid_amount),
                    }
                })
                .collect()
        })
        .collect();

    Grid {
        rectangles: rects,
        width: width_pixels,
    }
}

impl Grid {
    pub fn distribute(&mut self, boids: &Vec<Boid>) {
        self.clear();

        for (i, boid) in boids.iter().enumerate() {
            let (col, row) = self.get_idx(boid);

            self.rectangles[col][row].boids.push(BoidId(i));
        }
    }

    fn clear(&mut self) {
        for col in self.rectangles.iter_mut() {
            for rect in col.iter_mut() {
                rect.boids.clear();
            }
        }
    }

    fn get_idx(&self, boid: &Boid) -> (usize, usize) {
        let col = (boid.x / self.width as f32).floor();
        let row = (boid.y / self.width as f32).floor();

        (col as usize, row as usize)
    }
}
