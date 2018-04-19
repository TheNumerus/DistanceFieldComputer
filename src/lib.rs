extern crate image;

use image::{DynamicImage, GenericImage, Pixel};

struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    fn new(coords: (f32, f32, f32)) -> Vec3 {
        Vec3{x: coords.0, y: coords.1, z: coords.2}
    }
}

struct Point {
    coords: Vec3,
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

struct Face {
    verts: Vec<Vec3>
}

impl Face {
    fn compute_normal(&self) -> Vec3 {
        // NOT YET IMPLEMENTED
        panic!("Kuso FUCK OFF")
    }

    fn distance_to_point(&self, coords: (f32, f32, f32)) -> f32 {
        // NOT YET IMPLEMENTED
        panic!("Kuso FUCK OFF")
    }
}

fn generate_mesh () {

}

struct Extrema {
    min: u8,
    max: u8
}

fn get_image_extrema(img: &DynamicImage) -> Extrema {
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