#![allow(dead_code)]

use std::{collections::HashSet, time::Duration};

use crate::engine::{Color, Key, Renderer, Scene, Shape, V3};

use crate::engine;

struct Editor {
    objects: Vec<Object>,
    next_object_id: u32,
    keys_pressed: HashSet<Key>,
    camera_pos: V3,
}

impl Editor {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
            next_object_id: 0,
            keys_pressed: HashSet::new(),
            camera_pos: V3(0.0, 0.0, -1.0),
        }
    }

    fn spawn(&mut self, object_kind: ObjectKind) {
        let object = Object {
            kind: object_kind,
            id: self.next_object_id,
        };
        self.objects.push(object);
        self.next_object_id += 1
    }

    fn despawn(&mut self, id: u32) {
        let index = self
            .objects
            .iter()
            .position(|o| o.id == id)
            .expect("doesn't exist");
        self.objects.remove(index);
    }
}

impl<R: Renderer> engine::Game<R> for Editor {
    fn update(&mut self, delta_time: Duration) {
        for object in &mut self.objects {
            object.update(delta_time);
        }
        if self.keys_pressed.contains(&Key::Left) {
            self.camera_pos.0 += delta_time.as_secs_f64() * 1.0;
        } else if self.keys_pressed.contains(&Key::Right) {
            self.camera_pos.0 -= delta_time.as_secs_f64() * 1.0;
        }
    }

    fn render(&mut self, r: &mut R) {
        let mut scene = Scene::new();
        for object in &self.objects {
            object.render(&mut scene);
        }
        scene.render(r, self.camera_pos);
    }

    fn event(&mut self, event: engine::Event) {
        match event {
            engine::Event::KeyDown { key } => self.keys_pressed.insert(key),
            engine::Event::KeyUp { key } => self.keys_pressed.remove(&key),
        };
    }
}

struct Object {
    kind: ObjectKind,
    id: u32,
}

enum ObjectKind {
    Box { pos: V3, vel: V3 },
}

struct ShapeGroupShape {
    shape: Shape,
    offset: V3,
}

struct ShapeGroup {
    pub shapes: Vec<ShapeGroupShape>,
}

impl ShapeGroup {
    pub fn new(shapes: Vec<ShapeGroupShape>) -> Self {
        Self { shapes }
    }
    pub fn rotate(mut self, rot: V3) -> Self {
        for shape in &mut self.shapes {
            shape.shape = shape
                .shape
                .translate(shape.offset)
                .rotate(rot)
                .translate(shape.offset * -1.0);
        }
        self
    }
    pub fn translate(mut self, offset: V3) -> Self {
        for shape in &mut self.shapes {
            shape.shape = shape.shape.translate(offset);
        }
        self
    }
    pub fn draw(self, pos: V3, scene: &mut Scene, outline_color: Color, fill_color: Color) {
        for shape in self.shapes {
            scene.draw_shape(
                pos,
                &shape.shape.translate(shape.offset),
                outline_color,
                fill_color,
            );
        }
    }
}

impl Object {
    fn update(&mut self, delta_time: Duration) {
        match &mut self.kind {
            ObjectKind::Box { pos, vel } => {}
        }
    }

    fn render(&self, scene: &mut Scene) {
        match self.kind {
            ObjectKind::Box { pos, .. } => {
                scene.draw_shape(
                    pos,
                    &Shape::new_cube(V3(1.0, 1.0, 1.0)),
                    Color::Green,
                    Color::Black,
                );
                scene.draw_shape(
                    pos + V3(0.0, -0.5, 0.0),
                    &Shape::new_cube(V3(0.5, 0.5, 0.5)),
                    Color::Green,
                    Color::Black,
                );
            }
        }
    }
}

pub fn editor_main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sdl_io = engine::SdlIo::new()?;
    let mut editor = Editor::new();

    let objects = [ObjectKind::Box {
        pos: V3(0.0, 0.0, 0.0),
        vel: V3(0.0, 0.0, 0.0),
    }];

    for object in objects {
        editor.spawn(object)
    }

    sdl_io.run(&mut editor);
    Ok(())
}
