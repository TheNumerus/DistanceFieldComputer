use std::fmt;
use std::ops::{Add, Sub};

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(coords: (f32, f32, f32)) -> Vec3 {
        Vec3 {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        let inv_len = 1.0 / self.len();
        self.x = self.x * inv_len;
        self.y = self.y * inv_len;
        self.z = self.z * inv_len;
    }

    pub fn normalized(&self) -> Vec3 {
        let inv_len = 1.0 / self.len();
        Vec3::new((self.x * inv_len, self.y * inv_len, self.z * inv_len))
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new((
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        ))
    }

    pub fn distance_to(self, other: &Vec3) -> f32 {
        (&self - other).len()
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3::new((self.x, self.y, self.z))
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({};{};{})", self.x, self.y, self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<'a> Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3::new((self.x - other.x, self.y - other.y, self.z - other.z))
    }
}

impl<'a> Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: &'a Vec3) -> Vec3 {
        Vec3::new((self.x + other.x, self.y + other.y, self.z + other.z))
    }
}
