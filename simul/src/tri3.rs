use crate::{tri2::Tri2, v3::V3};

#[derive(Clone, Copy, Debug)]
pub struct Tri3(pub V3, pub V3, pub V3);

impl Tri3 {
    pub fn init(v: V3) -> Self {
        Self(v, v, v)
    }

    pub fn map<F: Fn(V3) -> V3>(&self, func: F) -> Self {
        Self(func(self.0), func(self.1), func(self.2))
    }

    pub fn zip<F: Fn(V3, V3) -> V3>(&self, rhs: Tri3, func: F) -> Self {
        Self(
            func(self.0, rhs.0),
            func(self.1, rhs.1),
            func(self.2, rhs.2),
        )
    }

    pub fn rotate(&self, rot: V3) -> Self {
        self.map(|v| v.rotate(rot))
    }

    pub fn project_2d(&self, camera_pos: V3) -> Tri2 {
        Tri2(
            self.0.project_2d(camera_pos),
            self.1.project_2d(camera_pos),
            self.2.project_2d(camera_pos),
        )
    }

    pub fn translate(&self, offset: V3) -> Self {
        self.map(|v| v + offset)
    }

    pub fn points(&self) -> [V3; 3] {
        [self.0, self.1, self.2]
    }

    pub fn normal(&self) -> V3 {
        (self.1 - self.0).cross(self.2 - self.1)
    }
}
