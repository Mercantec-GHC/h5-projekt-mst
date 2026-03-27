use crate::engine::{
    math::{V2, V3},
    Triangle2,
};
use std::{marker::PhantomData, time::Duration};

use super::error::Error;

pub trait Io<R: Renderer, G: Game<R>> {
    fn run(&mut self, game: &mut G);
}

pub trait Renderer {
    fn draw_rect(&mut self, pos: V2, size: V2, color: Color);
    fn draw_point(&mut self, pos: V2, color: Color);
    fn draw_line(&mut self, from: V2, to: V2, color: Color);
    fn draw_triangle(&mut self, triangle: Triangle2, color: Color);
    fn draw_triangles(&mut self, triangles: &[Triangle2], color: Color);
}

#[derive(Clone, Copy)]
pub enum Color {
    Hex(u32),
    White,
    Green,
    Red,
    Cyan,
    Black,
}

pub enum Event {
    Exit,
}

pub trait Game<R: Renderer> {
    fn update(&mut self, delta_time: Duration);
    fn render(&mut self, r: &mut R);
    fn event(&mut self, event: Event);
}
