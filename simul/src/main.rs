#![allow(unused)]

use obj::{Obj, load_obj};
use serde::Deserialize;
use std::{collections::HashMap, f64::consts::PI, fs::File, io::BufReader};

use crate::{
    m3x3::M3x3,
    quats::RadianQuat,
    scene::{Model, Scene},
    tri2::Tri2,
    tri3::Tri3,
    v2::V2,
    v3::V3,
    window::{Color, Event, Keycode, Renderer, Window},
};

mod m3x3;
mod quats;
mod scene;
mod tri2;
mod tri3;
mod v2;
mod v3;
mod window;

struct AssetStore {
    assets: HashMap<String, Obj>,
}

impl AssetStore {
    fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    fn load<S: Into<String>>(&mut self, path: S) -> Obj {
        let path = path.into();
        if self.assets.contains_key(&path) {
            return self.assets.get(&path).unwrap().clone();
        }
        let asset: Obj = load_obj(BufReader::new(File::open(&path).unwrap())).unwrap();
        self.assets.insert(path, asset.clone());
        asset
    }

    fn get<S: Into<String>>(&self, path: S) -> Option<Obj> {
        let path = path.into();
        self.assets.get(&path).cloned()
    }
}

pub struct App {
    assets: AssetStore,
    rot: V3,
    motion_dev: MotionDevice,
    dev_rot_rot: M3x3,
    dev_accel_rot: M3x3,
}

impl App {
    pub fn new(motion_dev: MotionDevice) -> Self {
        Self {
            assets: AssetStore::new(),
            rot: V3::init(0.0),
            motion_dev,
            dev_rot_rot: M3x3::identity(),
            dev_accel_rot: M3x3::identity(),
        }
    }
}

fn calculate_accel_axis_angle(axis: f64, tangent0: f64, tangent1: f64) -> f64 {
    (axis / (tangent0 * tangent0 + tangent1 * tangent1).sqrt()).atan()
}

impl<R: Renderer> window::App<R> for App {
    fn update(&mut self, delta_time: std::time::Duration) {
        self.rot.0 += PI * 2.0 * delta_time.as_secs_f64() * 0.05;
        // self.rot.2 += PI * 2.0 * delta_time.as_secs_f64() * 0.2;

        self.motion_dev
            .update(delta_time.as_micros() as i64, |m, dt| {
                self.dev_rot_rot *= M3x3::new_rotate_x(m.rotation().0 * dt);
                self.dev_rot_rot *= M3x3::new_rotate_y(m.rotation().1 * dt);
                self.dev_rot_rot *= M3x3::new_rotate_z(m.rotation().2 * dt);

                self.dev_accel_rot = M3x3::new_rotate_x(calculate_accel_axis_angle(
                    m.accel().0,
                    m.accel().1,
                    m.accel().2,
                ));
                self.dev_accel_rot *= M3x3::new_rotate_y(calculate_accel_axis_angle(
                    m.accel().1,
                    m.accel().0,
                    m.accel().2,
                ));
            });
    }

    fn render(&self, r: &mut R) {
        let mut scene = Scene::new();

        let mut cube = Model::new();
        let cube_obj = self.assets.get("assets/cube.obj").unwrap();
        cube.add_obj(&cube_obj, Color::RGB(165, 125, 165))
            .rotate_by_m3x3(self.dev_rot_rot)
            .scale(V3(0.4, 0.4, 0.4))
            .translate(V3(0.0, 0.0, 1.0));
        scene.draw_model(cube);

        let probe_obj = self.assets.get("assets/probe.obj").unwrap();

        let mut probe_rot = Model::new();
        probe_rot
            .add_obj(&probe_obj, Color::RGB(100, 160, 100))
            .rotate_by_m3x3(M3x3::new_rotate_x(PI * 1.5))
            .rotate_by_m3x3(M3x3::new_rotate_y(PI))
            .rotate_by_m3x3(self.dev_rot_rot)
            .scale(V3(0.15, 0.15, 0.15))
            .translate(V3(1.5, -0.8, 1.0));
        scene.draw_model(probe_rot);
        let mut probe_accel = Model::new();
        probe_accel
            .add_obj(&probe_obj, Color::RGB(160, 100, 100))
            .rotate_by_m3x3(M3x3::new_rotate_x(PI * 1.5))
            .rotate_by_m3x3(M3x3::new_rotate_y(PI))
            .rotate_by_m3x3(self.dev_accel_rot)
            .scale(V3(0.15, 0.15, 0.15))
            .translate(V3(1.0, -0.8, 1.0));
        scene.draw_model(probe_accel);

        scene.render(
            r,
            V3(0.0, 0.0, -1.0),
            M3x3::from_v3_rot(V3(0.0, 0.0, 0.0)),
            V3(0.0, 0.0, 0.0),
        );

        let mut draw_v3 = |name: &str, v: V3, pos: V2| {
            r.draw_text(
                format!("{}: [{: >7.2}, {: >7.2}, {: >7.2}]", name, v.0, v.1, v.2).as_str(),
                pos + V2(0.002, -0.002),
                Color::BLACK,
            );
            r.draw_text(
                format!("{}: [{: >7.2}, {: >7.2}, {: >7.2}]", name, v.0, v.1, v.2).as_str(),
                pos,
                Color::WHITE,
            );
        };

        let m = self.motion_dev.current();
        draw_v3("acceleration", m.accel(), V2(-0.99, -0.5));
        draw_v3("    rotation", m.rotation(), V2(-0.99, -0.53));
    }

    fn event(&mut self, event: Event) {}
}

#[derive(Debug, Deserialize)]
pub struct Measurement {
    time_delta_us: i64,
    accel_x: f64,
    accel_y: f64,
    accel_z: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
}

impl Measurement {
    pub fn accel(&self) -> V3 {
        V3(self.accel_x, self.accel_y, self.accel_z)
    }

    pub fn rotation(&self) -> V3 {
        V3(self.rotation_x, self.rotation_y, self.rotation_z)
    }
}

pub struct MotionDevice {
    measurements: Vec<Measurement>,
    idx: usize,
    timer_us: i64,
}

impl MotionDevice {
    pub fn new(measurements: Vec<Measurement>) -> Self {
        Self {
            measurements,
            idx: 0,
            timer_us: 0,
        }
    }

    pub fn update<F: FnMut(&Measurement, f64) -> ()>(&mut self, delta_time_us: i64, mut func: F) {
        self.timer_us += delta_time_us;
        while self.timer_us >= self.measurements[self.idx].time_delta_us {
            func(
                &self.measurements[self.idx],
                self.measurements[self.idx].time_delta_us as f64 / 1_000_000.0,
            );
            self.timer_us -= self.measurements[self.idx].time_delta_us;
            self.step();
        }
    }

    fn step(&mut self) {
        self.idx += 1;
        if self.idx >= self.measurements.len() {
            self.idx = 0;
        }
    }

    pub fn current(&self) -> &Measurement {
        &self.measurements[self.idx]
    }
}

fn main() {
    let mut reader = csv::Reader::from_path("assets/inputs_still.csv").unwrap();
    let mut measurements = Vec::<Measurement>::new();
    for entry in reader.deserialize::<Measurement>() {
        let entry: Measurement = entry.unwrap();
        measurements.push(entry);
    }
    let motion_dev = MotionDevice::new(measurements);

    let mut window = Window::new();
    let mut app = App::new(motion_dev);
    app.assets.load("assets/cube.obj");
    app.assets.load("assets/probe.obj");

    window.run(&mut app);
}
