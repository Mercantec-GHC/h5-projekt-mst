#![allow(unused)]

use std::{collections::HashMap, f64::consts::PI, fs::File, io::BufReader};

use obj::{Obj, load_obj};

use crate::{
    m3x3::M3x3,
    scene::{Model, Scene},
    tri2::Tri2,
    tri3::Tri3,
    v2::V2,
    v3::V3,
    window::{Color, Event, Keycode, Renderer, Window},
};

mod m3x3;
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
}

impl App {
    pub fn new() -> Self {
        Self {
            assets: AssetStore::new(),
            rot: V3::init(0.0),
        }
    }
}

impl<R: Renderer> window::App<R> for App {
    fn update(&mut self, delta_time: std::time::Duration) {
        self.rot.0 += PI * 2.0 * delta_time.as_secs_f64() * 0.1;
        self.rot.1 += PI * 2.0 * delta_time.as_secs_f64() * 0.1;
    }

    fn render(&self, r: &mut R) {
        let mut scene = Scene::new();

        // let mut model1 = Model::new();
        // model1
        //     .add_tri(
        //         Tri3(V3(0.0, 0.0, 0.0), V3(0.3, 0.3, 0.0), V3(0.3, 0.0, 0.0)),
        //         Color::RGB(200, 200, 200),
        //     )
        //     .rotate_by_m3x3(M3x3::from_v3_rot(self.rot))
        //     .translate(V3(1.0, 0.0, 2.0))
        //     .scale(V3(1.0, 1.0, 1.0));
        //
        // scene.draw_model(model1);

        let mut model2 = Model::new();

        let cube_obj = self.assets.get("assets/cube.obj").unwrap();
        model2
            .add_obj(&cube_obj, Color::RGB(165, 125, 165))
            .rotate_by_m3x3(M3x3::from_v3_rot(self.rot))
            .scale(V3(0.2, 0.2, 0.2))
            .translate(V3(0.0, 0.0, 1.0));

        scene.draw_model(model2);

        // scene.draw_obj(&cube_obj, V3(0.1, 0.1, 0.1), Color::RGB(125, 125, 125));

        scene.render(
            r,
            V3(0.0, 0.0, 0.0),
            M3x3::from_v3_rot(V3(0.0, 0.0, 0.0)),
            V3(0.0, 0.0, 1.0),
        );

        // r.draw_line(V2(0.0, 0.0), V2(0.6, 0.2), Color::RED);
    }

    fn event(&mut self, event: Event) {}
}

fn main() {
    let mut window = Window::new();
    let mut app = App::new();
    app.assets.load("assets/cube.obj");

    window.run(&mut app);
}
