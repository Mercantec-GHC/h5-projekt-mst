use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V2(pub f64, pub f64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct V3(pub f64, pub f64, pub f64);

impl V3 {
    pub fn project_2d(&self) -> V2 {
        // V2(self.0 / self.2, self.1 / self.2)
        let c = V3(0.0, 0.0, -1.0);
        let d = *self - c;
        let e = V3(0.0, 0.0, 0.0) - c;
        V2(e.2 / d.2 * d.0 + e.0, e.2 / d.2 * d.1 + e.1)
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let V3(ax, ay, az) = self;
        let V3(bx, by, bz) = rhs;
        Self(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
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
