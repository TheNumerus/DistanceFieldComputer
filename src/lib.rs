extern crate image;

pub mod settings;

use settings::{GenSettings};
use image::{DynamicImage, GenericImage, Pixel};

#[derive(Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(coords: (f32, f32, f32)) -> Vec3 {
        Vec3{x: coords.0, y: coords.1, z: coords.2}
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Vec3 {
        Vec3::new((self.x, self.y, self.z))
    }
}

#[derive(Debug)]
pub struct Point {
    pub coords: Vec3,
}

impl Point {
    fn new(coords: (f32, f32, f32)) -> Point {
        Point {coords: Vec3::new(coords)}
    }

    fn distance_to_point(&self, another: &Point) -> f32 {
        let deltas = (self.coords.x - another.coords.x, self.coords.y - another.coords.y, self.coords.z - another.coords.z);
        (deltas.0.powi(2) + deltas.1.powi(2) + deltas.2.powi(2)).sqrt()
    }

    fn distance_to_coords(&self, coords: (f32, f32, f32)) -> f32 {
        let deltas = (self.coords.x - coords.0, self.coords.y - coords.1, self.coords.z - coords.2);
        (deltas.0.powi(2) + deltas.1.powi(2) + deltas.2.powi(2)).sqrt()
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point::new((self.coords.x, self.coords.y, self.coords.z))
    }
}

#[derive(Debug)]
pub struct Face {
    pub verts: Vec<Point>
}

impl Face {
    fn new(first: Point, second: Point, third: Point) -> Face{
        Face{verts: vec![first, second, third]}
    }

    fn compute_normal(&self) -> Vec3 {
        // NOT YET IMPLEMENTED
        panic!("Kuso FUCK OFF")
    }

    fn distance_to_point(&self, coords: (f32, f32, f32)) -> f32 {
        // NOT YET IMPLEMENTED
        panic!("Kuso FUCK OFF")
    }
}

pub fn generate_mesh (img: &DynamicImage, settings: &GenSettings) -> Vec<Face>{
    let bounds = img.dimensions();
    let bounds = (bounds.0 -1, bounds.1 -1);
    // generate main part
    let mut faces : Vec<Face> = Vec::new();
    for y in 0..(bounds.1 - 1) {
        for x in 0..(bounds.0 - 1) {
            let point0 = Point::new(((x as f32) + 0.5, (y as f32) + 0.5, compute_height(img.get_pixel(x, 0).channels4().0, &settings)));
            let point1 = Point::new(((x as f32) + 0.5, (y as f32) + 1.5, compute_height(img.get_pixel(x, y + 1).channels4().0, &settings)));
            let point2 = Point::new(((x as f32) + 1.5, (y as f32) + 0.5, compute_height(img.get_pixel(x + 1, y).channels4().0, &settings)));
            let point3 = Point::new(((x as f32) + 1.5, (y as f32) + 1.5, compute_height(img.get_pixel(x + 1, y + 1).channels4().0, &settings)));
            let face0 = Face::new(point0, point1.clone(), point2.clone());
            let face1 = Face::new(point2, point1, point3);
            faces.push(face0);
            faces.push(face1);
        }
    }
    faces
}

#[derive(Debug)]
pub struct Extrema {
    min: u8,
    max: u8
}

pub fn get_image_extrema(img: &DynamicImage) -> Extrema {
    let mut e = Extrema{min: 255, max: 0};
    for pixel in img.pixels() {
        // only get the red channel, since all images should be monochrome
        let value = pixel.2.channels4().0;
        if value > e.max {
            e.max = value
        }
        if value < e.min {
            e.min = value
        }
    }
    e
}

pub fn get_capture_height(ext: &Extrema) -> u8 {
    if 255 > ext.max {
        return ext.max
    }
    255
}

pub fn compute_height(img_value: u8, settings: &settings::GenSettings) -> f32 {
    ((img_value / 255) as f32) * (settings.radius as f32) * (settings.img_height_mult)
}
