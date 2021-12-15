use crate::object::{Object, Shape};
use crate::prelude::*;
use crate::vector::{Point, Vector};

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    /// Index
    pub object: usize,
    pub distance: float,
    pub normal: Vector,
}

pub fn raycast(from: Point, direction: Vector, objects: &[Object]) -> Option<RayHit> {
    let direction = direction.normalized();

    let mut closest: Option<RayHit> = None;

    for (i, object) in objects.iter().enumerate() {
        let opt_hit = match object.shape {
            Shape::Sphere { center, radius } => ray_sphere(from, direction, center, radius),
            Shape::Triangle { corners } => ray_triange(from, direction, corners),
        };

        if let Some(mut hit) = opt_hit {
            hit.object = i; // Fill in the object
            if let Some(old) = closest {
                if old.distance > hit.distance {
                    closest = Some(hit);
                }
            } else {
                closest = Some(hit);
            }
        }
    }

    closest
}

/// Object is filled back later
fn ray_sphere(from: Point, direction: Vector, center: Point, radius: float) -> Option<RayHit> {
    // Center of the sphere, shifted as if the ray was short from the origo
    let relative = center - from;

    let c = relative.len2() - radius.powi(2);
    let d = direction.dot(relative).powi(2) - c;

    if d <= 0.0 {
        return None;
    }

    let t_a = 0.5 * d.sqrt() + relative.dot(direction);
    let t_b = -0.5 * d.sqrt() + relative.dot(direction);
    let distance = t_a.min(t_b);

    if distance <= 0.0 {
        return None;
    }

    let hit_point: Point = from + direction * distance;
    let normal = (hit_point - center).normalized();

    Some(RayHit {
        object: 0,
        distance,
        normal,
    })
}

/// Object is filled back later
fn ray_triange(from: Point, direction: Vector, corners: [Point; 3]) -> Option<RayHit> {
    let edge1 = corners[1] - corners[0];
    let edge2 = corners[2] - corners[0];

    if edge1.len2() < 0.00001 || edge2.len2() < 0.00001 {
        return None;
    }

    let normal = edge1.normalized().cross(edge2.normalized());
    if normal.len2() < 0.0001 {
        println!("{:?}", edge1);
        println!("{:?}", edge2);
        println!("{:?}", normal);
        panic!("!!");
        // return None;
    }
    
    let normal = normal.normalized();

    // Is parallel?
    if normal.dot(direction).abs() < 0.000001 {
        return None;
    }

    // Plane intersection point
    let distance = (normal.dot(corners[0] - from)) / normal.dot(direction);

    // Behind the ray?
    if distance <= 0.0001 {
        return None;
    }

    let hit_point = from + direction * distance;

    // Inside-outside test for each edge

    let edge = corners[1] - corners[0];
    // assert!(edge.len2() > 0.0001);
    let vp = hit_point - corners[0];
    if vp.len2() < 0.0001 {
        return None;
    }
    if normal.dot(edge.cross(vp)) < 0.0 {
        return None;
    }

    let edge = corners[2] - corners[1];
    // assert!(edge.len2() > 0.0001);
    let vp = hit_point - corners[1];
    if vp.len2() < 0.0001 {
        return None;
    }
    if normal.dot(edge.cross(vp)) < 0.0 {
        return None;
    }

    let edge = corners[0] - corners[2];
    // assert!(edge.len2() > 0.0001);
    let vp = hit_point - corners[2];
    if vp.len2() < 0.0001 {
        return None;
    }
    if normal.dot(edge.cross(vp)) < 0.0 {
        return None;
    }

    Some(RayHit {
        object: 0,
        distance,
        normal: if normal.dot(direction) >= 0.0 {
            -normal
        } else {
            normal
        },
    })
}
