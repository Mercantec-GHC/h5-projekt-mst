use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V2(pub f64, pub f64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V3(pub f64, pub f64, pub f64);

impl V3 {
    pub fn filled(v: f64) -> Self {
        Self(v, v, v)
    }

    /// See https://en.wikipedia.org/wiki/3D_projection#Mathematical_formula
    /// for details on the implementation.
    pub fn project_2d(&self, camera_pos: V3) -> V2 {
        let a = *self;
        let c = camera_pos;
        let d = a - c;
        let e = (camera_pos + V3(0.0, 0.0, 1.0)) - c;
        V2(e.2 / d.2 * d.0 + e.0, e.2 / d.2 * d.1 + e.1)
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let V3(ax, ay, az) = self;
        let V3(bx, by, bz) = rhs;
        Self(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        let V3(ax, ay, az) = self;
        let V3(bx, by, bz) = rhs;
        ax * bx + ay * by + az * bz
    }

    pub fn len(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Self {
        self.map(|v| v / self.len())
    }

    pub fn map<F: Fn(f64) -> f64>(&self, func: F) -> Self {
        V3(func(self.0), func(self.1), func(self.2))
    }

    pub fn rotate(&self, rot: Self) -> Self {
        M3x3::new_rotate_z(rot.2)
            * (M3x3::new_rotate_y(rot.1) * (M3x3::new_rotate_x(rot.0) * *self))
    }

    pub fn distance(&self, rhs: Self) -> f64 {
        (*self - rhs).len()
    }

    pub fn angle(&self, rhs: Self) -> f64 {
        (self.dot(rhs) / (self.len() * rhs.len())).acos()
    }
}

impl Add for V3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Sub for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Self> for V3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Add<&Self> for V3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Sub<&Self> for V3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for V3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign<f64> for V3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Triangle2(pub V2, pub V2, pub V2);

#[derive(Clone, PartialEq, Debug)]
pub struct Triangle3(pub V3, pub V3, pub V3);

impl Triangle3 {
    pub fn map<F: Fn(V3) -> V3>(&self, f: F) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }

    pub fn normal(&self) -> V3 {
        (self.1 - self.0).cross(self.2 - self.1)
    }

    pub fn translate(&self, offset: V3) -> Self {
        Self(self.0 + offset, self.1 + offset, self.2 + offset)
    }

    pub fn rotate(&self, rot: V3) -> Self {
        self.map(|v| v.rotate(rot))
    }

    pub fn project_2d(&self, camera_pos: V3) -> Triangle2 {
        Triangle2(
            self.0.project_2d(camera_pos),
            self.1.project_2d(camera_pos),
            self.2.project_2d(camera_pos),
        )
    }

    pub fn middle(&self) -> V3 {
        (self.0 + self.1 + self.2).map(|v| v / 3.0)
    }

    pub fn points(&self) -> [V3; 3] {
        [self.0, self.1, self.2]
    }
}

struct M3x3([f64; 9]);

impl M3x3 {
    #[rustfmt::skip]
    pub fn new_rotate_x(angle: f64) -> Self {
        Self([
            1.0,             0.0,              0.0,
            0.0, f64::cos(angle), -f64::sin(angle),
            0.0, f64::sin(angle),  f64::cos(angle),
        ])
    }

    #[rustfmt::skip]
    pub fn new_rotate_y(angle: f64) -> Self {
        Self([
             f64::cos(angle), 0.0, f64::sin(angle),
                         0.0, 1.0,             0.0,
            -f64::sin(angle), 0.0, f64::cos(angle),
        ])
    }

    #[rustfmt::skip]
    pub fn new_rotate_z(angle: f64) -> Self {
        Self([
            f64::cos(angle), -f64::sin(angle), 0.0,
            f64::sin(angle),  f64::cos(angle), 0.0,
                        0.0,              0.0, 1.0,
        ])
    }
}

impl Mul<V3> for M3x3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> Self::Output {
        let V3(x, y, z) = rhs;
        let [xx, xy, xz, yx, yy, yz, zx, zy, zz] = self.0;
        V3(
            x * xx + y * xy + z * xz,
            x * yx + y * yy + z * yz,
            x * zx + y * zy + z * zz,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn v3_ops() {
        let v3 = |v: f64| V3(v, v, v);

        assert_eq!(v3(1.0) + v3(2.0), v3(3.0));
        assert_eq!(v3(3.0) - v3(2.0), v3(1.0));
        assert_eq!(v3(2.0) * 2.0, v3(4.0));

        let mut a = v3(1.0);
        a += v3(2.0);
        assert_eq!(a, v3(3.0));

        let mut a = v3(2.0);
        a *= 2.0;
        assert_eq!(a, v3(4.0));
    }
}
