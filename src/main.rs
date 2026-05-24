use raylib::prelude::{Color, RaylibDraw};

struct Boid {
    x: i32,
    y: i32,

    speedx: i32,
    speedy: i32,
}

impl Boid {
    fn step(&mut self) {
        self.x += self.speedx;
        self.y += self.speedy;
    }
}

fn main() {
    println!("Hello, world!");

    let width = 640;
    let height = 480;
    let (mut rl, thread) = raylib::init().size(width, height).build();

    rl.set_target_fps(60);

    let mut boid = Boid {
        x: 12,
        y: 12,
        speedx: 10,
        speedy: 5,
    };

    while !rl.window_should_close() {
        boid.step();

        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::WHITE);
        draw.draw_circle(boid.x % width, boid.y % height, 20.0, Color::BLACK);
    }
}
