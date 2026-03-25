use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V2(pub f64, pub f64);

#[derive(Clone, Copy, PartialEq, Debug)]
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
    pub fn project(&self) -> V2 {
        V2(self.0 / self.2, self.1 / self.2)
    }

    pub fn delta(&self, v: V3) -> V3 {
        V3(v.0 - self.0, v.1 - self.1, v.2 - self.2)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Vertex(pub V3, pub V3, pub V3);

impl Vertex {
    pub fn normal_vector(&self) -> V3 {
        let vector_a = self.1.delta(self.0);
        let vector_b = self.1.delta(self.2);

        V3(
            vector_a.1 * vector_b.2 - vector_a.2 * vector_b.1,
            vector_a.2 * vector_b.0 - vector_a.0 * vector_b.2,
            vector_a.0 * vector_b.1 - vector_a.1 * vector_b.0,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn v3_ops() {
        let v2 = |v: f64| V2(v, v);
        let v3 = |v: f64| V3(v, v, v);

        assert_eq!(v3(1.0) + v3(2.0), v3(3.0));

        let mut a = v3(1.0);
        a += v3(2.0);
        assert_eq!(a, v3(3.0));

        assert_eq!(v3(2.0) * 2.0, v3(4.0));

        let mut a = v3(2.0);
        a *= 2.0;
        assert_eq!(a, v3(4.0));
    }
}
