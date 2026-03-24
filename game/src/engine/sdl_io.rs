use std::time::Instant;

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormat},
    render::{Canvas, FRect},
    video::Window,
    Sdl, VideoSubsystem,
};

use crate::engine::{
    error::Error,
    game,
    math::{V2, V3},
    Object,
};

static WIDTH: f64 = 1280.0;
static HEIGHT: f64 = 720.0;

pub struct SdlIo {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
}

impl SdlIo {
    pub fn new() -> Result<Self, Error> {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Game", WIDTH as u32, HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Ok(Self {
            sdl_context,
            video_subsystem,
            canvas,
        })
    }

    pub fn run<O: game::Object<Self>>(&mut self, game: &mut game::Game<Self, O>) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut time_before = Instant::now();
        let time_per_frame = 1_000_000_000 / 60;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            let time_now = Instant::now();
            let delta_time = time_now - time_before;
            time_before = time_now;
            game.update(delta_time);

            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();
            game.render(self);
            self.canvas.present();
        }
    }

    fn world_to_screen(&self, v: V2) -> V2 {
        V2(v.0 + WIDTH / 2.0, HEIGHT / 2.0 - v.1)
    }
}

impl game::Renderer for SdlIo {
    fn draw_rect(&mut self, pos: V2, size: V2, color: Color) {
        let pos = self.world_to_screen(pos);
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(FRect::new(
                (pos.0 - size.0 / 2.0) as _,
                (pos.1 - size.1 / 2.0) as _,
                size.0 as _,
                size.1 as _,
            ))
            .unwrap();
    }

    fn point(&mut self, pos: V2, color: Color) {
        let pos = self.world_to_screen(pos);
        let size = V2(10.0, 10.0);
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(FRect::new(
                (pos.0 - size.0 / 2.0) as _,
                (pos.1 - size.1 / 2.0) as _,
                10.0,
                10.0,
            ))
            .unwrap();
    }

    fn draw_line(&mut self, from: V2, to: V2, color: Color) {
        todo!()
    }

    fn draw_cube(&mut self, pos: V3, size: V3, outline_color: Color, fill_color: Color) {
        let V3(x, y, z) = pos;
        let V3(w, h, d) = size;
        let points = [
            pos,
            V3(x + w, y, z),
            V3(x, y + h, z),
            V3(x + w, y + h, z),
            V3(x, y, z + d / 100.0),
            V3(x + w, y, z + d / 100.0),
            V3(x, y + h, z + d / 100.0),
            V3(x + w, y + h, z + d / 100.0),
        ];
        for point in points {
            self.point(point.as_2d(), outline_color);
        }
    }
}
