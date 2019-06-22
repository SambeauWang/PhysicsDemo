#[macro_use]
extern crate glium;
extern crate std;

mod cyclone;
use cyclone::Application;
use cyclone::BallisticDemo;
use cyclone::TimingData;
use cyclone::App;

use glium::{glutin, Surface};

fn main() {
    // let mut events_loop = glutin::EventsLoop::new();

    let mut closed = false;
    let mut app = BallisticDemo::new();
    while !closed {
        TimingData::get().lock().unwrap().update();
        app.update();
        app.draw();

        App::get().lock().unwrap().events_loop.poll_events(|ev|{
            match ev{
                glutin::Event::WindowEvent{event, ..} => match event{
                    glutin::WindowEvent::CloseRequested => closed = true,
                    glutin::WindowEvent::KeyboardInput{
                        input: glutin::KeyboardInput{virtual_keycode, ..}, ..
                    } => app.handle_key(virtual_keycode, &mut closed),
                    glutin::WindowEvent::MouseInput{
                        state, ..
                    } => app.handle_mouse(state),
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
