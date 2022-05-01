#[macro_use] extern crate glium;
extern crate image;

mod shaders;
mod util;

use glium::{
    glutin, 
    Surface,
};
use shaders::{
    vertex_shader,
    fragment_shader,
};
use std::io::Cursor;
use util::vertex::Vertex;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();
    let display = glium::Display::new(
        window_builder, 
        context_builder, 
        &event_loop
    )
    .unwrap();

    // Load image
    let image = image::load(Cursor::new(&include_bytes!("../img/29757705.png")),
        image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    // Upload image as texture
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        &display, 
        vertex_shader::VERTEX_SHADER_SRC, 
        fragment_shader::FRAGMENT_SHADER_SRC, 
        None
    ).unwrap();

    let vertices = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [0.0, 0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [0.5, -0.25], tex_coords: [1.0, 0.0] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices)
        .unwrap();

    let mut t: f32 = -0.5;
    event_loop.run(move |ev, _, control_flow| {
        
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0],
            ],
            tex: &texture,
        };


        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
