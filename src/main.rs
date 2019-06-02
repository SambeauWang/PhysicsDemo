#[macro_use]
extern crate glium;

mod cyclone;
use cyclone::Application;

use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex{
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

struct App{
    display:glium::Display,
    vertex_buffer:glium::VertexBuffer<Vertex>,
    indices:glium::index::NoIndices,
    program:glium::Program
}

impl App{
    fn new(events_loop: &glium::glutin::EventsLoop) -> App{
        let wb = glium::glutin::WindowBuilder::new();
        let cb = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        // vertex data
        let vertex1 = Vertex{ position: [-0.5, -0.5]};
        let vertex2 = Vertex{ position: [0.0, 0.5]};
        let vertex3 = Vertex{ position: [0.5, -0.25]};
        let shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // shader
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            void main(){
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140
            out vec4 color;
            void main(){
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let app = App{
            display:display,
            vertex_buffer:vertex_buffer,
            indices:indices,
            program:program
        };

        app
    }
}

impl Application for App{
    fn handle_key(&self, virtual_keycode: Option<glium::glutin::VirtualKeyCode>, closed: &mut bool){
        match virtual_keycode{
            Some(glium::glutin::VirtualKeyCode::Escape) => *closed=true,
            _ => ()
        }
    }

    fn update(&self){

    }

    fn draw(&self){
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer, &self.indices, &self.program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}

fn main() {
    // let mut events_loop = glutin::EventsLoop::new();

    // let mut closed = false;
    // let app = App::new(&events_loop);
    // while !closed {
    //     app.draw();

    //     events_loop.poll_events(|ev|{
    //         match ev{
    //             glutin::Event::WindowEvent{event, ..} => match event{
    //                 glutin::WindowEvent::CloseRequested => closed = true,
    //                 glutin::WindowEvent::KeyboardInput{
    //                     input: glutin::KeyboardInput{virtual_keycode, ..},
    //                     ..} => app.handle_key(virtual_keycode, &mut closed),
    //                 _ => (),
    //             },
    //             _ => (),
    //         }
    //     });
    // }
    let t = cyclone::Vector3::new(1.0, 2.0, 3.0);
    println!("{:?}", t);
}
