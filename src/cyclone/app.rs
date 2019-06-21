use glium::Surface;
use cgmath::{Matrix4, Point3, Vector3};
use cgmath::conv::*;

#[derive(Copy, Clone)]
struct Vertex{
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct App{
    display:glium::Display,
    vertex_buffer:glium::VertexBuffer<Vertex>,
    indices:glium::index::NoIndices,
    program:glium::Program
}

impl App{
    fn update(&mut self){

    }

    pub fn draw(&mut self){
        // view matrix
        let viewmatrix = Matrix4::look_at(Point3{x:-25.0, y:8.0, z:5.0}, Point3{x:0.0, y:5.0, z:22.0}, Vector3{x:0.0, y:1.0, z:0.0});
        let uniforms = uniform!{
            viewmatrix: Into::<[[f32; 4]; 4]>::into(viewmatrix),
        };

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }

    pub fn new(events_loop: &glium::glutin::EventsLoop) -> App{
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
            uniform mat4 viewmatrix;
            void main(){
                gl_Position = viewmatrix*vec4(position, 0.5, 1.0);
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