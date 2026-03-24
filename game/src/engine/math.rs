use std::ops::{Add, AddAssign, Mul, MulAssign};

pub struct V2(pub f64, pub f64);

#[derive(Copy, Clone)]
pub struct V3(pub f64, pub f64, pub f64);

impl Add for V3 {
    type Output = V3;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<f64> for V3 {
    type Output = V3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign<f64> for V3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl V3 {
    pub fn as_2d(&self) -> V2 {
        V2(self.0 / self.2, self.1 / self.2)
    }
}

pub struct Vertex(V3, V3, V3);

pub struct Cube {
    vertices: [Vertex; 12],
}

impl Cube {
    pub fn new(V3(x, y, z): V3, V3(w, h, d): V3) -> Self {
        Self {
            vertices: [
                Vertex(V3(x, y, z), V3(x, y + h, z), V3(x + w, y + h, z)),
                Vertex(V3(x + w, y + h, z), V3(x + w, y, z), V3(x, y + h, z)),
                Vertex(V3(x + w, y, z), V3(x + w, y + h, z), V3(x + w, y, z + d)),
                Vertex(
                    V3(x + w, y + h, z + d),
                    V3(x + w, y, z + d),
                    V3(x, y + h, z + d),
                ),
                Vertex(V3(x, y + h, z), V3(x + w, y + h, z), V3(x, y + h, z + d)),
                Vertex(
                    V3(x + w, y + h, z),
                    V3(x + w, y + h, z + d),
                    V3(x, y + h, z + d),
                ),
                Vertex(V3(x, y, z), V3(x, y + h, z), V3(x, y, z + d)),
                Vertex(V3(x, y + h, z), V3(x, y + h, z + d), V3(x, y, z + d)),
                Vertex(V3(x, y, z), V3(x + w, y, z), V3(x, y, z + d)),
                Vertex(V3(x + w, y, z), V3(x + w, y, z + d), V3(x, y, z + d)),
                Vertex(V3(x, y, z + d), V3(x + w, y, z + d), V3(x, y + h, z + d)),
                Vertex(
                    V3(x + w, y, z + d),
                    V3(x + w, y + h, z + d),
                    V3(x, y + h, z + d),
                ),
            ],
        }
    }
}
