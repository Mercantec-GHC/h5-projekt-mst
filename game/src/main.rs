#![allow(dead_code)]

use std::{f64, time::Duration};

use crate::engine::{Color, R3d, Shape, V3};

mod engine;

enum MyObject {
    Player { pos: V3, vel: V3 },
}

impl<R: engine::Renderer> engine::Object<R> for MyObject {
    fn update(&mut self, delta_time: Duration) {
        match self {
            MyObject::Player { pos, vel } => {
                *pos += *vel * delta_time.as_secs_f64();
                if pos.0 >= 2.1 {
                    pos.0 = -2.0;
                }
            }
        }
    }

    fn render(&mut self, r: &mut R) {
        match self {
            MyObject::Player { pos, .. } => {
                let mut r3d = R3d::new(r);

                for z in 0..10 {
                    for x in -10..10 {
                        r3d.draw_shape(
                            V3(x as f64 * 0.1, -0.5, z as f64 * 0.1),
                            &Shape::new_plane(V3(0.1, 0.0, 0.1)),
                            Color::Cyan,
                            Color::Black,
                        );
                    }
                }

                r3d.draw_shape(
                    *pos + V3(0.0, 0.2, 0.0),
                    &Shape::new_cube(V3(0.2, 0.1, 0.2)),
                    Color::Green,
                    Color::Black,
                );
                r3d.draw_shape(
                    *pos + V3(-0.8, -0.2, 0.0),
                    &Shape::new_cube(V3(0.2, 0.2, 0.2))
                        .translate(V3(-0.1, -0.1, -0.1))
                        .rotate(V3(pos.0 * 5.0, pos.0 * 5.0, pos.0 * 5.0)),
                    Color::Green,
                    Color::Black,
                );
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sdl_io = engine::SdlIo::new()?;
    let mut game = engine::Game::<engine::SdlIo, MyObject>::new()?;

    let player = MyObject::Player {
        pos: V3(-1.0, -0.1, 0.0),
        vel: V3(0.4, 0.0, 0.0),
    };
    game.spawn(player);

    sdl_io.run(&mut game);
    Ok(())
}
