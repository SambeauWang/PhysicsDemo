pub use self::particle::Particle;
pub use self::vector3::Vector3;
pub use self::ballistic::BallisticDemo;
pub use self::timing::TimingData;
pub use self::app::App;

mod particle;
mod vector3;
mod ballistic;
mod timing;
mod app;
mod errors;
mod vertex;
mod sphere;

pub trait Application{
    fn handle_key(&self, virtual_keycode: Option<glium::glutin::VirtualKeyCode>, closed: &mut bool);
    fn update(&mut self);
    fn draw(&mut self);
    // fn new(events_loop: &glium::glutin::EventsLoop) -> Self;
    fn new() -> Self;
    fn handle_mouse(&mut self, state: glium::glutin::ElementState);
}
