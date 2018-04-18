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