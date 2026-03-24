#![allow(dead_code)]
#![allow(unused)]

use std::time::Duration;

use sdl3::pixels::Color;

use crate::engine::math::{V2, V3};

mod engine;

enum Object {
    Player { pos: V3, vel: V3 },
}

impl<R: engine::Renderer> engine::Object<R> for Object {
    fn update(&mut self, delta_time: Duration) {
        match self {
            Object::Player { pos, vel } => {
                *pos += *vel * delta_time.as_secs_f64();
            }
        }
    }

    fn render(&mut self, r: &mut R) {
        match self {
            Object::Player { pos, .. } => {
                // r.draw_rect(V2(pos.0, pos.1), V2(400.0, 400.0));
                r.draw_cube(*pos, V3(100.0, 100.0, 100.0), Color::GREEN, Color::WHITE);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sdl_io = engine::SdlIo::new()?;
    let mut game = engine::Game::<engine::SdlIo, Object>::new()?;

    let player = Object::Player {
        pos: V3(-50.0, -50.0, 0.0),
        vel: V3(0.0, 0.0, 1.0),
    };
    game.spawn(player);

    sdl_io.run(&mut game);
    Ok(())
}
