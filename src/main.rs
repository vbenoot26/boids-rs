use raylib::prelude::{Color, RaylibDraw};

mod boid;
mod world;

fn main() {
    println!("Hello, world!");

    let width = 640;
    let height = 480;
    let mut world = world::init(width, height);

    let (mut rl, thread) = raylib::init().size(width, height).build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        world.step();

        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::WHITE);
        world
            .boids
            .iter()
            .for_each(|b| draw.draw_circle(b.x, b.y, 20.0, Color::BLACK));
    }
}
