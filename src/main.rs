#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate nalgebra as na;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{Event, GlContext, GlRequest, WindowEvent};
use glutin::Api::OpenGl;

use na::{Matrix4, Point3, Vector3, Vector4};

pub mod color;
pub mod mesh;
pub mod program;
pub mod object;

use color::Color;
use mesh::Mesh;
use program::{pipe, ColorFormat, DepthFormat, Light};

use object::Object;

fn main() {
    let width = 800;
    let height = 600;

    let mut event_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Window")
        .with_dimensions(width, height);
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl, (4, 1)))
        .with_vsync(true);
    let (window, mut device, mut factory, color_view, depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &event_loop);

    let program = factory
        .create_pipeline_simple(
            include_bytes!("assets/shaders/shader.vert"),
            include_bytes!("assets/shaders/shader.frag"),
            pipe::new(),
        )
        .unwrap();

    let mut running = true;

    let model_trans = Object::new(
        Point3::new(0.5, 0.0, -5.5),
        Vector3::new(1.0, 1.0, 1.0),
        Vector3::new(45.0f32.to_radians(), 45.0f32.to_radians(), 0.0),
    );

    let view_mat = Matrix4::look_at_rh(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(0.0, 0.0, -1.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );

    let projection_mat = Matrix4::new_perspective(
        width as f32 / height as f32,
        90f32.to_radians(),
        0.01,
        100.0,
    );

    let mut tri_mesh = Mesh::new();

    tri_mesh
        .add_verticies(&[
            Vector3::new(0.5, 0.5, 0.5),    // 0- front, top, right
            Vector3::new(-0.5, 0.5, 0.5),   // 1- front, top, left
            Vector3::new(-0.5, -0.5, 0.5),  // 2- front, bottom, left
            Vector3::new(0.5, -0.5, 0.5),   // 3- front, bottom, right
            Vector3::new(0.5, 0.5, -0.5),   // 4- back, top, right
            Vector3::new(-0.5, 0.5, -0.5),  // 5- back, top, left
            Vector3::new(-0.5, -0.5, -0.5), // 6- back, bottom, left
            Vector3::new(0.5, -0.5, -0.5),  // 7- back, bottom, right
        ])
        .add_tris(&[
            // front
            (0, 1, 3),
            (1, 2, 3),
            //back
            (5, 4, 7),
            (7, 6, 4),
            //top
            (0, 5, 1),
            (0, 4, 5),
            //bottom
            (6, 7, 3),
            (3, 2, 6),
            //left
            (1, 5, 6),
            (2, 1, 6),
            //right
            (0, 7, 4),
            (0, 3, 7),
        ])
        .preprocess();

    let mut tri_data = tri_mesh
        .build(&mut factory, color_view.clone(), depth_view.clone())
        .unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    while running {
        event_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::Closed |
                    WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => running = false,
                    _ => {}
                }
            }
        });

        encoder.clear(&color_view, Color::gray().into());
        encoder.clear_depth(&depth_view, 1.0);

        let mut lights = Vec::new();

        const LIGHT_COUNT: u32 = 9;

        for _ in 0..LIGHT_COUNT {
            lights.push(Light {
                position: Vector4::new(0.0, 0.0, 0.0, 1.0).into(),
                direction: Vector4::new(0.0, 0.0, -1.0, 0.0).into(),
                diffuse_color: Color::white().into(),
                // _padding: 0i32,
                // _padding2: 0,
            });
        }

        object::upload_lights(&mut encoder, &mut tri_data, lights.as_slice());

        object::draw(
            &mut encoder,
            &mut tri_data,
            &program,
            &model_trans,
            &view_mat,
            &projection_mat,
        );

        encoder.flush(&mut device); // execute draw commands

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
