use raylib::prelude::{Color, RaylibDraw};

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init().size(640, 480).build();

    while !rl.window_should_close() {
        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::WHITE);
        draw.draw_text("Howdy, cowboy!", 12, 12, 20, Color::BLACK);
    }
}
