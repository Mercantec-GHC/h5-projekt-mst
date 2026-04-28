use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use sdl3::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color as SdlColor, FColor, PixelFormat},
    rect::Point,
    render::{Canvas, FPoint, FRect, Texture, TextureCreator, Vertex, VertexIndices},
    ttf::{Font, Sdl3TtfContext},
    video::{Window, WindowContext},
    Sdl, VideoSubsystem,
};

use crate::engine::{
    error::Error,
    game,
    math::{V2, V3},
    Color, Renderer, Triangle2,
};

pub static WIDTH: f64 = 1920.0;
pub static HEIGHT: f64 = 1080.0;

pub struct SdlIo<'me> {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    ttf_context: Sdl3TtfContext,
    canvas: Canvas<Window>,
    texture_handler: TextureHandler<'me>,
}

mod textures {
    use std::collections::HashMap;

    use sdl3::{
        render::{Texture, TextureCreator},
        ttf::Font,
        video::WindowContext,
    };

    use crate::engine::Color;

    pub struct TextureHandler<'me> {
        texture_creator: TextureCreator<WindowContext>,
        textures: HashMap<u32, Texture<'me>>,
        id_counter: u32,
    }

    impl<'me> TextureHandler<'me> {
        pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
            Self {
                texture_creator,
                textures: Default::default(),
                id_counter: 0,
            }
        }
        pub fn render_font(&mut self, font: Font, text: &str, color: Color) -> u32 {
            let surface = font.render(text).blended(color).unwrap();
            let texture = unsafe {
                let static_texture_creator = &self.texture_creator as *const _;
                let texture: Texture<'me> = surface.as_texture(&*static_texture_creator).unwrap();
                texture
            };
            let id = self.id_counter;
            self.textures.insert(id, texture);
            self.id_counter += 1;
            id
        }
        pub fn get(&self, id: u32) -> &Texture<'me> {
            &self.textures[&id]
        }
        pub fn remove(&mut self, id: u32) {
            self.textures.remove(&id);
        }
    }
}

use textures::*;

impl SdlIo<'_> {
    pub fn new() -> Result<Self, Error> {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl3::ttf::init().map_err(|e| e.to_string())?;
        let window = video_subsystem
            .window("Game", WIDTH as u32, HEIGHT as u32)
            .position_centered()
            .fullscreen()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas();

        canvas.set_draw_color(SdlColor::BLACK);
        canvas.clear();
        canvas.present();

        let texture_handler = TextureHandler::new(canvas.texture_creator());

        Ok(Self {
            sdl_context,
            video_subsystem,
            ttf_context,
            canvas,
            texture_handler,
        })
    }

    pub fn run(&mut self, game: &mut impl game::Game<Self>) {
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
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => match TryInto::<game::Key>::try_into(key).ok() {
                        Some(key) => game.event(game::Event::KeyDown { key }),
                        None => {}
                    },
                    Event::KeyUp {
                        keycode: Some(key), ..
                    } => match TryInto::<game::Key>::try_into(key).ok() {
                        Some(key) => game.event(game::Event::KeyUp { key }),
                        None => {}
                    },
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

impl Renderer for SdlIo<'_> {
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

    fn draw_triangles(&mut self, triangles: &[Triangle2], color: Color) {
        let vertices = triangles
            .iter()
            .flat_map(|t| [t.0, t.1, t.2])
            .map(|p| self.point_w2s(p))
            .map(|p| Vertex {
                position: p.into(),
                color: color.into(),
                tex_coord: FPoint::new(0.0, 0.0),
            })
            .collect::<Vec<_>>();

        self.canvas
            .render_geometry(&vertices, None, VertexIndices::Sequential)
            .unwrap();
    }

    fn draw_triangle(&mut self, triangle: Triangle2, color: Color) {
        let vertices = [triangle.0, triangle.1, triangle.2]
            .into_iter()
            .map(|p| self.point_w2s(p))
            .map(|p| Vertex {
                position: p.into(),
                color: color.into(),
                tex_coord: FPoint::new(0.0, 0.0),
            })
            .collect::<Vec<_>>();
        self.canvas
            .render_geometry(&vertices, None, VertexIndices::Sequential)
            .unwrap();
    }

    fn draw_texture(&mut self, id: u32, pos: V2) {
        let texture = self.texture_handler.get(id);
        let target = FRect::new(
            pos.0 as f32,
            pos.1 as f32,
            texture.width() as f32,
            texture.height() as f32,
        );
        self.canvas.copy(&texture, None, Some(target)).unwrap();
        self.texture_handler.remove(id);
    }

    fn load_text(&mut self, text: &str, size: f64, color: Color) -> u32 {
        let font = self
            .ttf_context
            .load_font(
                "assets/BitcountGridDouble/BitcountGridDouble.ttf",
                size as f32,
            )
            .unwrap();
        let id = self.texture_handler.render_font(font, text, color);
        id
    }

    fn query_texture(&mut self, id: u32) -> V2 {
        let texture = self.texture_handler.get(id);
        V2(texture.width() as f64, texture.height() as f64)
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
            Color::Hex(v) => SdlColor::RGB(
                ((v >> 16) & 0xff) as u8,
                ((v >> 8) & 0xff) as u8,
                (v & 0xff) as u8,
            ),
            Color::White => SdlColor::WHITE,
            Color::Green => SdlColor::GREEN,
            Color::Red => SdlColor::RED,
            Color::Cyan => SdlColor::CYAN,
            Color::Black => SdlColor::BLACK,
        }
    }
}

impl From<Color> for FColor {
    fn from(value: Color) -> Self {
        FColor::from(<Color as Into<SdlColor>>::into(value))
    }
}

impl TryFrom<Keycode> for game::Key {
    type Error = ();

    fn try_from(value: Keycode) -> Result<Self, Self::Error> {
        Ok(match value {
            Keycode::Left => game::Key::Left,
            Keycode::Right => game::Key::Right,
            Keycode::Up => game::Key::Up,
            Keycode::Down => game::Key::Down,
            Keycode::W => game::Key::W,
            Keycode::A => game::Key::A,
            Keycode::S => game::Key::S,
            Keycode::D => game::Key::D,
            Keycode::LShift => game::Key::LShift,
            Keycode::LCtrl => game::Key::LCtrl,
            _ => return Err(()),
        })
    }
}
