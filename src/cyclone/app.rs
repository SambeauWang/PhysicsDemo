use glium::{glutin, Surface};
use glium::glutin::dpi::LogicalSize;
use cgmath::{Matrix4, Point3, Vector3, PerspectiveFov, Rad, Vector4};
use super::vertex::Vertex;
// use cgmath::conv::*;
use super::sphere;

use std::sync::Once;
use std::sync::{Arc, Mutex};
use std::mem;

pub struct App{
    display: glium::Display,
    sphere: sphere::Sphere,
    indices: glium::index::NoIndices,
    program: glium::Program,
    project_matrix: PerspectiveFov<f32>,
    view_matrix: Matrix4<f32>,
    pub events_loop: glutin::EventsLoop,
}

impl App{
    fn update(&mut self){

    }

    pub fn begin_draw(&mut self) -> glium::Frame{
        let mut target = self.display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        // 
        let model_matrix = Matrix4::from_translation(Vector3{x: 0.0, y: 1.5, z:0.0})*Matrix4::from_scale(0.1);
        self.draw_sphere(&mut target, model_matrix, Some(Vector4{x: 0.0, y: 0.0, z:0.0, w:1.0}));

        // draw lines
        let mut vertices = Vec::<Vertex>::with_capacity(20);
        for i in 0..20 {
            vertices.push(Vertex{
                position: [-5.0, 0.0, (i as f32)*10.0],
                normal: [0.0, 0.0, 0.0],
                texcoord: [0.0, 0.0]
            });
            vertices.push(Vertex{
                position: [5.0, 0.0, (i as f32)*10.0],
                normal: [0.0, 0.0, 0.0],
                texcoord: [0.0, 0.0]
            });
        }
        self.draw_lines(&mut target, &vertices);

        target
    }

    pub fn end_draw(&mut self, target: glium::Frame){
        target.finish().unwrap();
    }

    // draw sphere
    pub fn draw_sphere(&mut self, target: &mut glium::Frame, model_matrix: Matrix4<f32>, color: Option<Vector4<f32>>){
        let color = color.unwrap_or(Vector4{x: 0.0, y: 0.0, z:0.0, w:1.0});

        let uniforms = uniform!{
            model_matrix: Into::<[[f32; 4]; 4]>::into(model_matrix),
            view_matrix: Into::<[[f32; 4]; 4]>::into(self.view_matrix),
            project_matrix: Into::<[[f32; 4]; 4]>::into(Matrix4::from(self.project_matrix)),
            color: Into::<[f32; 4]>::into(color),
        };

        target.draw(&self.sphere.vertices, &self.indices, &self.program, &uniforms, &Default::default()).unwrap();
    }

    // draw line
    pub fn draw_lines(&mut self, target: &mut glium::Frame, vertex: &Vec::<Vertex>){
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertex).unwrap();
        let color = Vector4{x: 0.75, y: 0.75, z:0.75, w:1.0};
        let uniforms = uniform!{
            model_matrix: Into::<[[f32; 4]; 4]>::into(Matrix4::from_translation(Vector3{x: 0.0, y:0.0, z:0.0})),
            view_matrix: Into::<[[f32; 4]; 4]>::into(self.view_matrix),
            project_matrix: Into::<[[f32; 4]; 4]>::into(Matrix4::from(self.project_matrix)),
            color: Into::<[f32; 4]>::into(color),
        };

        target.draw(&vertex_buffer, &glium::index::NoIndices(glium::index::PrimitiveType::LinesList), &self.program, &uniforms, &Default::default()).unwrap();
    }

    fn new() -> App{
        let events_loop = glutin::EventsLoop::new();

        // windows
        let wb = glium::glutin::WindowBuilder::new().with_dimensions(LogicalSize::new(960.0, 480.0)).with_title("Example Viewer");
        let cb = glium::glutin::ContextBuilder::new();
        let display = glium::backend::glutin::Display::new(wb, cb, &events_loop).unwrap();

        // sphere
        let sphere = sphere::SphereBuilder::new().scale(1.0, 1.0, 1.0).build(&display)
        .expect("Failed to build sphere shape");

        // matrix
        let view_matrix = Matrix4::look_at(Point3{x:-25.0, y:8.0, z:5.0}, Point3{x:0.0, y:5.0, z:22.0}, Vector3{x:0.0, y:1.0, z:0.0});
        let project_matrix = PerspectiveFov{fovy: Rad(3.1415_f32 / 3.0), aspect:960.0/480.0, near:1.0, far:500.0};

        // vertex data
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // shader
        let vertex_shader_src = r#"
            #version 140
            in vec3 position;

            uniform mat4 model_matrix;
            uniform mat4 view_matrix;
            uniform mat4 project_matrix;

            void main(){
                gl_Position = project_matrix*view_matrix*model_matrix*vec4(position, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140
            uniform vec4 color;

            out vec4 a_color;
            void main(){
                a_color = color;
            }
        "#;
        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        let app = App{
            events_loop: events_loop,
            display: display,
            sphere: sphere,
            indices: indices,
            program: program,
            view_matrix: view_matrix,
            project_matrix: project_matrix,
        };

        app
    }

    pub fn get() -> Arc<Mutex<App>>{
        static mut VAL: *const Arc<Mutex<App>> = 0 as *const Arc<Mutex<App>>;
        static INIT: Once = Once::new();

        unsafe{
            INIT.call_once(|| {
                let val = App::new();
                VAL = mem::transmute(Box::new(Arc::new(Mutex::new(val))));
            });

            (*VAL).clone()
        }
    }
}