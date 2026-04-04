use std::ops::Mul;

use crate::v3::V3;

#[derive(Clone, Copy, Debug)]
pub struct M3x3([[f64; 3]; 3]);

impl M3x3 {
    pub fn init(v: f64) -> Self {
        Self([[v; 3]; 3])
    }

    pub fn from_v3_rot(rot: V3) -> Self {
        M3x3::new_rotate_x(rot.0) * M3x3::new_rotate_y(rot.1) * M3x3::new_rotate_z(rot.2)
    }

    #[rustfmt::skip]
    pub fn new_rotate_x(angle: f64) -> Self {
        Self([
            [1.0,             0.0,              0.0],
            [0.0, f64::cos(angle), -f64::sin(angle)],
            [0.0, f64::sin(angle),  f64::cos(angle)],
        ])
    }

    #[rustfmt::skip]
    pub fn new_rotate_y(angle: f64) -> Self {
        Self([
            [ f64::cos(angle), 0.0, f64::sin(angle)],
            [             0.0, 1.0,             0.0],
            [-f64::sin(angle), 0.0, f64::cos(angle)],
        ])
    }

    #[rustfmt::skip]
    pub fn new_rotate_z(angle: f64) -> Self {
        Self([
            [f64::cos(angle), -f64::sin(angle), 0.0],
            [f64::sin(angle),  f64::cos(angle), 0.0],
            [            0.0,              0.0, 1.0],
        ])
    }
}

impl Mul for M3x3 {
    type Output = M3x3;

    fn mul(self, rhs: M3x3) -> Self::Output {
        let mut output = M3x3([[0.0; 3]; 3]);
        for i in 0..3 {
            for j in 0..3 {
                let v = &mut output.0[i][j];
                for k in 0..3 {
                    *v += self.0[i][k] * rhs.0[k][j];
                }
            }
        }
        output
    }
}

impl Mul<V3> for M3x3 {
    type Output = V3;

    fn mul(self, rhs: V3) -> Self::Output {
        let V3(x, y, z) = rhs;
        let [[xx, xy, xz], [yx, yy, yz], [zx, zy, zz]] = self.0;
        V3(
            x * xx + y * xy + z * xz,
            x * yx + y * yy + z * yz,
            x * zx + y * zy + z * zz,
        )
    }
}
