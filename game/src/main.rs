#![allow(dead_code)]

use std::{
    collections::HashSet,
    f64::consts::PI,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    engine::{Color, Key, Renderer, Scene, Shape, V3},
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
    keys_pressed: HashSet<Key>,
}

impl Game {
    fn new(event_queue: Arc<Mutex<EventQueue>>) -> Self {
        Self {
            objects: Vec::new(),
            next_object_id: 0,
            event_queue,
            keys_pressed: HashSet::new(),
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
            if self.keys_pressed.contains(&Key::Left) {
                match &mut object.kind {
                    ObjectKind::SkateBoard { vel, .. } => vel.0 -= 0.01,
                    _ => {}
                }
            }
            if self.keys_pressed.contains(&Key::Right) {
                match &mut object.kind {
                    ObjectKind::SkateBoard { vel, .. } => vel.0 += 0.01,
                    _ => {}
                }
            }
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

    fn event(&mut self, event: engine::Event) {
        match event {
            engine::Event::KeyDown { key } => self.keys_pressed.insert(key),
            engine::Event::KeyUp { key } => self.keys_pressed.remove(&key),
        };
    }
}

struct Object {
    kind: ObjectKind,
    id: u32,
}

enum ObjectKind {
    SkateBoard {
        pos: V3,
        vel: V3,
        rot: V3,
    },
    Obstacle {
        pos: V3,
        vel: V3,
    },
    Ground {
        original_pos: V3,
        pos: V3,
        vel: V3,
        rot: V3,
        grid_item_size: f64,
        grid_width: i32,
        grid_depth: i32,
    },
}

struct ShapeGroupShape {
    shape: Shape,
    offset: V3,
}

struct ShapeGroup {
    shapes: Vec<ShapeGroupShape>,
}

impl ShapeGroup {
    pub fn new(shapes: Vec<ShapeGroupShape>) -> Self {
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

                // rot.0 += delta_time.as_secs_f64() * PI * 1.0;
                // rot.1 += delta_time.as_secs_f64() * PI * 1.0;
                // rot.2 += delta_time.as_secs_f64() * PI * 2.0;
            }
            ObjectKind::Obstacle { pos, vel } => {
                *pos += *vel * delta_time.as_secs_f64();
            }
            ObjectKind::Ground {
                pos,
                vel,
                grid_depth,
                grid_item_size,
                original_pos,
                ..
            } => {
                *pos += *vel * delta_time.as_secs_f64();
                *vel += V3(vel.0, vel.1, vel.0 - 0.01 * delta_time.as_secs_f64());
                if pos.2 <= original_pos.2 - *grid_depth as f64 * *grid_item_size as f64 {
                    pos.2 += *grid_depth as f64 * *grid_item_size as f64
                }
            }
        }
    }

    fn render(&self, scene: &mut Scene) {
        match self.kind {
            ObjectKind::SkateBoard { pos, rot, .. } => {
                let board = ShapeGroup::new(vec![
                    ShapeGroupShape {
                        shape: Shape::new_cube(V3(0.2, 0.01, 0.05)),
                        offset: V3(0.0, 0.0, 0.0),
                    },
                    ShapeGroupShape {
                        shape: Shape::new_cube(V3(0.02, 0.02, 0.05)),
                        offset: V3(0.04, -0.02, 0.0),
                    },
                    ShapeGroupShape {
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
            ObjectKind::Ground {
                pos,
                grid_depth,
                grid_item_size,
                grid_width,
                ..
            } => {
                for grid in 0..5 {
                    let mut shapes = Vec::new();
                    for z in 0..grid_depth {
                        for x in -grid_width / 2..grid_width / 2 {
                            shapes.push(ShapeGroupShape {
                                shape: Shape::new_plane(V3(grid_item_size, 0.0, grid_item_size)),
                                offset: V3(
                                    x as f64 * grid_item_size,
                                    0.0,
                                    z as f64 * grid_item_size
                                        + grid as f64 * grid_depth as f64 * grid_item_size,
                                ),
                            });
                        }
                    }
                    let ground = ShapeGroup::new(shapes);
                    ground.draw(pos, scene, Color::Cyan, Color::Black);
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
        pos: V3(0.0, -0.1, -0.7),
        vel: V3(0.0, 0.0, 0.0),
        rot: V3(0.0, PI * 0.5, 0.0),
    });
    objects.push(ObjectKind::Ground {
        original_pos: V3(0.0, -0.25, -0.6),
        pos: V3(0.0, -0.25, -0.6),
        vel: V3(0.0, 0.0, -0.1),
        rot: V3(0.0, 0.0, 0.0),
        grid_item_size: 0.1,
        grid_width: 10,
        grid_depth: 10,
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
