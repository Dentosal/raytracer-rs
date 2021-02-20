use crate::prelude::*;
use crate::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub shape: Shape,
    pub material_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere { center: Point, radius: float },
    Triangle { corners: [Point; 3] },
}
