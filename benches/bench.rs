use criterion::{criterion_group, criterion_main, Criterion};

fn bench_step(c: &mut Criterion) {
    let ctx = boids::context::Context::default();
    let mut world = boids::world::init(ctx);

    c.bench_function("world_step", |b| {
        b.iter(|| world.step());
    });
}

criterion_group!(benches, bench_step);
criterion_main!(benches);
