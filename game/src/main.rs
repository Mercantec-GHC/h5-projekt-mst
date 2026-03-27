#![allow(dead_code)]

use std::time::Duration;

use crate::engine::{Color, Renderer, Scene, Shape, V3};

mod engine;
mod vermiparous;

struct Game {
    objects: Vec<Object>,
}

impl Game {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn spawn(&mut self, object: Object) {
        self.objects.push(object);
    }
}

impl<R: Renderer> engine::Game<R> for Game {
    fn update(&mut self, delta_time: Duration) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
    }

    fn render(&mut self, r: &mut R) {
        let mut scene = Scene::new();
        for object in &mut self.objects {
            object.render(&mut scene);
        }
        scene.render(r, V3(0.0, 0.0, -1.0));
    }

    fn event(&mut self, event: engine::Event) {}
}

enum Object {
    Player { pos: V3, vel: V3 },
    Obstacle { pos: V3, vel_z: f64 },
    Ground { pos: V3, vel_z: f64 },
}

impl Object {
    fn update(&mut self, delta_time: Duration) {
        match self {
            Object::Player { pos, vel } => {
                *pos += *vel * delta_time.as_secs_f64();
                if pos.0 >= 2.1 {
                    pos.0 = -2.0;
                }
            }
            Object::Obstacle { pos, vel_z } => {
                pos.2 += *vel_z * delta_time.as_secs_f64();
            }
            Object::Ground { pos, vel_z } => {
                pos.2 += *vel_z * delta_time.as_secs_f64();
            }
        }
    }

    fn render(&mut self, scene: &mut Scene) {
        match self {
            Object::Player { pos, .. } => {
                scene.draw_shape(
                    *pos + V3(0.0, 0.2, 0.0),
                    &Shape::new_cube(V3(0.2, 0.1, 0.2)),
                    Color::Green,
                    Color::Black,
                );
                scene.draw_shape(
                    *pos + V3(-0.8, -0.2, 0.0),
                    &Shape::new_cube(V3(0.2, 0.2, 0.2))
                        .translate(V3(-0.1, -0.1, -0.1))
                        .rotate(V3(pos.0 * 5.0, pos.0 * 5.0, pos.0 * 5.0)),
                    Color::Green,
                    Color::Black,
                );
            }
            Object::Obstacle { pos, vel_z } => todo!(),
            Object::Ground { pos, .. } => {
                for z in 0..10 {
                    for x in -10..10 {
                        scene.draw_shape(
                            V3(x as f64 * 0.1 + pos.0, pos.1, z as f64 * 0.1 + pos.2),
                            &Shape::new_plane(V3(0.1, 0.0, 0.1)),
                            Color::Cyan,
                            Color::Black,
                        );
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sdl_io = engine::SdlIo::new()?;
    let mut game = Game::new();

    let player = Object::Player {
        pos: V3(-1.0, -0.1, 0.0),
        vel: V3(0.4, 0.0, 0.0),
    };
    game.spawn(player);
    let ground = Object::Ground {
        pos: V3(0.0, -0.3, -0.5),
        vel_z: -0.1,
    };
    game.spawn(ground);

    sdl_io.run(&mut game);
    Ok(())
}
