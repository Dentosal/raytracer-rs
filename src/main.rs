#![feature(const_fn_floating_point_arithmetic)]

pub mod prelude;

mod angle;
mod color;
mod matrix;
mod object;
mod raycast;
mod vector;

pub use crate::angle::Angle;
pub use crate::color::Color;
pub use crate::matrix::Matrix;
pub use crate::vector::{Point, Vector};

use crate::object::{Object, Shape};
use crate::prelude::float;
use crate::raycast::raycast;

use rayon::prelude::*;
use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

// const WIDTH: u32 = 640;
// const HEIGHT: u32 = 480;

const WIDTH: u32 = 64*2;
const HEIGHT: u32 = 48*2;

const BOUNCES: usize = 3;

fn random_nudge(vector: Vector, weigth: float) -> Vector {
    (vector + Vector::random_spherepoint() * weigth).normalized()
}

fn raytrace(
    mut from: Point,
    mut direction: Vector,
    objects: &[object::Object],
    materials: &[tobj::Material],
    sun: Vector,
) -> Color {
    direction = direction.normalized();

    let mut any_hits = false;
    let mut mask_color = Color::WHITE; // Surfaces only reflect their own color
    let mut acc_color = Color::BLACK; // Total color

    for _ in 0..=BOUNCES {
        if let Some(hit) = raycast(from, direction, objects) {
            any_hits = true;

            let hit_point: Point = from + direction * hit.distance;
            if let Some(material_id) = objects[hit.object].material_id {
                let material = &materials[material_id];

                let w = (-direction).dot(hit.normal);
                assert!(w >= 0.0);
                acc_color = acc_color + (Color::from(material.ambient) * mask_color).darken(w);
                mask_color = mask_color * Color::from(material.diffuse).darken(w);
            } else {
                // Default material
                let w = (-direction).dot(hit.normal);
                mask_color = mask_color * Color::WHITE.darken(w);
            }

            let reflection = direction.reflect(hit.normal);

            // let reflection = if reflect_mode {
            //     direction.reflect(hit.normal)
            // } else {
            //     Vector::random_spherepoint().normalized()
                // random_nudge(hit.normal, 0.8)
            // };

            // Epsilon hack to avoid self-collision
            direction = (reflection + hit.normal * 1.0001).normalized();
            from = hit_point;
        } else {
            // No hit, check for sun
            let s = (-sun).dot(direction);
            if s > 0.0 && any_hits {
                acc_color = acc_color + (Color::WHITE * mask_color).darken(s);
            }

            // Skybox
            acc_color = acc_color + (Color::WHITE * mask_color).darken(0.4);

            break;
        }
    }

    acc_color
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let time_start = Instant::now();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Raytracer test")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut camera = Matrix::translation(Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }) * Matrix::rotation(
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Angle { radians: 0.0 },
    );

    let sun = (Vector {
        x: 0.1,
        y: -1.0,
        z: 0.2,
    })
    .normalized();

    let mut objects = Vec::new();

    let (models, materials) =
        tobj::load_obj("objs/cornell_box.obj", true).expect("Failed to load file");

    const SCALE: f32 = 1.0 / 4.0;
    // const SCALE: f32 = 1.0;

    for model in models.iter() {
        let mesh = &model.mesh;
        let mut next_face = 0;
        for f in 0..mesh.num_face_indices.len() {
            let end = next_face + mesh.num_face_indices[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();

            assert_eq!(face_indices.len(), 3);
            let mut tri = [Vector::ZERO; 3];
            for i in 0..3 {
                let k = *face_indices[i] as usize;
                tri[i] = Vector {
                    x: mesh.positions[3 * k + 0] * SCALE,
                    y: mesh.positions[3 * k + 1] * SCALE,
                    z: mesh.positions[3 * k + 2] * SCALE,
                };
            }
            objects.push(Object {
                shape: Shape::Triangle { corners: tri },
                material_id: mesh.material_id,
            });

            next_face = end;
        }
    }

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // draw

            let frame = pixels.get_frame();

            frame
                .par_chunks_exact_mut(4)
                .enumerate()
                .for_each(|(i, pixel)| {
                    let x = (i % WIDTH as usize) as u16;
                    let y = (i / WIDTH as usize) as u16;

                    let aspect_ratio = (WIDTH as f32) / (HEIGHT as f32);

                    let cx = 0.5 * (WIDTH as f32);
                    let cy = 0.5 * (HEIGHT as f32);

                    let p = (Vector {
                        z: (cx - x as f32) / (WIDTH as f32) * aspect_ratio,
                        y: (cy - y as f32) / (HEIGHT as f32),
                        x: 1.0f32, // affects fov calculation
                    })
                    .normalized();

                    let rays = 1;

                    let mut sum = Color::BLACK;
                    for i in 0..rays {
                        sum = sum
                            + raytrace(
                                camera.pos(),
                                camera.mul_rotate(p),
                                &objects,
                                &materials,
                                sun,
                            );
                    }

                    let color = sum / (rays as float);

                    let c = color.to_pixel_color();

                    // let c = [
                    //     4 * pixel[0] / 5 + c[0] / 5,
                    //     4 * pixel[1] / 5 + c[1] / 5,
                    //     4 * pixel[2] / 5 + c[2] / 5,
                    //     0xff
                    // ];

                    pixel.copy_from_slice(&c);
                });

            pixels.render().unwrap();
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                camera = camera
                    * Matrix::translation(Vector {
                        x: 0.0,
                        y: 0.1,
                        z: 0.0,
                    });
            }

            if input.key_pressed(VirtualKeyCode::E) {
                camera = camera
                    * Matrix::translation(Vector {
                        x: 0.0,
                        y: -0.1,
                        z: 0.0,
                    });
            }

            if input.key_pressed(VirtualKeyCode::W) {
                camera = camera
                    * Matrix::translation(Vector {
                        x: 0.1,
                        y: 0.0,
                        z: 0.0,
                    });
            }

            if input.key_pressed(VirtualKeyCode::S) {
                camera = camera
                    * Matrix::translation(Vector {
                        x: -0.1,
                        y: 0.0,
                        z: 0.0,
                    });
            }

            if input.key_pressed(VirtualKeyCode::A) {
                camera = camera
                    * Matrix::rotation(
                        Vector {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        },
                        Angle { radians: -0.1 },
                    );
            }

            if input.key_pressed(VirtualKeyCode::D) {
                camera = camera
                    * Matrix::rotation(
                        Vector {
                            x: 0.0,
                            y: 1.0,
                            z: 0.0,
                        },
                        Angle { radians: 0.1 },
                    );
            }

            if input.key_pressed(VirtualKeyCode::Z) {
                camera = camera
                    * Matrix::rotation(
                        Vector {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Angle { radians: -0.1 },
                    );
            }

            if input.key_pressed(VirtualKeyCode::X) {
                camera = camera
                    * Matrix::rotation(
                        Vector {
                            x: 1.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        Angle { radians: 0.1 },
                    );
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // update
            // let t = (time_start.elapsed().as_micros() as float) / 1_000_000.0;

            window.request_redraw();
        }
    });
}
