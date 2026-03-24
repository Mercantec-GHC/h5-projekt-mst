use std::time::Duration;

pub trait Object {
    fn update(&mut self, delta: Duration);
    fn render(&mut self);
}


