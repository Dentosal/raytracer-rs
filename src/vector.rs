use crate::prelude::*;

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: float,
    pub y: float,
    pub z: float,
}

impl Vector {
    pub fn len(self) -> float {
        self.len2().sqrt()
    }

    pub fn len2(self) -> float {
        self.dot(self)
    }

    pub fn is_normalized(self) -> bool {
        approx_eq(self.len2(), 1.0)
    }

    pub fn normalized(self) -> Self {
        let len = self.len();
        assert!(len > 0.0001);
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn dot(self, other: Self) -> float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// https://math.stackexchange.com/a/13266/300156
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * (2.0 * self.dot(normal) / self.len2())
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
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
