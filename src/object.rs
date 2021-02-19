use crate::prelude::*;
use crate::{Color, Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: float,
    pub color: Color,
    pub emits_light: bool,
}
