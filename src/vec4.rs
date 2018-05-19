use std::fmt;
use std::ops::{Add, Index, Sub};

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(coords: (f32, f32, f32, f32)) -> Vec4 {
        Vec4 {
            x: coords.0,
            y: coords.1,
            z: coords.2,
            w: coords.3,
        }
    }
}

impl Clone for Vec4 {
    fn clone(&self) -> Vec4 {
        Vec4::new((self.x, self.y, self.z, self.w))
    }
}

impl fmt::Debug for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({};{};{};{})", self.x, self.y, self.z, self.w)
    }
}

impl PartialEq for Vec4 {
    fn eq(&self, other: &Vec4) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl<'a> Sub<&'a Vec4> for &'a Vec4 {
    type Output = Vec4;
    fn sub(self, other: &'a Vec4) -> Vec4 {
        Vec4::new((self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w))
    }
}

impl<'a> Add<&'a Vec4> for &'a Vec4 {
    type Output = Vec4;
    fn add(self, other: &'a Vec4) -> Vec4 {
        Vec4::new((self.x + other.x, self.y + other.y, self.z + other.z, self.w - other.w))
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;
    fn index(&self, num: usize) -> &f32 {
        match num {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Invalid index"),
        }
    }
}
