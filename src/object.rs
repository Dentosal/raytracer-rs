use crate::prelude::float;
use crate::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: float,
}
