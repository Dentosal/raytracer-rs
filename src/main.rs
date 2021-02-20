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

use crate::object::{Material, Object, Shape};
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
            let material = objects[hit.object].material.clone();

            if material.emits_light {
                let w = (-direction).dot(hit.normal);
                assert!(w >= 0.0);
                acc_color = acc_color + (material.color * mask_color).darken(w);
            }

            let w = (-direction).dot(hit.normal);
            mask_color = mask_color * material.color.darken(w);

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

    let mut objects = vec![
        Object {
            shape: Shape::Sphere {
                center: Point {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                radius: 0.2,
            },
            material: Material {
                color: Color {
                    r: 1.0,
                    g: 0.2,
                    b: 0.2,
                },
                emits_light: false,
            },
        },
        Object {
            shape: Shape::Sphere {
                center: Point {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                radius: 0.08,
            },
            material: Material {
                color: Color::GREEN,
                emits_light: true,
            },
        },
        Object {
            shape: Shape::Sphere {
                center: Point {
                    x: 3.0,
                    y: 0.0,
                    z: 0.0,
                },
                radius: 0.08,
            },
            material: Material {
                color: Color::WHITE,
                emits_light: true,
            },
        },
    ];

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

                    let color = raytrace(camera.pos(), camera.mul_rotate(p), &objects, sun);
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

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // update
            let t = (time_start.elapsed().as_micros() as float) / 1_000_000.0;

            if let Shape::Sphere { center, .. } = &mut objects[1].shape {
                *center = Vector {
                    x: 1.0 + t.sin() * 0.3,
                    y: 0.0,
                    z: t.cos() * 0.3,
                };
            }

            if let Shape::Sphere { center, .. } = &mut objects[2].shape {
                *center = Vector {
                    x: 1.0 + (t + 3.14).sin() * 0.3,
                    y: (t + 3.14).cos() * 0.3,
                    z: 0.0,
                };
            }

            window.request_redraw();
        }
    });
}
