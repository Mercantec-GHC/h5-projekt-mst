mod error;
mod game;
pub mod math;
mod sdl_io;
pub mod shapes;
mod system;

pub use game::{Event, Game, Io, Object, Renderer};
pub use math::V3;
pub use sdl_io::SdlIo;
