use face::Face;
use image::{DynamicImage, GenericImage, Pixel};
use settings::{GenSettings, ImgRepeat};
use std::f32;
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

const MOVES: [[f32; 2]; 8] = [
    [-1.0, 0.0],
    [-1.0, 1.0],
    [0.0, 1.0],
    [1.0, 1.0],
    [1.0, 0.0],
    [1.0, -1.0],
    [0.0, -1.0],
    [-1.0, -1.0],
];

#[derive(Debug)]
pub struct Mesh {
    pub faces: Vec<Face>,
    dimensions: (u32, u32),
}

impl Mesh {
    /// Main function for generating the whole mesh
    pub fn generate(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let mut middle = Mesh::generate_mesh_from_img(img, settings);
        println!("Middle part generated");
        middle.compute_out_of_range_mesh(settings);
        middle
    }

    // very inefficient for now
    // has extreme redundancy
    /// Export mesh data in obj. format.
    pub fn export(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
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

    /// Convert image coorinated to mesh coordinates
    pub fn image_to_mesh_coords(input: (u32, u32), dim: (u32, u32)) -> (f32, f32) {
        let x = (input.0 as f32) + 0.5;
        let y = (dim.1 as f32) - (input.1 as f32) - 0.5;
        (x, y)
    }

    pub fn empty_copy(other: &Mesh) -> Mesh {
        Mesh {
            faces: Vec::new(),
            dimensions: other.dimensions,
        }
    }

    /// Flipes mesh data along given axis.
    fn flip(&mut self, axis: (bool, bool))  {
        let mut flipped_faces: Vec<Face> = Vec::new();
        match axis {
            (false, false) => (),
            (false, true) => {
                let coords = (0.0, self.dimensions.1 as f32);
                for face in &mut self.faces {
                    let mut new_face = face.clone();
                    for vert in &mut new_face.verts {
                        vert.y = coords.1 - vert.y;
                    }
                    face.recompute();
                    flipped_faces.push(new_face);
                }
            },
            (true, false) => {
                let coords = (self.dimensions.0 as f32, 0.0);
                for face in &mut self.faces {
                    let mut new_face = face.clone();
                    for vert in &mut new_face.verts {
                        vert.x = coords.0 - vert.x;
                    }
                    face.recompute();
                    flipped_faces.push(new_face);
                }
            },
            (true, true) => {
                let coords = (self.dimensions.0 as f32, self.dimensions.1 as f32);
                for face in &mut self.faces {
                    let mut new_face = face.clone();
                    for vert in &mut new_face.verts {
                        vert.x = coords.0 - vert.x;
                        vert.y = coords.1 - vert.y;
                    }
                    face.recompute();
                    flipped_faces.push(new_face);
                }
            }
        }
        self.faces = flipped_faces;
    }

    /// Moves mesh data by given cooridnates.
    fn translate(&mut self, coords: Vec3) {
        let mut faces: Vec<Face> = Vec::new();
        for face in self.faces.iter() {
            faces.push(Face::new(
                &face.verts[0] + &coords,
                &face.verts[1] + &coords,
                &face.verts[2] + &coords,
            ))
        }
        self.faces = faces;
    }

    /// Return generated clippped version of the mesh.
    fn clamp(points: &Vec<Vec3>, axis: MeshClamp) -> Mesh {
        panic!("Not yet implemented");
    }

    /// Merge two meshes together.
    fn append_data(&mut self, other: &mut Mesh) {
        self.faces.append(&mut other.faces);
    }

    /// Generate border triangles.
    fn compute_border_faces(&mut self, settings: &ImgRepeat) {
        // panic!("Not yet implemented");
    }

    /// Remove unaccessible data from mesh.
    fn clean_far_faces(&mut self, settings: &GenSettings) {
        let mut faces: Vec<Face> = Vec::new();
        let rad = settings.radius as f32;
        for face in self.faces.iter() {
            let x = face.center.x < ((self.dimensions.0 as f32) + 10.0 + rad)
                && face.center.x > -(rad + 10.0);
            let y = face.center.y < ((self.dimensions.1 as f32) + 10.0 + rad)
                && face.center.y > -(rad + 10.0);
            if x && y {
                faces.push(face.clone());
            }
        }
        self.faces = faces;
    }

    /// Generate mesh data out of given image frame.
    fn compute_out_of_range_mesh(&mut self, settings: &GenSettings) {
        let mut outer_mesh: Mesh = match settings.repeat {
            ImgRepeat::Repeat => {
                let mut outer = Mesh::empty_copy(&self);
                for xy in MOVES.iter() {
                    let mut moved = self.clone();
                    moved.translate(Vec3::new((
                        self.dimensions.0 as f32 * xy[0],
                        self.dimensions.1 as f32 * xy[1],
                        0.0,
                    )));
                    outer.append_data(&mut moved);
                }
                outer
            }
            ImgRepeat::Clamp => {
                panic!("Not yet implemented");
            }
            ImgRepeat::Mirror => {
                let mut outer = Mesh::empty_copy(&self);
                for xy in MOVES.iter() {
                    let mut moved = self.clone();
                    if f32::abs(xy[0]) + f32::abs(xy[1]) == 2.0 {
                        moved.flip((true, true));
                    } else if f32::abs(xy[0]) == 1.0 && f32::abs(xy[1]) == 0.0 {
                        moved.flip((true, false));
                    } else if f32::abs(xy[0]) == 0.0 && f32::abs(xy[1]) == 1.0 {
                        moved.flip((false, true));
                    }
                    moved.translate(Vec3::new((
                        self.dimensions.0 as f32 * xy[0],
                        self.dimensions.1 as f32 * xy[1],
                        0.0,
                    )));
                    outer.append_data(&mut moved);
                }
                outer
            }
        };
        println!("Out of range parts generated");
        outer_mesh.compute_border_faces(&settings.repeat);
        println!("Border faces generated");
        outer_mesh.clean_far_faces(&settings);
        println!("Far faces cleaned");
        outer_mesh.recompute_normals();
        println!("Normals recomputed");
        self.append_data(&mut outer_mesh);
    }

    /// Recompute mesh normals
    fn recompute_normals(&mut self) {
        // panic!("Not yet implemented");
    }

    /// Generate mesh data from given image.
    fn generate_mesh_from_img(img: &DynamicImage, settings: &GenSettings) -> Mesh {
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
                    Mesh::compute_height(img.get_pixel(x, y).channels()[0], &settings),
                ));
                let point1 = Vec3::new((
                    coords.0,
                    coords.1 - 1.0,
                    Mesh::compute_height(img.get_pixel(x, y + 1).channels()[0], &settings),
                ));
                let point2 = Vec3::new((
                    coords.0 + 1.0,
                    coords.1,
                    Mesh::compute_height(img.get_pixel(x + 1, y).channels()[0], &settings),
                ));
                let point3 = Vec3::new((
                    coords.0 + 1.0,
                    coords.1 - 1.0,
                    Mesh::compute_height(img.get_pixel(x + 1, y + 1).channels()[0], &settings),
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

    /// Compute mesh height from given image value
    fn compute_height(pix: u8, settings: &GenSettings) -> f32 {
        (((pix as f32) / 255.0) * (settings.radius as f32) * (settings.img_height_mult))
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Mesh {
        Mesh {
            faces: self.faces.clone(),
            dimensions: self.dimensions,
        }
    }
}

pub enum MeshClamp {
    Up,
    Down,
    Left,
    Right,
}
