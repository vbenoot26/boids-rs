use boids::{context, world};
use raylib::{
    drawing::RaylibTextureModeExt,
    prelude::{Color, RaylibDraw},
};

fn main() {
    let ctx = context::Context::default();
    let mut world = world::init(ctx.clone());

    let (mut rl, thread) = raylib::init().size(ctx.width, ctx.height).build();

    let mut texture = rl
        .load_render_texture(&thread, ctx.width as u32, ctx.height as u32)
        .unwrap();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        world.step();

        let fps_text = format!("FPS: {}", rl.get_fps());

        let mut draw = rl.begin_texture_mode(&thread, &mut texture);

        draw.clear_background(Color::new(0, 0, 0, 128));
        world
            .boids
            .iter()
            .for_each(|b| draw.draw_pixel(b.x as i32, b.y as i32, Color::WHITE));

        drop(draw);

        let mut draw = rl.begin_drawing(&thread);
        draw.draw_texture(&texture, 0, 0, Color::WHITE);
        draw.draw_text(&fps_text, 10, 10, 20, Color::WHITE);
    }
}
