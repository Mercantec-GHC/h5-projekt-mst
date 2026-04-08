#![allow(dead_code)]

pub mod editor;
mod engine;
mod event_queue;
pub mod vermiparous;

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

struct Skateboard {
    pos: V3,
    vel: V3,
    rot: V3,
    nyoom_factor: f64,
    pivot_deg: f64,
}

impl Skateboard {
    fn update(&mut self, delta_time: Duration) {
        self.vel.0 = self.pivot_deg * delta_time.as_secs_f64();
        self.pos += self.vel * delta_time.as_secs_f64();
        self.nyoom_factor += 16.0 * delta_time.as_secs_f64();

        // rot.0 += delta_time.as_secs_f64() * PI * 1.0;
        // rot.1 += delta_time.as_secs_f64() * PI * 0.2;
        // rot.2 += delta_time.as_secs_f64() * PI * 2.0;
    }

    fn render(&self, scene: &mut Scene) {
        let board_size = V3(0.175, 0.005, 0.05);
        let trucks = {
            let anchor_size = V3(0.005, 0.01, 0.005);
            let anchor_y = -anchor_size.1 - board_size.1 * 0.5;
            let pivot = vec![
                ShapeGroupShape {
                    shape: Shape::new_cube(anchor_size),
                    offset: V3(
                        anchor_size.0 * -0.5 + board_size.0 * 0.4,
                        anchor_y,
                        anchor_size.2 * -0.5,
                    ),
                },
                ShapeGroupShape {
                    shape: Shape::new_cube(anchor_size),
                    offset: V3(
                        anchor_size.0 * -0.5 - board_size.0 * 0.4,
                        anchor_y,
                        anchor_size.2 * -0.5,
                    ),
                },
            ];
            let wheel_rail_size = V3(anchor_size.0, anchor_size.0, board_size.2 * 0.8);
            let wheel_rail_y = anchor_y - wheel_rail_size.1;
            let wheel_rail = vec![
                ShapeGroupShape {
                    shape: Shape::new_cube(wheel_rail_size),
                    offset: V3(
                        wheel_rail_size.0 * -0.5 + board_size.0 * 0.4,
                        wheel_rail_y,
                        wheel_rail_size.2 * -0.5,
                    ),
                },
                ShapeGroupShape {
                    shape: Shape::new_cube(wheel_rail_size),
                    offset: V3(
                        wheel_rail_size.0 * -0.5 - board_size.0 * 0.4,
                        wheel_rail_y,
                        wheel_rail_size.2 * -0.5,
                    ),
                },
            ];
            let wheel_size = V3(0.01, 0.01, 0.0025);
            let wheel_y = wheel_rail_y - wheel_size.1 * 0.25;
            let wheel_rot = V3(0.0, 0.0, self.nyoom_factor);
            let wheel_rot_trans = V3(0.5, 0.5, 0.0);
            let wheel = vec![
                ShapeGroupShape {
                    shape: Shape::new_cube(wheel_size)
                        .translate(wheel_size * wheel_rot_trans * -1.0)
                        .rotate(wheel_rot)
                        .translate(wheel_size * wheel_rot_trans),
                    offset: V3(
                        wheel_size.0 * -0.5 + board_size.0 * 0.4,
                        wheel_y,
                        wheel_rail_size.2 * -0.5 - wheel_size.2,
                    ),
                },
                ShapeGroupShape {
                    shape: Shape::new_cube(wheel_size)
                        .translate(wheel_size * wheel_rot_trans * -1.0)
                        .rotate(wheel_rot)
                        .translate(wheel_size * wheel_rot_trans),

                    offset: V3(
                        wheel_size.0 * -0.5 - board_size.0 * 0.4,
                        wheel_y,
                        wheel_rail_size.2 * -0.5 - wheel_size.2,
                    ),
                },
                ShapeGroupShape {
                    shape: Shape::new_cube(wheel_size)
                        .translate(wheel_size * wheel_rot_trans * -1.0)
                        .rotate(wheel_rot)
                        .translate(wheel_size * wheel_rot_trans),

                    offset: V3(
                        wheel_size.0 * -0.5 + board_size.0 * 0.4,
                        wheel_y,
                        wheel_rail_size.2 * 0.5,
                    ),
                },
                ShapeGroupShape {
                    shape: Shape::new_cube(wheel_size)
                        .translate(wheel_size * wheel_rot_trans * -1.0)
                        .rotate(wheel_rot)
                        .translate(wheel_size * wheel_rot_trans),

                    offset: V3(
                        wheel_size.0 * -0.5 - board_size.0 * 0.4,
                        wheel_y,
                        wheel_rail_size.2 * 0.5,
                    ),
                },
            ];
            vec![pivot, wheel_rail, wheel].into_iter().flatten()
        };
        let board_pivot_trans = V3(0.0, 0.5, 0.5);
        let mut board = vec![ShapeGroupShape {
            shape: Shape::new_cube(board_size)
                .translate(board_size * board_pivot_trans * -1.0)
                .rotate(V3(self.pivot_deg * (PI / 180.0), 0.0, 0.0))
                .translate(board_size * board_pivot_trans),
            offset: board_size * -0.5,
        }];
        board.extend(trucks);
        let board = ShapeGroup::new(board).rotate(self.rot);
        board.draw(self.pos, scene, Color::Green, Color::Black);
    }
}

struct Game {
    skateboard: Skateboard,
    camera_pos: V3,
    objects: Vec<Object>,
    next_object_id: u32,
    event_queue: Arc<Mutex<EventQueue>>,
    keys_pressed: HashSet<Key>,
}

impl Game {
    fn new(event_queue: Arc<Mutex<EventQueue>>) -> Self {
        Self {
            skateboard: Skateboard {
                pos: V3(0.0, -0.15, -0.4),
                vel: V3(0.0, 0.0, 0.2),
                rot: V3(0.0, PI * 0.5, 0.0),
                nyoom_factor: 0.0,
                pivot_deg: 0.0,
            },
            camera_pos: V3(0.0, 0.0, -1.0),
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
        self.skateboard.update(delta_time);
        self.camera_pos = V3(
            self.skateboard.pos.0,
            self.skateboard.pos.1 + 0.15,
            self.skateboard.pos.2 - 0.4,
        );

        if self.keys_pressed.contains(&Key::Left) == self.keys_pressed.contains(&Key::Right) {
            let decay_rate = 1.0 - (4.0 * delta_time.as_secs_f64());
            self.skateboard.pivot_deg *= decay_rate;
        }
        if self.keys_pressed.contains(&Key::Left) {
            self.skateboard.pivot_deg -= 36.0 * delta_time.as_secs_f64();
            if self.skateboard.pivot_deg < -12.5 {
                self.skateboard.pivot_deg = -12.5;
            }
        }
        if self.keys_pressed.contains(&Key::Right) {
            self.skateboard.pivot_deg += 36.0 * delta_time.as_secs_f64();
            if self.skateboard.pivot_deg > 12.5 {
                self.skateboard.pivot_deg = 12.5;
            }
        }

        for object in &mut self.objects {
            object.update(delta_time);
        }
    }

    fn render(&mut self, r: &mut R) {
        let mut scene = Scene::new();
        self.skateboard.render(&mut scene);
        for object in &self.objects {
            object.render(&mut scene);
        }
        scene.render(r, self.camera_pos, V3(0.0, 0.0, 0.0));
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
    Section {
        objects: Vec<ObjectKind>,
        pos: V3,
        vel: V3,
    },
    Obstacle {
        pos: V3,
        vel: V3,
    },
    Ground {
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
    pub shapes: Vec<ShapeGroupShape>,
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
            ObjectKind::Obstacle { pos, vel, .. } => {
                *pos += *vel * delta_time.as_secs_f64();
            }
            ObjectKind::Ground { pos, vel, .. } => {

                // *vel += V3(vel.0, vel.1, vel.0 - 0.01 * delta_time.as_secs_f64());
            }
            ObjectKind::Section { objects, .. } => todo!(),
        }
    }

    fn render(&self, scene: &mut Scene) {
        match self.kind {
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
                let mut shapes = Vec::new();
                for z in 0..grid_depth {
                    for x in -grid_width / 2..grid_width / 2 {
                        shapes.push(ShapeGroupShape {
                            shape: Shape::new_plane(V3(grid_item_size, 0.0, grid_item_size)),
                            offset: V3(x as f64 * grid_item_size, 0.0, z as f64 * grid_item_size),
                        });
                    }
                }
                let ground = ShapeGroup::new(shapes);
                ground.draw(pos, scene, Color::Cyan, Color::Black);
            }
            ObjectKind::Section { ref objects, .. } => {}
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
    let objects: Vec<ObjectKind> = vec![
        ObjectKind::Ground {
            pos: V3(0.0, -0.35, -0.4),
            vel: V3(0.0, 0.0, 0.0),
            rot: V3(0.0, 0.0, 0.0),
            grid_item_size: 0.1,
            grid_width: 10,
            grid_depth: 20,
        },
        ObjectKind::Ground {
            pos: V3(0.0, -0.35, 1.6),
            vel: V3(0.0, 0.0, 0.0),
            rot: V3(0.0, 0.0, 0.0),
            grid_item_size: 0.1,
            grid_width: 10,
            grid_depth: 20,
        },
    ];

    for object in objects {
        game.spawn(object);
    }

    sdl_io.run(&mut game);
    Ok(())
}
