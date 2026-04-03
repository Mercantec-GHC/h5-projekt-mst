use std::ops::Mul;

use crate::v3::V3;

pub struct M3x3([f64; 9]);

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
