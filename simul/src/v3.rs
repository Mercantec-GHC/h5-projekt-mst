use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{m3x3::M3x3, v2::V2};

#[derive(Clone, Copy, Debug)]
pub struct V3(pub f64, pub f64, pub f64);

impl V3 {
    pub fn init(v: f64) -> Self {
        Self(v, v, v)
    }

    pub fn map<F: Fn(f64) -> f64>(&self, func: F) -> Self {
        Self(func(self.0), func(self.1), func(self.2))
    }

    pub fn zip<F: Fn(f64, f64) -> f64>(&self, rhs: V3, func: F) -> Self {
        Self(
            func(self.0, rhs.0),
            func(self.1, rhs.1),
            func(self.2, rhs.2),
        )
    }

    pub fn reduce<F: Fn(f64, f64) -> f64>(&self, initial: f64, func: F) -> f64 {
        let acc = initial;
        let acc = func(acc, self.0);
        let acc = func(acc, self.1);
        func(acc, self.0)
    }

    pub fn cross(&self, rhs: V3) -> Self {
        let V3(ax, ay, az) = self;
        let V3(bx, by, bz) = rhs;
        V3(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
    }

    pub fn rotate_by_v3(&self, rot: V3) -> Self {
        M3x3::new_rotate_z(rot.2)
            * (M3x3::new_rotate_y(rot.1) * (M3x3::new_rotate_x(rot.0) * *self))
    }

    pub fn rotate_by_m3x3(&self, rot: M3x3) -> Self {
        rot * *self
    }

    /// See https://en.wikipedia.org/wiki/3D_projection#Mathematical_formula
    /// for details on the implementation.
    pub fn project_2d(&self, camera_pos: V3, camera_rot: M3x3, screen_rel_pos: V3) -> V2 {
        let a = *self;
        let c = camera_pos;
        let d = (a - c).rotate_by_m3x3(camera_rot);
        let e = screen_rel_pos - c;
        V2(e.2 / d.2 * d.0 + e.0, e.2 / d.2 * d.1 + e.1)
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.zip(rhs, |a, b| a * b).reduce(0.0, |acc, v| acc + v)
    }

    pub fn translate(&self, offset: Self) -> Self {
        *self + offset
    }

    pub fn scale(&self, scale: Self) -> Self {
        self.zip(scale, |a, b| a * b)
    }

    pub fn len(&self) -> f64 {
        self.map(|v| v.powi(2)).reduce(0.0, |acc, v| acc + v)
    }

    pub fn unit(&self) -> V3 {
        *self / self.len()
    }
}

impl Add for V3 {
    type Output = V3;

    fn add(self, rhs: V3) -> Self::Output {
        self.zip(rhs, |a, b| a + b)
    }
}

impl Sub for V3 {
    type Output = V3;

    fn sub(self, rhs: V3) -> Self::Output {
        self.zip(rhs, |a, b| a - b)
    }
}

impl Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|v| v * rhs)
    }
}

impl Div<f64> for V3 {
    type Output = V3;

    fn div(self, rhs: f64) -> Self::Output {
        self.map(|v| v / rhs)
    }
}

impl Neg for V3 {
    type Output = V3;

    fn neg(self) -> Self::Output {
        V3::init(0.0) - self
    }
}
