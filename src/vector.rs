use crate::prelude::*;

use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: float,
    pub y: float,
    pub z: float,
}

impl Vector {
    pub fn len(self) -> float {
        self.dot(self)
    }

    pub fn dot(self, other: Self) -> float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<float> for Vector {
    type Output = Self;

    fn mul(self, rhs: float) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

pub type Point = Vector;
