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
    pos: V3,
    size: V3,
    vel: V3,
    rot: V3,
    nyoom_factor: f64,
    pivot_deg: f64,
    pivot_deg_target: f64,
}

fn lose() {
    panic!("You lost 🫵 😂");
}

impl Skateboard {
    fn update(&mut self, delta_time: Duration) {
        self.vel.0 = self.pivot_deg * 2.0 * delta_time.as_secs_f64();
        self.pos += self.vel * delta_time.as_secs_f64();
        self.nyoom_factor += 16.0 * delta_time.as_secs_f64();
        if self.pos.0 < -0.5 || self.pos.0 > 0.5 {
            lose();
        }
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

struct GroundManager {
    ground: VecDeque<Ground>,
    start_position: V3,
    grid_item_size: f64,
    grid_width: i32,
    grid_depth: i32,
}

impl GroundManager {
    pub fn new(
        start_position: V3,
        grid_item_size: f64,
        grid_width: i32,
        grid_depth: i32,
        render_distance: i32,
    ) -> Self {
        let mut ground = VecDeque::new();
        for i in 0..render_distance {
            ground.push_back(Ground {
                grid_width,
                grid_depth,
                grid_item_size,
                pos: start_position + V3(0.0, 0.0, grid_depth as f64 * grid_item_size * i as f64),
            });
        }
        Self {
            start_position,
            grid_depth,
            grid_width,
            grid_item_size,
            ground,
        }
    }

    pub fn shuffle(&mut self) {
        let mut first = self.ground.pop_front().unwrap();
        let last = self.ground.back().unwrap();
        first.pos = last.pos + V3(0.0, 0.0, self.grid_depth as f64 * self.grid_item_size);
        self.ground.push_back(first);
    }
    pub fn should_shuffle(&self, z: f64) -> bool {
        let first = self.ground.front().unwrap();
        first.pos.2 + self.grid_depth as f64 * self.grid_item_size < z
    }
    pub fn render(&self, scene: &mut Scene) {
        for ground_part in &self.ground {
            let mut shapes = Vec::new();
            for z in 0..self.grid_depth {
                for x in -self.grid_width / 2..self.grid_width / 2 {
                    shapes.push(ShapeGroupShape {
                        shape: Shape::new_plane(V3(self.grid_item_size, 0.0, self.grid_item_size)),
                        offset: V3(
                            x as f64 * self.grid_item_size,
                            0.0,
                            z as f64 * self.grid_item_size,
                        ),
                    });
                }
            }
            let ground = ShapeGroup::new(shapes);
            ground.draw(ground_part.pos, scene, Color::Cyan, Color::Black);
        }
    }
}

struct SegmentFactory {
    segment_depth: f64,
    segment_width: f64,
}

impl SegmentFactory {
    pub fn new(segment_depth: f64, segment_width: f64) -> Self {
        Self {
            segment_depth,
            segment_width,
        }
    }

    pub fn random_obstacle_segment(&self, pos: V3) -> Segment {
        let mut rng = rand::thread_rng();
        let mut obstacles: Vec<Obstacle> = Vec::new();
        for i in 1..5 {
            obstacles.push(Obstacle {
                pos: V3(
                    rng.gen_range((-self.segment_width / 2.0)..(self.segment_width / 2.0)) - 0.1,
                    0.0,
                    self.segment_depth / i as f64,
                ),
                vel: V3(0.0, 0.0, 0.0),
                size: V3(0.1, 0.1, 0.1),
            })
        }
        Segment::new(pos, obstacles)
    }

    pub fn slalom_obstacle_segment(&self, pos: V3) -> Segment {
        let mut obstacles: Vec<Obstacle> = Vec::new();
        for i in 0..3 {
            let is_left = i % 2 == 0;
            let width = 0.5;
            let offset = if is_left { -0.5 } else { 0.5 - width };
            obstacles.push(Obstacle {
                pos: V3(offset as f64, 0.0, self.segment_depth / i as f64),
                vel: V3(0.0, 0.0, 0.0),
                size: V3(width, 0.1, 0.1),
            })
        }
        Segment::new(pos, obstacles)
    }

    pub fn new_random_segment(&self, pos: V3) -> Segment {
        use SegmentKind::*;
        let mut rng = rand::thread_rng();

        let segment_kind = &[RandomObstacles, SlalomObstacles][rng.gen_range(0..2)];

        match segment_kind {
            SegmentKind::RandomObstacles => self.random_obstacle_segment(pos),
            SegmentKind::MovingObstacles => todo!(),
            SegmentKind::SlalomObstacles => self.slalom_obstacle_segment(pos),
        }
    }
}

struct SegmentManager {
    segment_depth: f64,
    segment_width: f64,
    segment_factory: SegmentFactory,
    segments: VecDeque<Segment>,
    start_pos: V3,
}

impl SegmentManager {
    pub fn new(
        start_pos: V3,
        render_distance: i32,
        segment_depth: f64,
        segment_width: f64,
    ) -> Self {
        let mut segments: VecDeque<Segment> = VecDeque::new();
        let segment_factory = SegmentFactory {
            segment_depth,
            segment_width,
        };
        for i in 0..render_distance {
            segments.push_back(segment_factory.new_random_segment(V3(
                start_pos.0,
                start_pos.1,
                start_pos.2 + segment_depth * i as f64,
            )));
        }

        Self {
            segment_depth,
            segment_width,
            segment_factory,
            segments,
            start_pos,
        }
    }

    pub fn shuffle(&mut self) {
        let mut first = self.segments.pop_front().unwrap();
        let last = self.segments.back().unwrap();
        first.pos = last.pos + V3(0.0, 0.0, self.segment_depth as f64);
        self.segments.push_back(first);
    }
    pub fn should_shuffle(&self, z: f64) -> bool {
        let first = self.segments.front().unwrap();
        first.pos.2 + self.segment_depth < z
    }

    pub fn update(&mut self, cx: &mut UpdateCx, delta_time: Duration) {
        for segment in &mut self.segments {
            segment.update(cx, delta_time);
        }
    }

    pub fn render(&self, scene: &mut Scene) {
        for segment in &self.segments {
            segment.render(scene);
        }
    }
}

struct Game {
    skateboard: Skateboard,
    segment_manager: SegmentManager,
    camera_pos: V3,
    keys_pressed: HashSet<Key>,
    event_queue: Arc<Mutex<VecDeque<f64>>>,
    ground: GroundManager,
}

impl Game {
    fn new() -> Self {
        Self {
            skateboard: Skateboard {
                pos: V3(0.0, -0.20, 0.0),
                size: V3(0.05, 0.02, 0.15),
                vel: V3(0.0, 0.0, 0.2),
                rot: V3(0.0, PI * 0.5, 0.0),
                nyoom_factor: 0.0,
                pivot_deg: 0.0,
                pivot_deg_target: 0.0,
            },
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            camera_pos: V3(0.0, 0.0, 0.0),
            segment_manager: SegmentManager::new(V3(0.0, -0.24, -0.4), 3, 2.0, 1.0),
            keys_pressed: HashSet::new(),
            ground: GroundManager::new(V3(0.0, -0.25, -0.4), 0.1, 10, 20, 3),
        }
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
        if self.ground.should_shuffle(self.camera_pos.2) {
            self.ground.shuffle();
        }

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

        let mut cx = UpdateCx {
            skateboard: &mut self.skateboard,
        };

        self.segment_manager.update(&mut cx, delta_time);

        if self.segment_manager.should_shuffle(self.camera_pos.2) {
            self.segment_manager.shuffle();
        }
    }

    fn render(&mut self, r: &mut R) {
        let mut scene = Scene::new();
        self.ground.render(&mut scene);

        self.skateboard.render(&mut scene);
        self.segment_manager.render(&mut scene);
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
    grid_item_size: f64,
    grid_width: i32,
    grid_depth: i32,
}

enum SegmentKind {
    RandomObstacles,
    MovingObstacles,
    SlalomObstacles,
}

struct Segment {
    pos: V3,
    obstacles: Vec<Obstacle>,
}

impl Segment {
    fn new(pos: V3, obstacles: Vec<Obstacle>) -> Self {
        Self { pos, obstacles }
    }

    fn update<'game>(&mut self, cx: &mut UpdateCx<'game>, delta_time: Duration) {
        for obstacle in &mut self.obstacles {
            obstacle.pos += obstacle.vel * delta_time.as_secs_f64();
            let actual_obstacle_pos = obstacle.pos + self.pos;
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
            if skateboard_pos.0 < actual_obstacle_pos.0 + obstacle.size.0
                && skateboard_pos_and_size.0 > actual_obstacle_pos.0
                && skateboard_pos.1 < actual_obstacle_pos.2 + obstacle.size.2
                && skateboard_pos_and_size.1 > actual_obstacle_pos.2
            {
                lose();
            }
        }
    }

    fn render(&self, scene: &mut Scene) {
        for obstacle in &self.obstacles {
            scene.draw_shape(
                self.pos + obstacle.pos,
                &Shape::new_cube(obstacle.size),
                Color::Red,
                Color::Black,
            );
        }
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

    sdl_io.run(&mut game);
    Ok(())
}
