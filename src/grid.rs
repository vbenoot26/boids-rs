use std::mem::take;

use crate::{boid::Boid, context::Context, world::BoidId};

struct Rectangle {
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
        .map(|_| {
            (0..rows)
                .map(|_| Rectangle {
                    boids: Vec::with_capacity(ctx.boid_amount),
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
    pub fn distribute(&mut self, boids: &[Boid]) {
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

        (
            (col as usize).min(self.rectangles.len() - 1),
            (row as usize).min(self.rectangles[0].len() - 1),
        )
    }

    pub fn get_possible_neighbours(&self, boid: &Boid) -> impl Iterator<Item = BoidId> {
        let (col_center, row_center) = self.get_idx(boid);

        let col_min = col_center.saturating_sub(1);
        let col_max = (col_center + 1).min(self.rectangles.len() - 1);

        let row_min = row_center.saturating_sub(1);
        let row_max = (row_center + 1).min(self.rectangles[0].len() - 1);

        (col_min..=col_max).flat_map(move |c| {
            (row_min..=row_max).flat_map(move |r| self.rectangles[c][r].boids.iter().copied())
        })
    }

    pub fn sort_by_location(&mut self, boids: &mut Vec<Boid>) -> Vec<Boid> {
        let mut result = Vec::with_capacity(boids.len());
        let mut counter = 0;

        for col in self.rectangles.iter_mut() {
            for rect in col.iter_mut() {
                for i in 0..rect.boids.len() {
                    result.push(boids[rect.boids[i].0]);

                    rect.boids[i] = BoidId(counter);
                    counter += 1;
                }
            }
        }

        result
    }
}
