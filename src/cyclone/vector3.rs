use std::ops::{Index, AddAssign, Add, SubAssign, Sub, Mul, MulAssign, Div, DivAssign};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3{
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3{
    pub fn new(x: f32, y: f32, z: f32) -> Vector3{
        Vector3{
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn componentProduct(&self, other: Self) -> Self{
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }

    pub fn componentProductUpdate(&mut self, other: Self){
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }

    pub fn vectorProduct(&self, other: Self) -> Self{
        Self {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }

    pub fn scalarProduct(&self, other: Self) -> f32{
        self.x*other.x + self.y * other.y + self.z * other.z
    }

    pub fn addScaledVector(&mut self, base: Self, scale: f32){
        self.x += base.x * scale;
        self.y += base.y * scale;
        self.z += base.z * scale;
    }

    pub fn squareMagnitude(&self) -> f32{
        self.x*self.x + self.y*self.y + self.z *self.z
    }

    pub fn magnitude(&self) -> f32{
        self.squareMagnitude().sqrt()
    }

    pub fn trim(&mut self, size: f32){
        let t = self.squareMagnitude();
        if t > size*size{
            *self *= size;
        }
    }

    pub fn normalise(&mut self){
        let l = self.magnitude();
        if l > 0.0 {
            *self *= (1.0/l);
        }
    }

    pub fn unit(&self) -> Self{
        let mut res = self.clone();
        res.normalise();
        res
    }

    pub fn clear(&mut self){
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn invert(&mut self){
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

impl Index<i32> for Vector3{
    type Output = f32;
    fn index(&self, i: i32) -> &Self::Output{
        match i{
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl AddAssign for Vector3{
    fn add_assign(&mut self, other: Self){
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Add for Vector3{
    type Output = Self;

    fn add(self, other: Self) -> Self{
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl SubAssign for Vector3{
    fn sub_assign(&mut self, other: Self){
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Sub for Vector3{
    type Output = Self;

    fn sub(self, other: Self) -> Self{
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vector3{
    type Output = f32;

    fn mul(self, other: Self) -> Self::Output{
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}

impl Mul<f32> for Vector3{
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output{
        Self{
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f32> for Vector3{
    fn mul_assign(&mut self, rhs: f32){
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div for Vector3{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output{
        self.vectorProduct(other)
    }
}

impl DivAssign for Vector3{
    fn div_assign(&mut self, other: Self){
        *self = Self {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
}

impl PartialOrd for Vector3{
    fn partial_cmp(&self, other: &Self) ->Option<Ordering>{
        if self.x == other.x && self.y == other.y && self.z == other.z{
            Some(Ordering::Equal)
        }else if self.x <= other.x && self.y <= other.y && self.z <= other.z{
            Some(Ordering::Less)
        }else if self.x >= other.x && self.y >= other.y && self.z >= other.z{
            Some(Ordering::Greater)
        }else{
            None
        }
    }
}

impl PartialEq for Vector3{
    fn eq(&self, other: &Self) -> bool{
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

