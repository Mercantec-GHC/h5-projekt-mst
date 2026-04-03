use std::time::{Duration, Instant};

use sdl3::{
    Sdl,
    event::Event as SdlEvent,
    render::{Canvas, FPoint, FRect, Vertex, VertexIndices},
    video::Window as SdlWindow,
};

pub use sdl3::{keyboard::Keycode, pixels::Color};

use crate::{tri2::Tri2, v2::V2};

pub static WIDTH: f64 = 1280.0;
pub static HEIGHT: f64 = 720.0;

pub trait App<R: Renderer> {
    fn update(&mut self, delta_time: Duration);
    fn render(&self, r: &mut R);
    fn event(&mut self, event: Event);
}

pub trait Renderer {
    fn draw_rect(&mut self, pos: V2, size: V2, color: Color);
    fn draw_line(&mut self, from: V2, to: V2, color: Color);
    fn draw_triangles(&mut self, tris: &[Tri2], color: Color);
}

pub enum Event {
    KeyDown(Keycode),
    KeyUp(Keycode),
}

pub struct Window {
    sdl_context: Sdl,
    canvas: Canvas<SdlWindow>,
}

impl Window {
    pub fn new() -> Self {
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

        Self {
            sdl_context,
            canvas,
        }
    }

    pub fn run(&mut self, app: &mut impl App<Self>) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        let mut time_before = Instant::now();
        let time_per_frame = Duration::from_secs_f64(1.0 / 60.0);
        let mut print_fps_timer = Instant::now();
        let mut fps_count = 0;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    SdlEvent::Quit { .. }
                    | SdlEvent::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    SdlEvent::KeyDown {
                        keycode: Some(key), ..
                    } => app.event(Event::KeyDown(key)),
                    SdlEvent::KeyUp {
                        keycode: Some(key), ..
                    } => app.event(Event::KeyUp(key)),
                    _ => {}
                }
            }

            let time_now = Instant::now();
            let delta_time = time_now - time_before;
            time_before = time_now;
            app.update(delta_time);

            self.canvas.set_draw_color(Color::RGB(20, 20, 20));
            self.canvas.clear();
            app.render(self);
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

    fn scale_w2s(&self, v: V2) -> V2 {
        let factor = WIDTH / 2.0;
        V2(v.0 * factor, v.1 * factor)
    }
    fn translate_w2s(&self, v: V2) -> V2 {
        let aspect_ratio_h2w = HEIGHT / WIDTH;
        let middle = V2(1.0, 1.0 * aspect_ratio_h2w);
        V2(v.0 + middle.0, middle.1 - v.1)
    }
    fn point_w2s(&self, v: V2) -> V2 {
        self.scale_w2s(self.translate_w2s(v))
    }
}

impl Renderer for Window {
    fn draw_rect(&mut self, pos: V2, size: V2, color: Color) {
        let size = self.scale_w2s(size);
        let pos = self.point_w2s(pos).translate(V2(0.0, -size.0));
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(FRect::new(pos.0 as _, pos.1 as _, size.0 as _, size.1 as _))
            .unwrap();
    }
    fn draw_line(&mut self, from: V2, to: V2, color: Color) {
        let n = (to - from).normal().unit() * 0.0005;

        let p0 = from - n;
        let p1 = to - n;
        let p2 = from + n;
        let p3 = to + n;

        self.draw_triangles(&[Tri2(p0, p1, p2), Tri2(p3, p2, p1)], color);
    }

    fn draw_triangles(&mut self, tris: &[Tri2], color: Color) {
        let vertices = tris
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
}

impl From<V2> for FPoint {
    fn from(value: V2) -> Self {
        FPoint::new(value.0 as _, value.1 as _)
    }
}
