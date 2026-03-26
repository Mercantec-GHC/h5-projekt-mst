use std::time::{Duration, Instant};

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color as SdlColor, PixelFormat},
    rect::Point,
    render::{Canvas, FPoint, FRect},
    video::Window,
    Sdl, VideoSubsystem,
};

use crate::engine::{
    error::Error,
    game,
    math::{V2, V3},
    Color, Renderer,
};

pub static WIDTH: f64 = 1280.0;
pub static HEIGHT: f64 = 720.0;

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

        canvas.set_draw_color(SdlColor::BLACK);
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
        let time_per_frame = Duration::from_secs_f64(1.0 / 60.0);
        let mut print_fps_timer = Instant::now();
        let mut fps_count = 0;

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

            self.canvas.set_draw_color(SdlColor::BLACK);
            self.canvas.clear();
            game.render(self);
            self.canvas.present();

            fps_count += 1;
            if (time_now - print_fps_timer).as_secs_f64() > 1.0 {
                println!("fps: {fps_count}");
                fps_count = 0;
                print_fps_timer = time_now;
            }
            if delta_time < time_per_frame {
                std::thread::sleep(time_per_frame - delta_time);
            }
        }
    }

    /// world space to screen space
    fn scale_w2s(&self, v: V2) -> V2 {
        let factor = WIDTH / 2.0;
        V2(v.0 * factor, v.1 * factor)
    }
    /// world space to screen space.
    fn translate_w2s(&self, v: V2) -> V2 {
        let middle = V2(WIDTH / 2.0, HEIGHT / 2.0);
        V2(v.0 + middle.0, middle.1 - v.1)
    }

    /// scale -> translate
    fn point_w2s(&self, v: V2) -> V2 {
        self.translate_w2s(self.scale_w2s(v))
    }
}

impl Renderer for SdlIo {
    fn draw_rect(&mut self, pos: V2, size: V2, color: Color) {
        let pos = self.point_w2s(pos);
        let size = self.scale_w2s(size);
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(FRect::new(pos.0 as _, pos.1 as _, size.0 as _, size.1 as _))
            .unwrap();
    }

    fn draw_point(&mut self, pos: V2, color: Color) {
        let pos = self.point_w2s(pos);
        let size = 4.0;
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(FRect::new(
                (pos.0 - size / 2.0) as _,
                (pos.1 - size / 2.0) as _,
                size as _,
                size as _,
            ))
            .unwrap();
    }

    fn draw_line(&mut self, from: V2, to: V2, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas
            .draw_line(self.point_w2s(from), self.point_w2s(to))
            .unwrap();
    }
}

impl From<V2> for FPoint {
    fn from(value: V2) -> Self {
        FPoint::new(value.0 as _, value.1 as _)
    }
}

impl From<Color> for SdlColor {
    fn from(value: Color) -> Self {
        match value {
            Color::HEX(v) => SdlColor::from_u32(&PixelFormat::BGR24, v),
            Color::WHITE => SdlColor::WHITE,
            Color::GREEN => SdlColor::GREEN,
            Color::RED => SdlColor::RED,
            Color::CYAN => SdlColor::CYAN,
        }
    }
}
