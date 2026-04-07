use crate::v3::V3;

const I: V3 = V3(1.0, 0.0, 0.0);
const J: V3 = V3(0.0, 1.0, 0.0);
const K: V3 = V3(0.0, 0.0, 1.0);

#[derive(Copy, Clone)]
pub struct RadianQuat {
    pub rad: f64,
    pub u: V3,
}

#[derive(Copy, Clone)]
pub struct Quat {
    pub a: f64,
    pub u: V3,
}

impl Quat {
    fn len(&self) -> f64 {
        (self.a.powi(2) + self.u.0.powi(2) + self.u.1.powi(2) + self.u.2.powi(2)).sqrt()
    }
    fn as_unit(&self) -> Quat {
        let len = self.len();
        Quat {
            a: self.a / len,
            u: self.u.map(|x| x / len),
        }
    }
}

impl RadianQuat {
    pub fn as_quat(&self) -> Quat {
        let RadianQuat { rad, u } = self.as_unit();
        Quat {
            a: rad.cos(),
            u: u.map(|x| x * rad.sin()),
        }
    }
    fn as_unit(&self) -> Self {
        let len = self.u.0 + self.u.1 + self.u.2;
        let u = self.u.map(|x| x / len);

        RadianQuat { rad: self.rad, u }
    }
    pub fn as_inverse_quat(&self) -> Quat {
        let RadianQuat { rad, u } = self.as_unit();
        let u = u.map(|x| x * -1.0 * rad.sin());
        Quat { a: -rad.cos(), u }
    }
    pub fn vector_quat(&self) -> V3 {
        self.u
    }
}
