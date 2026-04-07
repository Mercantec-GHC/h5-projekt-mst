use crate::{m3x3::M3x3, quats::RadianQuat, tri2::Tri2, v3::V3};

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

    pub fn rotate_by_v3(&self, rot: V3) -> Self {
        self.map(|v| v.rotate_by_v3(rot))
    }
    pub fn rotate_by_m3x3(&self, rot: M3x3) -> Self {
        self.map(|v| v.rotate_by_m3x3(rot))
    }
    pub fn rotate_by_quat(&self, rot: RadianQuat) -> Self {
        self.map(|v| v.rotate_by_quat(rot))
    }

    pub fn project_2d(&self, camera_pos: V3, camera_rot: M3x3, screen_rel_pos: V3) -> Tri2 {
        Tri2(
            self.0.project_2d(camera_pos, camera_rot, screen_rel_pos),
            self.1.project_2d(camera_pos, camera_rot, screen_rel_pos),
            self.2.project_2d(camera_pos, camera_rot, screen_rel_pos),
        )
    }

    pub fn translate(&self, offset: V3) -> Self {
        self.map(|v| v.translate(offset))
    }

    pub fn scale(&self, scale: V3) -> Self {
        self.map(|v| v.scale(scale))
    }

    pub fn points(&self) -> [V3; 3] {
        [self.0, self.1, self.2]
    }

    pub fn normal(&self) -> V3 {
        (self.1 - self.0).cross(self.2 - self.1)
    }

    pub fn middle(&self) -> V3 {
        (self.0 + self.1 + self.2).map(|v| v / 3.0)
    }
}
