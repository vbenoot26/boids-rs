use criterion::{Criterion, criterion_group, criterion_main};
use rand::SeedableRng;
use rand::rngs::StdRng;

fn bench_step(c: &mut Criterion) {
    let ctx = boids::context::Context::default();
    let mut speeds = vec![(0.0, 0.0); ctx.boid_amount];

    let mut world = boids::world::init(ctx, &mut StdRng::seed_from_u64(26));

    for _ in 0..399 {
        world.step(&mut speeds);
    }

    c.bench_function("world_step", |b| {
        b.iter(|| world.step(&mut speeds));
    });
}

criterion_group!(benches, bench_step);
criterion_main!(benches);
