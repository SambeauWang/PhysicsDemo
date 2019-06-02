pub use self::particle::Particle;
pub use self::vector3::Vector3;
pub use self::ballistic::BallisticDemo;

mod particle;
mod vector3;
mod ballistic;

pub trait Application{
    fn handle_key(&self, virtual_keycode: Option<glium::glutin::VirtualKeyCode>, closed: &mut bool);
    fn update(&self);
    fn draw(&self);
}
