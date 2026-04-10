#![allow(dead_code)]

mod engine;
mod server;
use rand::Rng;

use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    f64::consts::PI,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    engine::{Color, Key, Renderer, Scene, Shape, V2, V3, WIDTH},
    server::Server,
};

struct Skateboard {
    start_pos: V3,
    pos: V3,
    size: V3,
    vel: V3,
    rot: V3,
    nyoom_factor: f64,
    pivot_deg: f64,
    pivot_deg_target: f64,
}

impl Skateboard {
    fn update(&mut self, delta_time: Duration) {
        self.vel.0 = self.pivot_deg * 2.0 * delta_time.as_secs_f64();
        self.pos += self.vel * delta_time.as_secs_f64();
        self.nyoom_factor += 16.0 * delta_time.as_secs_f64();
        self.pos.0 = self.pos.0.clamp(-0.5 + 0.05, 0.5 - 0.05);
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

struct UpdateCx<'game> {
    skateboard: &'game mut Skateboard,
}

struct Game {
    skateboard: Skateboard,
    segments: Vec<Segment>,
    camera_pos: V3,
    keys_pressed: HashSet<Key>,
    event_queue: Arc<Mutex<VecDeque<f64>>>,
}

impl Game {
    fn new() -> Self {
        let start_pos = V3(0.0, -0.15, 0.0);
        Self {
            skateboard: Skateboard {
                start_pos,
                pos: start_pos,
                size: V3(0.05, 0.02, 0.15),
                vel: V3(0.0, 0.0, 0.2),
                rot: V3(0.0, PI * 0.5, 0.0),
                nyoom_factor: 0.0,
                pivot_deg: 0.0,
                pivot_deg_target: 0.0,
            },
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            camera_pos: V3(0.0, 0.0, -0.6),
            segments: Vec::new(),
            keys_pressed: HashSet::new(),
        }
    }

    fn spawn(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    fn despawn(&mut self, id: i32) {
        let index = self
            .segments
            .iter()
            .position(|s| s.id == id)
            .expect("doesn't exist");
        self.segments.remove(index);
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

        // if self.keys_pressed.contains(&Key::Left) == self.keys_pressed.contains(&Key::Right) {
        //     let decay_rate = 1.0 - (4.0 * delta_time.as_secs_f64());
        //     self.skateboard.pivot_deg *= decay_rate;
        // }
        if self.keys_pressed.contains(&Key::Left) {
            self.skateboard.pivot_deg -= 36.0 * delta_time.as_secs_f64();
        }
        if self.keys_pressed.contains(&Key::Right) {
            self.skateboard.pivot_deg += 36.0 * delta_time.as_secs_f64();
        }

        {
            let mut angles = self.event_queue.lock().unwrap();

            for angle in angles.drain(..) {
                self.skateboard.pivot_deg_target = angle;
            }
        }

        self.skateboard.vel.2 += 0.02 * delta_time.as_secs_f64();

        self.skateboard.pivot_deg += (self.skateboard.pivot_deg_target - self.skateboard.pivot_deg)
            * 4.0
            * delta_time.as_secs_f64();
        self.skateboard.pivot_deg = self.skateboard.pivot_deg.clamp(-12.5, 12.5);

        let ids = self
            .segments
            .iter()
            .enumerate()
            .map(|(i, segment)| (i, segment.id))
            .rev()
            .collect::<Vec<_>>();

        for (i, id) in ids {
            let segment = &self.segments[i];
            if segment.should_despawn {
                self.despawn(segment.id);
            }
        }

        let ids = self
            .segments
            .iter()
            .enumerate()
            .map(|(i, segment)| (i, segment.id))
            .collect::<Vec<_>>();

        let mut cx = UpdateCx {
            skateboard: &mut self.skateboard,
        };

        for (i, id) in ids {
            let segment = &mut self.segments[i];
            segment.update(&mut cx, delta_time);
        }
    }

    fn render(&mut self, r: &mut R) {
        let mut scene = Scene::new();
        self.skateboard.render(&mut scene);
        for object in &self.segments {
            object.render(&mut scene);
        }
        scene.render(r, self.camera_pos);
        let id = r.load_text(
            &format!("{}", (self.skateboard.pos.2 * 100.0) as u32),
            48.0,
            Color::Green,
        );
        let V2(width, _height) = r.query_texture(id);
        r.draw_texture(id, V2(WIDTH / 2.0 - width / 2.0, 50.0));
    }

    fn event(&mut self, event: engine::Event) {
        match event {
            engine::Event::KeyDown { key } => self.keys_pressed.insert(key),
            engine::Event::KeyUp { key } => self.keys_pressed.remove(&key),
        };
    }
}
#[derive(Clone, Copy)]

struct Obstacle {
    pos: V3,
    vel: V3,
    size: V3,
}
#[derive(Clone, Copy)]

struct Ground {
    pos: V3,
    rot: V3,
    grid_item_size: f64,
    grid_width: i32,
    grid_depth: i32,
}

struct Segment {
    id: i32,
    obstacles: Vec<Obstacle>,
    ground: Ground,
    should_despawn: bool,
}

impl Segment {
    fn new(id: i32, obstacles: Vec<Obstacle>, ground: Ground) -> Self {
        Self {
            id,
            obstacles,
            ground,
            should_despawn: false,
        }
    }

    fn update<'game>(&mut self, cx: &mut UpdateCx<'game>, delta_time: Duration) {
        for obstacle in &mut self.obstacles {
            obstacle.pos += obstacle.vel * delta_time.as_secs_f64();
            let (skateboard_pos, skateboard_pos_and_size) = (
                V2(
                    cx.skateboard.pos.0 - cx.skateboard.size.0 / 2.0,
                    cx.skateboard.pos.2 - cx.skateboard.size.2 / 2.0,
                ),
                V2(
                    cx.skateboard.pos.0 + cx.skateboard.size.0 / 2.0,
                    cx.skateboard.pos.2 + cx.skateboard.size.2 / 2.0,
                ),
            );
            if skateboard_pos.0 < obstacle.pos.0 + obstacle.size.0
                && skateboard_pos_and_size.0 > obstacle.pos.0
                && skateboard_pos.1 < obstacle.pos.2 + obstacle.size.2
                && skateboard_pos_and_size.1 > obstacle.pos.2
            {
                panic!("You lost 🫵 😂");
            }
        }
        if cx.skateboard.pos.2
            >= self.ground.pos.2 + self.ground.grid_depth as f64 * self.ground.grid_item_size as f64
        {
            self.should_despawn = true
        }
    }

    fn render(&self, scene: &mut Scene) {
        for obstacle in &self.obstacles {
            scene.draw_shape(
                obstacle.pos,
                &Shape::new_cube(obstacle.size),
                Color::Red,
                Color::Black,
            );
        }
        let mut shapes = Vec::new();
        for z in 0..self.ground.grid_depth {
            for x in -self.ground.grid_width / 2..self.ground.grid_width / 2 {
                shapes.push(ShapeGroupShape {
                    shape: Shape::new_plane(V3(
                        self.ground.grid_item_size,
                        0.0,
                        self.ground.grid_item_size,
                    )),
                    offset: V3(
                        x as f64 * self.ground.grid_item_size,
                        0.0,
                        z as f64 * self.ground.grid_item_size,
                    ),
                });
            }
        }
        let ground = ShapeGroup::new(shapes);
        ground.draw(self.ground.pos, scene, Color::Cyan, Color::Black);
    }
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sdl_io = engine::SdlIo::new()?;

    let mut game = Game::new();
    let event_queue = game.event_queue.clone();
    let _ = thread::spawn(move || {
        let mut server = Server::new("10.133.51.127:8888").unwrap();
        server
            .subscribe(|measurement| {
                event_queue.lock().unwrap().push_back(measurement.angle);
                println!("angle = {}", measurement.angle);
            })
            .unwrap();
    });

    let first_segment: Segment = Segment::new(
        0,
        vec![Obstacle {
            pos: V3(0.0, -0.248, 1.0),
            vel: V3(0.0, 0.0, 0.0),
            size: V3(0.1, 0.1, 0.1),
        }],
        Ground {
            pos: V3(0.0, -0.25, -0.4),
            rot: V3(0.0, 0.0, 0.0),
            grid_item_size: 0.1,
            grid_width: 10,
            grid_depth: 20,
        },
    );
    let mut rng = rand::thread_rng();

    let mut segments: Vec<Segment> = Vec::new();
    for i in 0..5 {
        let ground = first_segment.ground;
        let obstacle = first_segment.obstacles[0];
        segments.push(Segment::new(
            first_segment.id + i,
            vec![Obstacle {
                pos: V3(
                    obstacle.pos.0 + rng.gen_range(-0.5..0.5),
                    obstacle.pos.1,
                    obstacle.pos.2 + obstacle.pos.2 * i as f64,
                ),
                vel: obstacle.vel,
                size: obstacle.size,
            }],
            Ground {
                pos: V3(
                    ground.pos.0,
                    ground.pos.1,
                    ground.pos.2
                        + ground.grid_depth as f64
                            * ground.grid_item_size
                            * ground.pos.2.abs()
                            * i as f64,
                ),
                rot: ground.rot,
                grid_item_size: ground.grid_item_size,
                grid_width: ground.grid_width,
                grid_depth: ground.grid_depth,
            },
        ))
    }

    for segment in segments {
        game.spawn(segment);
    }

    sdl_io.run(&mut game);
    Ok(())
}
