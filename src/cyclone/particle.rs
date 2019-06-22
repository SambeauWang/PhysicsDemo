use super::Vector3;

#[derive(Debug, Default)]
pub struct Particle{
    inverse_mass: f32,
    damping: f32,
    position: Vector3,
    velocity: Vector3,
    force_accum: Vector3,
    acceleration: Vector3,
}

impl Particle{
    pub fn clear_accumulator(&mut self){
        self.force_accum.clear();
    }

    pub fn integrate(&mut self, duration: f32){
        if self.inverse_mass < 0.0 { return; }
        assert!(duration > 0.0);

        self.position.addScaledVector(self.velocity, duration);

        let mut result_acc = self.acceleration.clone();
        result_acc.addScaledVector(self.force_accum, self.inverse_mass);

        self.velocity.addScaledVector(result_acc, duration);
        self.velocity *= self.damping.powf(duration);

        self.clear_accumulator();
    }

    pub fn set_mass(&mut self, mass: f32){
        assert_ne!(mass, 0.0);
        self.inverse_mass = 1.0/mass;
    }

    pub fn get_mass(self) -> f32{
        if (self.inverse_mass == 0.0){
            std::f32::MAX
        }else{
            1.0 / self.inverse_mass
        }
    }

    pub fn set_inverse_mass(&mut self, inverse_mass: f32){
        self.inverse_mass = inverse_mass;
    }

    pub fn get_inverse_mass(self) -> f32{
        self.inverse_mass
    }

    pub fn has_finite_mass(self) -> bool{
        self.inverse_mass == 0.0
    }

    pub fn set_damping(&mut self, damping: f32){
        self.damping = damping;
    }

    pub fn get_damping(self) -> f32{
        self.damping
    }

    pub fn set_positionXYZ(&mut self, x: f32, y: f32, z: f32){
        self.position = Vector3::new(x, y, z);
    }

    pub fn set_position(&mut self, position: Vector3){
        self.position = position;
    }

    pub fn get_position(&self) -> Vector3{
        self.position.clone()
    }

    pub fn set_velocityXYZ(&mut self, x: f32, y: f32, z: f32){
        self.velocity = Vector3::new(x, y, z);
    }

    pub fn set_velocity(&mut self, velocity: Vector3){
        self.velocity = velocity;
    }

    pub fn get_velocity(self) -> Vector3{
        self.velocity.clone()
    }

    pub fn set_accelerationXYZ(&mut self, x: f32, y: f32, z: f32){
        self.acceleration = Vector3::new(x, y, z);
    }

    pub fn set_acceleration(&mut self, acceleration: Vector3){
        self.acceleration = acceleration;
    }

    pub fn get_acceleration(self) -> Vector3{
        self.acceleration.clone()
    }

    pub fn add_force(&mut self, force: Vector3){
        self.force_accum += force;
    }
}