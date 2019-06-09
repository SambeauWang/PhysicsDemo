use super::Particle;
use super::Application;
use super::TimingData;

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

#[derive(Default, Debug)]
struct AmmoRound{
    pub particle: Particle,
    pub shot_type: ShotType,
    pub start_time: u128,
}

// AmmoRound render
impl AmmoRound{
    fn render(){

    }
}

pub struct BallisticDemo{
    nAmmoRound: u32,
    ammo: Vec<AmmoRound>,
    cur_shot_type: ShotType,
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
            item.shot_type = ShotType::UNUSED;

            item.particle.clear_accumulator()
        }
    }
}

impl Application for BallisticDemo{
    fn handle_key(&self, virtual_keycode: Option<glium::glutin::VirtualKeyCode>, closed: &mut bool){

    }
    fn update(&mut self){
        let time_data = TimingData::get().lock().unwrap();
        let duration = time_data.lastFrameDuration;
        if duration < 0 {
            return;
        }

        for i in &mut self.ammo{
            if i.shot_type != ShotType::UNUSED{
                i.particle.integrate(duration as f32);

                if i.particle.get_position().y < 0.0 || i.start_time + 5000 < time_data.lastFrameTimestamp || i.particle.get_position().z > 200.0 {
                    i.shot_type = ShotType::UNUSED;
                }
            }
        }
    }
    fn draw(&mut self){

    }
}

