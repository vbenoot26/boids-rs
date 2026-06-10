use boids::{context, world};
use raylib::prelude::{Color, RaylibDraw};

fn main() {
    let ctx = context::Context::default();
    let mut world = world::init(ctx.clone());

    let (mut rl, thread) = raylib::init().size(ctx.width, ctx.height).build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        world.step();

        let fps_text = format!("FPS: {}", rl.get_fps());
        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::BLACK);
        world
            .boids
            .iter()
            .for_each(|b| draw.draw_pixel(b.x as i32, b.y as i32, Color::WHITE));

        draw.draw_text(&fps_text, 10, 10, 20, Color::WHITE);
    }
}
