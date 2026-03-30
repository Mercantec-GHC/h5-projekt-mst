use std::{collections::VecDeque, sync::Mutex};

pub enum Event {
    Skateboard {
        x: f64,
        y: f64,
        z: f64,
        a: f64,
        b: f64,
        c: f64,
    },
}
pub struct EventQueue {
    queue: VecDeque<Event>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn poll(&mut self) -> Option<Event> {
        self.queue.pop_front()
    }

    pub fn push(&mut self, event: Event) {
        self.queue.push_back(event);
    }
}
