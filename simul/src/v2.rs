use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct V2(pub f64, pub f64);

impl V2 {
    pub fn init(v: f64) -> Self {
        Self(v, v)
    }

    pub fn map<F: Fn(f64) -> f64>(&self, func: F) -> Self {
        Self(func(self.0), func(self.1))
    }

    pub fn zip<F: Fn(f64, f64) -> f64>(&self, rhs: Self, func: F) -> Self {
        Self(func(self.0, rhs.0), func(self.1, rhs.1))
    }

    pub fn reduce<F: Fn(f64, f64) -> f64>(&self, initial: f64, func: F) -> f64 {
        let acc = initial;
        let acc = func(acc, self.0);
        func(acc, self.1)
    }

    pub fn normal(&self) -> V2 {
        V2(-self.1, self.0)
    }

    pub fn translate(&self, trans: V2) -> V2 {
        *self + trans
    }

    pub fn len(&self) -> f64 {
        self.map(|v| v.powi(2)).reduce(0.0, |acc, v| acc + v)
    }

    pub fn unit(&self) -> V2 {
        *self / self.len()
    }
}

impl Add for V2 {
    type Output = V2;

    fn add(self, rhs: V2) -> Self::Output {
        self.zip(rhs, |a, b| a + b)
    }
}

impl Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: V2) -> Self::Output {
        self.zip(rhs, |a, b| a - b)
    }
}

impl Mul<f64> for V2 {
    type Output = V2;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|v| v * rhs)
    }
}

impl Div<f64> for V2 {
    type Output = V2;

    fn div(self, rhs: f64) -> Self::Output {
        self.map(|v| v / rhs)
    }
}

impl Neg for V2 {
    type Output = V2;

    fn neg(self) -> Self::Output {
        V2::init(0.0) - self
    }
}
