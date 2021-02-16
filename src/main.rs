pub mod prelude;

mod angle;
mod object;
mod vector;

use std::time::Instant;

pub use crate::vector::{Point, Vector};

use pixels::{Error, Pixels, SurfaceTexture};
use prelude::float;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn raytrace(direction: Vector, objects: &[object::Sphere]) -> [u8; 4] {
    // debug_assert!(direction.is_nonzero());

    let mut closest = None;

    for (i, sphere) in objects.iter().enumerate() {
        // (vx * t - px)**2 + (vy * t - py)**2 + (vz * t - pz)**2 = r**2

        let dot_prod = sphere.center.dot(direction);

        let a = direction.len();
        let b = 2.0 * dot_prod;
        let c = sphere.center.len() - sphere.radius.powi(2);
        let d = b * b - 4.0 * a * c;

        if d <= 0.0 {
            continue;
        }

        let t_a = 0.5 * d.sqrt() + dot_prod;
        let t_b = -0.5 * d.sqrt() + dot_prod;

        if t_a <= 0.0 && t_b <= 0.0 {
            continue;
        }
        
        let dist = t_a.min(t_b);
        if let Some((distance, _)) = closest {
            if distance > dist {
                closest = Some((dist, i));
            }
        } else {
            closest = Some((dist, i));
        }
    }

    if let Some((_dist, i)) = closest {
        return match i {
            0 => [0xff, 0x00, 0x00, 0xff],
            1 => [0x00, 0xff, 0x00, 0xff],
            2 => [0x00, 0x00, 0xff, 0xff],
            _ => [0xff, 0xff, 0xff, 0xff],
        };
    } else {
        [0x00, 0x00, 0x00, 0xff]

    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let mut time_start = Instant::now();
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

    let mut objects = vec![
        object::Sphere {
            center: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.2,
        },
        object::Sphere {
            center: Point {
                x: 1.0,
                y: -0.2,
                z: 0.0,
            },
            radius: 0.12,
        },
    ];

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // update
            let t = (time_start.elapsed().as_micros() as float) / 1_000_000.0;
            objects[1].center.x = 1.0 + t.sin() * 0.3;
            objects[1].center.y = 0.0;
            objects[1].center.z = t.cos() * 0.3;

            // draw

            let frame = pixels.get_frame();

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let x = (i % WIDTH as usize) as u16;
                let y = (i / WIDTH as usize) as u16;

                let aspect_ratio = (WIDTH as f32) / (HEIGHT as f32);

                let cx = 0.5 * (WIDTH as f32);
                let cy = 0.5 * (HEIGHT as f32);

                let pz = (cx - x as f32) / (WIDTH as f32) * aspect_ratio;
                let py = (cy - y as f32) / (HEIGHT as f32);
                let px = 1.0f32; // affects fov calculation

                let len = (px.powi(2) + py.powi(2) + pz.powi(2)).sqrt();

                let px = px / len;
                let py = py / len;
                let pz = pz / len;

                let color = raytrace(
                    Vector {
                        x: px,
                        y: py,
                        z: pz,
                    },
                    &objects,
                );

                pixel.copy_from_slice(&color);
            }

            pixels.render().unwrap();
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Key1) || input.quit() {
                objects[0].center.x += 0.01;
                println!("{}", objects[0].center.x);
            }

            if input.key_pressed(VirtualKeyCode::Key2) || input.quit() {
                objects[0].center.x -= 0.01;
                println!("{}", objects[0].center.x);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}
