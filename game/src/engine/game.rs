

use std::time::Instant;

use sdl3::render::Canvas;
use sdl3::{EventPump, Sdl, VideoSubsystem};
use sdl3::video::Window;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

use super::error::Error;
use super::object::Object;

pub struct Game<Io, O: Object> {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    io: Io,
    objects: Vec<O>,
}

impl<Io, O: Object> Game<Io, O> {
    pub fn new(io: Io) -> Result<Self, Error> {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Game", 1280, 720).position_centered().build().unwrap();
        let mut canvas = window.into_canvas();
        
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();
        
        let event_pump = sdl_context.event_pump().unwrap();

        Ok(Self {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            io,
            objects: Vec::new()
        })
    }

    pub fn run(&mut self) {
        let mut time_before = Instant::now();
        let time_per_frame = 1_000_000_000 / 60;
        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
            let now = Instant::now();
            let delta = now - time_before;
            time_before = now;
            for object in self.objects.iter_mut() {
                object.update(delta);
            }

        }
    }
}