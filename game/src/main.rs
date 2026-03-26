#![allow(dead_code)]

use std::time::Duration;

use crate::engine::{Color, R3d, V3};

mod engine;

enum MyObject {
    Player { pos: V3, vel: V3 },
}

impl<R: engine::Renderer> engine::Object<R> for MyObject {
    fn update(&mut self, delta_time: Duration) {
        match self {
            MyObject::Player { pos, vel } => {
                *pos += *vel * delta_time.as_secs_f64();
            }
        }
    }

    fn render(&mut self, r: &mut R) {
        match self {
            MyObject::Player { pos, .. } => {
                let mut r3d = R3d::new(r);
                r3d.draw_cube(*pos, V3(0.2, 0.2, 0.2), Color::GREEN, Color::WHITE);
                r3d.draw_cube(
                    *pos + V3(-0.4, -0.2, 0.0),
                    V3(0.2, 0.2, 0.2),
                    Color::GREEN,
                    Color::WHITE,
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sdl_io = engine::SdlIo::new()?;
    let mut game = engine::Game::<engine::SdlIo, MyObject>::new()?;

    let player = MyObject::Player {
        pos: V3(-0.1, -0.1, 0.0),
        vel: V3(0.0, 0.1, 0.0),
    };
    game.spawn(player);

    sdl_io.run(&mut game);
    Ok(())
}
