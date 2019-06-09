pub use self::particle::Particle;
pub use self::vector3::Vector3;
pub use self::ballistic::BallisticDemo;
pub use self::timing::TimingData;

mod particle;
mod vector3;
mod ballistic;
mod timing;

pub trait Application{
    fn handle_key(&self, virtual_keycode: Option<glium::glutin::VirtualKeyCode>, closed: &mut bool);
    fn update(&mut self);
    fn draw(&mut self);
}
