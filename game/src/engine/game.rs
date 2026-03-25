use sdl3::pixels::Color;

use crate::engine::math::{Vertex, V2, V3};
use std::{marker::PhantomData, time::Duration};

use super::error::Error;

pub trait Object<R: Renderer> {
    fn update(&mut self, delta: Duration);
    fn render(&mut self, r: &mut R);
}

pub trait Io<R: Renderer, O: Object<R>> {
    fn run(&mut self, game: &mut Game<R, O>);
}

pub trait Renderer {
    fn draw_rect(&mut self, pos: V2, size: V2, color: Color);
    fn point(&mut self, pos: V2, color: Color);
    fn draw_line(&mut self, from: V2, to: V2, color: Color);
    fn draw_cube(&mut self, pos: V3, size: V3, outline_colors: Color, fill_color: Color);
    fn draw_triangle(&mut self, triangle: Vertex, color: Color);
}

pub enum Event {
    Exit,
}

pub struct Game<R: Renderer, O: Object<R>> {
    _phantom_data: PhantomData<R>,
    objects: Vec<O>,
}

impl<R: Renderer, O: Object<R>> Game<R, O> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            _phantom_data: PhantomData,
            objects: Vec::new(),
        })
    }

    pub fn spawn(&mut self, object: O) {
        self.objects.push(object);
    }

    pub fn update(&mut self, delta_time: Duration) {
        for object in self.objects.iter_mut() {
            object.update(delta_time);
        }
    }

    pub fn render(&mut self, r: &mut R) {
        for object in self.objects.iter_mut() {
            object.render(r);
        }
    }

    pub fn event(&mut self, event: Event) {}
}
