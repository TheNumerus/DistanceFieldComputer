use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Index, Sub};

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
        self.x *= inv_len;
        self.y *= inv_len;
        self.z *= inv_len;
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

    pub fn distance_to(&self, other: &Vec3) -> f32 {
        (self - other).len()
    }

    pub fn cmp_x(&self, other: &Vec3) -> Ordering {
        if self.x > other.x {
            return Ordering::Greater;
        } else if (self.x - other.x).abs() < 0.01 {
            return Ordering::Equal;
        };
        Ordering::Less
    }

    pub fn cmp_y(&self, other: &Vec3) -> Ordering {
        if self.y > other.y {
            return Ordering::Greater;
        } else if (self.y - other.y).abs() < 0.01 {
            return Ordering::Equal;
        };
        Ordering::Less
    }

    pub fn cmp_xy(&self, other: &Vec3) -> Ordering {
        if self.y > other.y {
            return Ordering::Greater;
        } else if (self.y - other.y).abs() < 0.01 {
            if self.x > other.x {
                return Ordering::Greater;
            } else if (self.x - other.x).abs() < 0.01 {
                return Ordering::Equal;
            };
            return Ordering::Less;
        };
        Ordering::Less
    }

    pub fn delta(&self, other: &Vec3) -> Vec3 {
        Vec3::new((self.x - other.x, self.y - other.y, self.z - other.z))
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

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'b Vec3) -> Vec3 {
        Vec3::new((self.x - other.x, self.y - other.y, self.z - other.z))
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new((self.x - other.x, self.y - other.y, self.z - other.z))
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3::new((self.x + other.x, self.y + other.y, self.z + other.z))
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, num: usize) -> &f32 {
        match num {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}
