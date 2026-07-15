use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Instant,
};

use boids::{context, world};
use raylib::{
    drawing::RaylibTextureModeExt,
    prelude::{Color, RaylibDraw},
};

fn main() {
    let mut ctx = context::Context::default();

    let (mut rl, thread) = raylib::init()
        .size(ctx.width, ctx.height)
        .fullscreen()
        .build();

    ctx.width = rl.get_screen_width();
    ctx.height = rl.get_screen_height();

    let mut texture = rl
        .load_render_texture(&thread, ctx.width as u32, ctx.height as u32)
        .unwrap();

    let (boids_tx, boids_rx) = mpsc::sync_channel(1);

    let stop = Arc::new(AtomicBool::new(false));
    let stop_clone = stop.clone();

    thread::spawn(move || {
        let mut speeds = vec![(0.0, 0.0); ctx.boid_amount];
        let mut world = world::init(ctx.clone(), &mut rand::rng());
        let mut prev_end = Instant::now();
        while !stop_clone.load(Ordering::Relaxed) {
            let delta_t = Instant::now().checked_duration_since(prev_end).unwrap();
            prev_end = Instant::now();
            world.step(&mut speeds, delta_t);

            let locations: Vec<(f32, f32)> = world.boids.iter().map(|b| (b.x, b.y)).collect();
            let res = boids_tx.send(locations);
            if let Err(_) = res {
                break;
            }
        }
    });

    for boids in boids_rx {
        if rl.window_should_close() {
            stop.store(true, Ordering::Relaxed);
            break;
        }

        let fps_text = format!("FPS: {}", rl.get_fps());

        let mut draw = rl.begin_texture_mode(&thread, &mut texture);

        draw.clear_background(Color::new(0, 0, 0, 128));
        boids
            .iter()
            .for_each(|b| draw.draw_pixel(b.0 as i32, b.1 as i32, Color::new(20, 153, 17, 255)));

        drop(draw);

        let mut draw = rl.begin_drawing(&thread);
        draw.draw_texture(&texture, 0, 0, Color::WHITE);
        draw.draw_text(&fps_text, 10, 10, 20, Color::WHITE);
    }
}
