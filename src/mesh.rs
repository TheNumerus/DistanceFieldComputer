use face::Face;
use image::{DynamicImage, GenericImage, Pixel};
use settings::{GenSettings, ImgRepeat};
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

#[derive(Debug)]
pub struct Mesh {
    pub faces: Vec<Face>,
    dimensions: (u32, u32),
}

impl Mesh {
    pub fn flip(&self, axis: (bool, bool)) -> Mesh {
        panic!("Not yet implemented");
    }

    pub fn translate(&self, coords: &Vec3) -> Mesh {
        let mut faces: Vec<Face> = Vec::new();
        for face in self.faces.iter() {
            faces.push(Face::new(
                &face.verts[0] + coords,
                &face.verts[1] + coords,
                &face.verts[2] + coords,
            ))
        }
        Mesh {
            faces,
            dimensions: self.dimensions.clone(),
        }
    }

    pub fn clamp(points: &Vec<Vec3>, axis: MeshClamp) -> Mesh {
        panic!("Not yet implemented");
    }

    pub fn append_data(&mut self, other: &mut Mesh) {
        self.faces.append(&mut other.faces);
    }

    pub fn compute_borders(&mut self, settings: ImgRepeat) {
        panic!("Not yet implemented");
    }

    pub fn clean_far_faces(&mut self, settings: GenSettings) {
        let mut faces: Vec<Face> = Vec::new();
        let rad = settings.radius as f32;
        for face in self.faces.iter() {
            let x = face.center.x < ((self.dimensions.0 as f32) + 10.0 + rad)
                && face.center.x > (rad - 10.0);
            let y = face.center.y < ((self.dimensions.1 as f32) + 10.0 + rad)
                && face.center.y > (rad - 10.0);
            if x && y {
                faces.push(face.clone());
            }
        }
        self.faces = faces;
    }

    // very inefficient for now
    // has extreme redundancy
    pub fn generate_obj(&self) {
        let mut file = File::create("output.obj").unwrap();
        for face in self.faces.iter() {
            for point in face.verts.iter() {
                file.write_all(format!("v {} {} {}\n", point.x, point.y, point.z).as_bytes())
                    .unwrap();
            }
        }
        println!("All points written.");
        let mut i = 1;
        for _ in &self.faces {
            file.write_all(format!("f {} {} {}\n", i + 2, i, i + 1).as_bytes())
                .unwrap();
            i = i + 3;
        }
        println!("All faces written.");
    }

    pub fn generate_mesh(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let dim = img.dimensions();
        let bounds = (dim.0 - 1, dim.1 - 1);
        // generate main part
        let mut faces: Vec<Face> = Vec::new();
        for y in 0..(bounds.1) {
            for x in 0..(bounds.0) {
                // image axis y is positive on the way down, so we flip it
                let coords = Mesh::image_to_mesh_coords((x, y), dim);
                let point0 = Vec3::new((
                    coords.0,
                    coords.1,
                    Mesh::compute_height(img.get_pixel(x, y).channels4().0, &settings),
                ));
                let point1 = Vec3::new((
                    coords.0,
                    coords.1 - 1.0,
                    Mesh::compute_height(img.get_pixel(x, y + 1).channels4().0, &settings),
                ));
                let point2 = Vec3::new((
                    coords.0 + 1.0,
                    coords.1,
                    Mesh::compute_height(img.get_pixel(x + 1, y).channels4().0, &settings),
                ));
                let point3 = Vec3::new((
                    coords.0 + 1.0,
                    coords.1 - 1.0,
                    Mesh::compute_height(img.get_pixel(x + 1, y + 1).channels4().0, &settings),
                ));
                let face0 = Face::new(point0, point1.clone(), point2.clone());
                let face1 = Face::new(point2, point1, point3);
                faces.push(face0);
                faces.push(face1);
            }
        }
        Mesh {
            faces,
            dimensions: dim,
        }
    }

    fn compute_height(img_value: u8, settings: &GenSettings) -> f32 {
        ((img_value as f32 / 255.0) * (settings.radius as f32) * (settings.img_height_mult))
    }

    pub fn image_to_mesh_coords(input: (u32, u32), dim: (u32, u32)) -> (f32, f32) {
        let x = (input.0 as f32) + 0.5;
        let y = (dim.1 as f32) - (input.1 as f32) - 0.5;
        (x, y)
    }
}

pub enum MeshClamp {
    Up,
    Down,
    Left,
    Right,
}
