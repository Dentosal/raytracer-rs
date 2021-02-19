use crate::prelude::*;
use crate::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: float,
    pub color: [u8; 3],
    pub emits_light: bool,
}
