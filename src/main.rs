#[macro_use]
extern crate glium;

mod cyclone;
use cyclone::Application;
use cyclone::BallisticDemo;
use cyclone::TimingData;

use glium::{glutin, Surface};

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
    // let t = cyclone::Vector3::new(1.0, 2.0, 3.0);
    // println!("{:?}", t);

    let mut events_loop = glutin::EventsLoop::new();

    let mut closed = false;
    let mut app = BallisticDemo::new(&events_loop);
    while !closed {
        TimingData::get().lock().unwrap().update();
        app.update();
        app.draw();

        events_loop.poll_events(|ev|{
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
