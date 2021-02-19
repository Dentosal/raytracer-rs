use crate::prelude::*;
use crate::{Color, Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub material: Material,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere { center: Point, radius: float },
    Triangle { corners: [Point; 3] },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub emits_light: bool,
}
