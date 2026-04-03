#![allow(unused)]

use crate::{
    scene::Scene,
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

pub struct App {}

impl<R: Renderer> window::App<R> for App {
    fn update(&mut self, delta_time: std::time::Duration) {}

    fn render(&self, r: &mut R) {
        r.draw_line(V2(0.0, 0.0), V2(0.6, 0.2), Color::RED);

        let mut scene = Scene::new();

        scene.draw_triangle(
            Tri3(V3(0.0, 0.0, 0.0), V3(0.3, 0.3, 0.0), V3(0.3, 0.0, 0.0)),
            Color::GREEN,
            Color::BLACK,
        );

        scene.render(r, V3(0.0, 0.0, -1.0));
    }

    fn event(&mut self, event: Event) {}
}

fn main() {
    let mut window = Window::new();
    let mut app = App {};

    window.run(&mut app);
}
