use raylib::prelude::{Color, RaylibDraw};

mod boid;
mod context;
mod world;

fn main() {
    println!("Hello, world!");
    let ctx = context::Context::default();
    let mut world = world::init(ctx.clone());

    let (mut rl, thread) = raylib::init().size(ctx.width, ctx.height).build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        world.step();

        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::BLACK);
        world
            .boids
            .iter()
            .for_each(|b| draw.draw_circle(b.x as i32, b.y as i32, 2.0, Color::PURPLE));
    }
}
