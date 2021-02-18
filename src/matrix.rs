use crate::angle::Angle;
use crate::prelude::float;
use crate::vector::{Point, Vector};

use std::fmt;
use std::ops::Mul;

/// 4x4 transformation matrix, outer array is rows, inner columns
#[derive(Debug, Clone, Copy)]
pub struct Matrix([[float; 4]; 4]);

impl Matrix {
    /// https://en.wikipedia.org/wiki/Transformation_matrix#Rotation_2
    pub fn rotation(about: Vector, angle: Angle) -> Self {
        assert!(about.is_normalized());

        let v = about;
        let s = angle.radians.sin();
        let c = angle.radians.cos();
        let a = 1.0 - c;

        Self([
            [
                v.x * v.x * a + c,
                v.x * v.y * a - v.z * s,
                v.x * v.z * a + v.y * s,
                0.0,
            ],
            [
                v.y * v.x * a + v.z * s,
                v.y * v.y * a + c,
                v.y * v.z * a - v.x * s,
                0.0,
            ],
            [
                v.z * v.x * a - v.y * s,
                v.z * v.y * a + v.x * s,
                v.z * v.z * a + c,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// https://en.wikipedia.org/wiki/Transformation_matrix#Rotation_2
    pub fn scale(scale: Vector) -> Self {
        Self([
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(translate: Vector) -> Self {
        Self([
            [1.0, 0.0, 0.0, translate.x],
            [0.0, 1.0, 0.0, translate.y],
            [0.0, 0.0, 1.0, translate.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Position (= translation) component
    pub fn pos(self) -> Point {
        Point {
            x: self.0[0][3],
            y: self.0[1][3],
            z: self.0[2][3],
        }
    }

    /// Direction (= rotation) vector
    pub fn dir(self) -> Vector {
        self.mul_rotate(Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        })
    }

    /// Uses source (x, y, z, 1.0)
    pub fn mul_translate(self, rhs: Vector) -> Vector {
        Vector {
            x: self.0[0][0] * rhs.x + self.0[0][1] * rhs.y + self.0[0][2] * rhs.z + self.0[0][3],
            y: self.0[1][0] * rhs.x + self.0[1][1] * rhs.y + self.0[1][2] * rhs.z + self.0[1][3],
            z: self.0[2][0] * rhs.x + self.0[2][1] * rhs.y + self.0[2][2] * rhs.z + self.0[2][3],
        }
    }

    /// Uses source (x, y, z, 0.0)
    pub fn mul_rotate(self, rhs: Vector) -> Vector {
        Vector {
            x: self.0[0][0] * rhs.x + self.0[0][1] * rhs.y + self.0[0][2] * rhs.z,
            y: self.0[1][0] * rhs.x + self.0[1][1] * rhs.y + self.0[1][2] * rhs.z,
            z: self.0[2][0] * rhs.x + self.0[2][1] * rhs.y + self.0[2][2] * rhs.z,
        }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = [[0.0; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                let mut s = 0.0;
                for i in 0..4 {
                    s += self.0[row][i] * rhs.0[i][col];
                }
                result[row][col] = s;
            }
        }
        Self(result)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for row in 0..4 {
            if row > 0 {
                write!(f, " ")?;
            }
            write!(f, "[")?;
            for col in 0..4 {
                write!(f, "{:+.2}  ", self.0[row][col])?;
            }
            write!(f, "]")?;
            if row < 3 {
                write!(f, "\n")?;
            }
        }
        write!(f, "]\n")?;
        Ok(())
    }
}
