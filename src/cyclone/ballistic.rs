use super::Particle;
use super::Application;
use super::TimingData;
use super::App;

#[derive(Debug, PartialEq)]
enum ShotType{
    UNUSED,
    PISTOL,
    ARTILLERY,
    FIREBALL,
    LASER,
}

impl Default for ShotType{
    fn default() -> Self{
        ShotType::UNUSED
    }
}

#[derive(Debug)]
struct AmmoRound{
    pub particle: Particle,
    pub shot_type: ShotType,
    pub start_time: u128,
}

impl Default for AmmoRound{
    fn default() -> Self{
        AmmoRound{
            particle: Default::default(),
            shot_type: ShotType::UNUSED,
            start_time: 0,
        }
    }
}

// AmmoRound render
impl AmmoRound{
    fn render(){

    }
}

pub struct BallisticDemo{
    ammo: Vec<AmmoRound>,
    cur_shot_type: ShotType,
    app: App,
}

impl BallisticDemo{
    fn fire(&mut self){
        let shot = match self.ammo.iter_mut().find(|c| (**c).shot_type == ShotType::UNUSED){
            Some(c) => Some(c),
            None =>{
                self.ammo.push(AmmoRound::default());
                self.ammo.last_mut()
            }
        };

        if let Some(item) = shot{
            item.particle.set_mass(1.0);
            item.particle.set_velocityXYZ(0.0, 0.0, 10.0);
            item.particle.set_accelerationXYZ(0.0, 0.6, 0.0);
            item.particle.set_damping(0.9);

            // set time
            item.particle.set_positionXYZ(0.0, 1.5, 0.0);
            item.start_time = TimingData::get().lock().unwrap().lastFrameTimestamp;
            item.shot_type = ShotType::FIREBALL;

            item.particle.clear_accumulator()
        }
    }
}

impl Application for BallisticDemo{
    fn new(events_loop: &glium::glutin::EventsLoop) -> BallisticDemo{
        BallisticDemo{
            ammo: Default::default(),
            cur_shot_type: ShotType::UNUSED,
            app: App::new(&events_loop),
        }
    }
    
    fn handle_mouse(&mut self, state: glium::glutin::ElementState){
        if state == glium::glutin::ElementState::Released{
            self.fire();
        }
    }

    fn handle_key(&self, virtual_keycode: Option<glium::glutin::VirtualKeyCode>, closed: &mut bool){
        match virtual_keycode{
            Some(glium::glutin::VirtualKeyCode::Escape) => *closed=true,
            _ => ()
        }
    }
    fn update(&mut self){
        let duration = TimingData::get().lock().unwrap().lastFrameDuration as f32 * 0.001;
        if duration <= 0.0 {
            return;
        }

        for i in &mut self.ammo{
            match i.shot_type{
                ShotType::UNUSED => (),
                _ => {
                    i.particle.integrate(duration);

                    let postion = i.particle.get_position();
                    if postion.y < 0.0 || i.start_time + 5000 < TimingData::get().lock().unwrap().lastFrameTimestamp || postion.z > 200.0 {
                        i.shot_type = ShotType::UNUSED;
                    }
                }
            }
        }
    }
    fn draw(&mut self){
        self.app.draw();

        for i in &mut self.ammo{
            match i.shot_type {
                ShotType::UNUSED => (),
                _ => {
                    // println!("{} {}", i.particle.get_position().y, i.particle.get_position().z)
                },
            }
        }
    }
}

