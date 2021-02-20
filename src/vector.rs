use crate::prelude::*;

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: float,
    pub y: float,
    pub z: float,
}

impl Vector {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

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

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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


#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use super::Vector;

    #[test]
    fn cross_product_simple_0() {
        let a = Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let b = Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let c = a.cross(b);

        assert!(approx_eq(c.x, 0.0));
        assert!(approx_eq(c.y, 0.0));
        assert!(approx_eq(c.z, 1.0));
    }

    #[test]
    fn cross_product_simple_1() {
        let a = Vector { x: 0.0, y: 0.0, z: 0.5 };
        let b = Vector { x: 0.0, y: 0.5, z: 0.0 };

        let c = a.cross(b).normalized();
        let d = b.cross(a).normalized();

        assert!(approx_eq(c.x, -1.0));
        assert!(approx_eq(c.y, 0.0));
        assert!(approx_eq(c.z, 0.0));

        assert!(approx_eq(d.x, 1.0));
        assert!(approx_eq(d.y, 0.0));
        assert!(approx_eq(d.z, 0.0));
    }
}
