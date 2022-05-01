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

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(
        &display, 
        vertex_shader::VERTEX_SHADER_SRC, 
        fragment_shader::FRAGMENT_SHADER_SRC, 
        None
    ).unwrap();

    let vertices = vec![
        Vertex { position: [-0.5, -0.5] },
        Vertex { position: [0.0, 0.5] },
        Vertex { position: [0.5, -0.25] },
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
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0 , 0.0, 0.0, 1.0],
            ]
        };


        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
