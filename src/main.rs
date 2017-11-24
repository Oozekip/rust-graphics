extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate nalgebra as na;
extern crate regex;
extern crate rgraphics;
extern crate time;

use gfx::traits::FactoryExt;
use gfx::Device;
use gfx_window_glutin as gfx_glutin;
use glutin::{Event, GlContext, GlRequest, WindowEvent};
use glutin::Api::OpenGl;

use na::{Matrix4, Point3, Vector3};

use rgraphics as rg;

use rg::color::Color;
use rg::light::Light;
// use mesh::Mesh;
use rg::program::{pipe, ColorFormat, DepthFormat};
use rg::material::Material;
use rg::mesh_loader;
use rg::light;
use rg::object;
use rg::object::Object;
use rgraphics::texture;

fn main() {
    let mut width = 800;
    let mut height = 600;

    let mut last_time = time::now();
    let mut elapsed_time = 0.0;

    let mut event_loop = glutin::EventsLoop::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Window")
        .with_dimensions(width, height);
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(GlRequest::Specific(OpenGl, (4, 1)))
        .with_vsync(true);
    let (window, mut device, mut factory, mut color_view, mut depth_view) =
        gfx_glutin::init::<ColorFormat, DepthFormat>(window_builder, context_builder, &event_loop);

    let program = factory
        .create_pipeline_simple(
            include_bytes!("assets/shaders/shader.vert"),
            include_bytes!("assets/shaders/shader.frag"),
            pipe::new(),
        )
        .unwrap();

    let mut running = true;


    let mut model_trans2 = Object::new(
        Material::Untextured {
            diffuse_color: Color::gray(),
            ambient_color: Color::black(),
            specular_color: Color::white(),
            specular_power: 3.0,
        },
        Point3::new(-0.25, 0.0, -8.0),
        Vector3::from_element(0.5),
        Vector3::zeros(),
    );

    let view_mat = Matrix4::look_at_rh(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(0.0, 0.0, -1.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );

    let mut projection_mat = Matrix4::new_perspective(
        width as f32 / height as f32,
        90f32.to_radians(),
        0.01,
        100.0,
    );


    const LIGHT_COUNT: usize = 1;

    let lights = vec![
        Light::new_point(
            Point3::new(0.0, 0.0, 0.0),
            // Vector3::new(0.0, 0.0, -1.0),
            // f32::to_radians(15.0),
            // f32::to_radians(30.0),
            // 1.0,
            Color::white(),
            Color::white(),
            Color::white()
        );
        LIGHT_COUNT
    ];

    let diff_tex = texture::load_texture(&mut factory, "src/assets/textures/diffuse.tga").unwrap();
    let spec_tex = texture::load_texture(&mut factory, "src/assets/textures/specular.tga").unwrap();

    let mat = Material::Textured {
        diffuse_texture: diff_tex,
        specular_texture: spec_tex,
        ambient_color: Color::black(),
    };

    // let mat = Material::Untextured {
    //         diffuse_color: Color::gray(),
    //         ambient_color: Color::black(),
    //         specular_color: Color::white(),
    //         specular_power: 3.0,
    //     };
    let mut model_trans = Object::new(
        mat,
        Point3::new(0.0, 0.0, -1.0),
        Vector3::from_element(0.25),
        Vector3::zeros(),
    );

    let bunny_mesh = mesh_loader::load_file("src/assets/models/suzanne.obj").unwrap();
    let horse_mesh = mesh_loader::load_file("src/assets/models/cube.obj").unwrap();
    //let cube_mesh = mesh_loader::load_file("src/assets/models/cube.obj").unwrap();

    let mut bunny_data = bunny_mesh
        .build(&mut factory, color_view.clone(), depth_view.clone())
        .unwrap();

    let mut horse_data = horse_mesh
        .build(&mut factory, color_view.clone(), depth_view.clone())
        .unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    // Upload light data before loop as they do not currently change
    light::upload_lights(&mut encoder, &mut bunny_data, lights.as_slice());
    light::upload_lights(&mut encoder, &mut horse_data, lights.as_slice());

    println!("Running");
    while running {
        // Update times and get dt
        let curr_time = time::now();
        let diff = curr_time - last_time;
        let nano = diff.num_nanoseconds().unwrap();
        let dt = nano as f32 / 1000000000.0;
        elapsed_time += dt;

        last_time = curr_time;

        // Poll events
        event_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    // Receive window closed event or excape key pressed
                    WindowEvent::Closed |
                    WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => running = false,

                    // Receive resize event
                    WindowEvent::Resized(w, h) => {
                        // Update width and height
                        width = w;
                        height = h;

                        // Resize the context (necessary in Walyand and OSX)
                        window.resize(width, height);

                        // Update render views for the window
                        gfx_glutin::update_views(&window, &mut color_view, &mut depth_view);

                        // Update remder views for mesh
                        bunny_data.update_views(color_view.clone(), depth_view.clone());
                        horse_data.update_views(color_view.clone(), depth_view.clone());

                        projection_mat = Matrix4::new_perspective(
                            width as f32 / height as f32,
                            90f32.to_radians(),
                            0.01,
                            100.0,
                        );
                    }

                    _ => {}
                }
            }
        });

        // Rotate the cube
        model_trans.rotation += Vector3::new(45f32.to_radians(), 90f32.to_radians(), 0.0) * dt;
        model_trans2.rotation +=
            Vector3::new((-45f32).to_radians(), (-90f32).to_radians(), 0.0) * dt;
        model_trans2.position = Point3::new(
            f32::cos(elapsed_time) * 2.0,
            0.0,
            f32::sin(elapsed_time) * 2.0 + -1.0,
        );
        // Clear buffers
        encoder.clear(&color_view, Color::black().into());
        encoder.clear_depth(&depth_view, 1.0);

        // Draw the bunny
        object::draw(
            &mut encoder,
            &mut bunny_data,
            &program,
            &model_trans,
            &view_mat,
            &projection_mat,
        );

        // Draw the horse
        object::draw(
            &mut encoder,
            &mut horse_data,
            &program,
            &model_trans2,
            &view_mat,
            &projection_mat,
        );


        // Flush command buffers
        encoder.flush(&mut device);

        // Swap buffers
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
