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

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const BOUNCES: usize = 5;

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
            acc_color = acc_color + (Color::WHITE * mask_color).darken(0.1);

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
        tobj::load_obj("objs/cornell_box.obj", false).expect("Failed to load file");

    const SCALE: f32 = 1.0 / 500.0;
    // const SCALE: f32 = 1.0;

    for (i, model) in models.iter().enumerate() {
        let mesh = &model.mesh;
        let mut next_face = 0;
        for f in 0..mesh.num_face_indices.len() {
            let end = next_face + mesh.num_face_indices[f] as usize;
            let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();

            match face_indices.len() {
                3 => {
                    let mut tri = [Vector::ZERO; 3];
                    for i in 0..3 {
                        tri[i] = Vector {
                            x: mesh.positions[3 * i + 0] * SCALE,
                            y: mesh.positions[3 * i + 1] * SCALE,
                            z: mesh.positions[3 * i + 2] * SCALE,
                        };
                    }
                    objects.push(Object {
                        shape: Shape::Triangle { corners: tri },
                        material_id: mesh.material_id,
                    });
                }
                4 => {
                    let mut tri1 = [Vector::ZERO; 3];
                    let mut tri2 = [Vector::ZERO; 3];
                    for i in 0..3 {
                        tri1[i] = Vector {
                            x: mesh.positions[3 * i + 0] * SCALE,
                            y: mesh.positions[3 * i + 1] * SCALE,
                            z: mesh.positions[3 * i + 2] * SCALE,
                        };
                    }
                    for (i, k) in [0usize, 2, 3].iter().copied().enumerate() {
                        tri2[i] = Vector {
                            x: mesh.positions[3 * k + 0] * SCALE,
                            y: mesh.positions[3 * k + 1] * SCALE,
                            z: mesh.positions[3 * k + 2] * SCALE,
                        };
                    }
                    objects.push(Object {
                        shape: Shape::Triangle { corners: tri1 },
                        material_id: mesh.material_id,
                    });
                    objects.push(Object {
                        shape: Shape::Triangle { corners: tri2 },
                        material_id: mesh.material_id,
                    });
                }
                other => panic!("Unsupported face sides {}", other),
            }

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

                    let color = raytrace(
                        camera.pos(),
                        camera.mul_rotate(p),
                        &objects,
                        &materials,
                        sun,
                    );
                    let c = color.to_pixel_color();

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
