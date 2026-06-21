use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};
use rand::SeedableRng;
use rand::rngs::StdRng;

fn bench_step(c: &mut Criterion) {
    let ctx = boids::context::Context::default();
    let mut speeds = vec![(0.0, 0.0); ctx.boid_amount];

    let mut world = boids::world::init(ctx, &mut StdRng::seed_from_u64(26));
    let mut prev_time = Instant::now();

    for _ in 0..399 {
        let delta_t = Instant::now().duration_since(prev_time);
        prev_time = Instant::now();
        world.step(&mut speeds, delta_t);
    }

    c.bench_function("world_step", |b| {
        b.iter(|| {
            let delta_t = Instant::now().duration_since(prev_time);
            prev_time = Instant::now();
            world.step(&mut speeds, delta_t);
        });
    });
}

criterion_group!(benches, bench_step);
criterion_main!(benches);
