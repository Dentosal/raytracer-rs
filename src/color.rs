use crate::prelude::*;

use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: float,
    pub g: float,
    pub b: float,
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };

    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
    };

    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
    };

    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
    };

    #[must_use]
    pub fn darken(self, ratio: float) -> Self {
        assert!(
            ratio >= 0.0 && ratio <= 1.001,
            "Ratio out of range: {}",
            ratio
        );

        let ratio = ratio.clamp(0.0, 1.0);

        Self {
            r: self.r * ratio,
            g: self.g * ratio,
            b: self.b * ratio,
        }
    }

    #[must_use]
    pub fn mix(self, other: Self, ratio: float) -> Self {
        assert!(
            ratio >= 0.0 && ratio <= 1.001,
            "Ratio out of range: {}",
            ratio
        );

        let ratio = ratio.clamp(0.0, 1.0);

        Self {
            r: self.r * (1.0 - ratio) + other.r * ratio,
            g: self.g * (1.0 - ratio) + other.g * ratio,
            b: self.b * (1.0 - ratio) + other.b * ratio,
        }
    }

    pub fn to_pixel_color(self) -> [u8; 4] {
        [
            (self.r.clamp(0.0, 1.0) * (0xff as float)) as u8,
            (self.g.clamp(0.0, 1.0) * (0xff as float)) as u8,
            (self.b.clamp(0.0, 1.0) * (0xff as float)) as u8,
            0xff,
        ]
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: (self.r * rhs.r),
            g: (self.g * rhs.g),
            b: (self.b * rhs.b),
        }
    }
}

impl Div<float> for Color {
    type Output = Self;

    fn div(self, rhs: float) -> Self {
        Self {
            r: (self.r / rhs),
            g: (self.g / rhs),
            b: (self.b / rhs),
        }
    }
}

impl From<[f32; 3]> for Color {
    fn from(color: [f32; 3]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }
}
