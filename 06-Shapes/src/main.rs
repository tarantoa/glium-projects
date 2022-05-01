#[macro_use] extern crate glium;

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
use util::shapes::teapot;

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

    let program = glium::Program::from_source(
        &display, 
        vertex_shader::VERTEX_SHADER_SRC, 
        fragment_shader::FRAGMENT_SHADER_SRC, 
        None
    ).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES).unwrap();

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

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ];

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw((&positions, &normals), &indices, &program, &uniform!{ matrix: matrix },
            &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
