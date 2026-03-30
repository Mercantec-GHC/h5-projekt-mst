#![allow(dead_code)]

use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    engine::{Color, Renderer, Scene, Shape, V3},
    event_queue::EventQueue,
    vermiparous::Server,
};

mod engine;
mod event_queue;
mod vermiparous;

struct Game {
    objects: Vec<Object>,
    next_object_id: u32,
    event_queue: Arc<Mutex<EventQueue>>,
}

impl Game {
    fn new(event_queue: Arc<Mutex<EventQueue>>) -> Self {
        Self {
            objects: Vec::new(),
            next_object_id: 0,
            event_queue,
        }
    }

    fn spawn(&mut self, object_kind: ObjectKind) {
        let object = Object {
            kind: object_kind,
            id: self.next_object_id,
        };
        self.objects.push(object);
        self.next_object_id += 1
    }

    fn despawn(&mut self, id: u32) {
        let index = self
            .objects
            .iter()
            .position(|o| o.id == id)
            .expect("doesn't exist");
        self.objects.remove(index);
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
        for object in &self.objects {
            object.render(&mut scene);
        }
        scene.render(r, V3(0.0, 0.0, -1.0));
    }

    fn event(&mut self, event: engine::Event) {}
}

struct Object {
    kind: ObjectKind,
    id: u32,
}

enum ObjectKind {
    SkateBoard { pos: V3, vel: V3, rot: V3 },
    Obstacle { pos: V3, vel: V3 },
    Ground { pos: V3, vel_z: f64 },
}

struct MultiShapeShape {
    shape: Shape,
    offset: V3,
}

struct MultiShape {
    shapes: Vec<MultiShapeShape>,
}

impl MultiShape {
    pub fn new(shapes: Vec<MultiShapeShape>) -> Self {
        Self { shapes }
    }
    pub fn rotate(mut self, rot: V3) -> Self {
        for shape in &mut self.shapes {
            shape.shape = shape
                .shape
                .translate(shape.offset)
                .rotate(rot)
                .translate(shape.offset * -1.0);
        }
        self
    }
    pub fn translate(mut self, offset: V3) -> Self {
        for shape in &mut self.shapes {
            shape.shape = shape.shape.translate(offset);
        }
        self
    }
    pub fn draw(self, pos: V3, scene: &mut Scene, outline_color: Color, fill_color: Color) {
        for shape in self.shapes {
            scene.draw_shape(
                pos,
                &shape.shape.translate(shape.offset),
                outline_color,
                fill_color,
            );
        }
    }
}

impl Object {
    fn update(&mut self, delta_time: Duration) {
        match &mut self.kind {
            ObjectKind::SkateBoard { pos, vel, rot } => {
                *pos += *vel * delta_time.as_secs_f64();
                if pos.0 >= 2.1 {
                    pos.0 = -2.0;
                }
                // rot.0 += delta_time.as_secs_f64() * PI * 1.0;
                rot.1 += delta_time.as_secs_f64() * PI * 1.0;
                // rot.2 += delta_time.as_secs_f64() * PI * 2.0;
            }
            ObjectKind::Obstacle { pos, vel } => {
                *pos += *vel * delta_time.as_secs_f64();
            }
            ObjectKind::Ground { pos, vel_z } => {
                pos.2 += *vel_z * delta_time.as_secs_f64();
            }
        }
    }

    fn render(&self, scene: &mut Scene) {
        match self.kind {
            ObjectKind::SkateBoard { pos, rot, .. } => {
                let board = MultiShape::new(vec![
                    MultiShapeShape {
                        shape: Shape::new_cube(V3(0.2, 0.01, 0.05)),
                        offset: V3(0.0, 0.0, 0.0),
                    },
                    MultiShapeShape {
                        shape: Shape::new_cube(V3(0.02, 0.02, 0.05)),
                        offset: V3(0.04, -0.02, 0.0),
                    },
                    MultiShapeShape {
                        shape: Shape::new_cube(V3(0.02, 0.02, 0.05)),
                        offset: V3(0.14, -0.02, 0.0),
                    },
                ])
                .translate(V3(-0.1, -0.005, -0.0025))
                .rotate(rot);
                board.draw(pos, scene, Color::Green, Color::Black);
            }
            ObjectKind::Obstacle { pos, .. } => {
                scene.draw_shape(
                    pos,
                    &Shape::new_cube(V3(0.2, 0.1, 0.2)),
                    Color::Green,
                    Color::Black,
                );
            }
            ObjectKind::Ground { pos, .. } => {
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
    let event_queue = Arc::new(Mutex::new(EventQueue::new()));
    let mut game = Game::new(event_queue.clone());
    std::thread::spawn(move || {
        let server = Server::bind("10.133.51.127:5000");
        server.start(event_queue);
    });
    let mut objects: Vec<ObjectKind> = Vec::new();
    objects.push(ObjectKind::SkateBoard {
        pos: V3(0.0, -0.1, 0.0),
        vel: V3(0.0, 0.0, 0.0),
        rot: V3(0.0, 0.0, 0.0),
    });
    objects.push(ObjectKind::Ground {
        pos: V3(0.0, -0.3, -0.5),
        vel_z: -0.1,
    });
    objects.push(ObjectKind::Obstacle {
        pos: V3(0.5, -0.2, 0.0),
        vel: V3(0.0, 0.0, -0.1),
    });

    for object in objects {
        game.spawn(object)
    }

    sdl_io.run(&mut game);
    Ok(())
}
