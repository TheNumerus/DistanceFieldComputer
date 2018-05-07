use face::Face;
use image::{DynamicImage, GenericImage, Pixel};
use settings::{GenSettings, ImgRepeat};
use std::f32;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Write};
use std::rc::Rc;
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
const EXPORT_SCALE:f32 = 1.0/100.0;


#[derive(Debug)]
pub struct Mesh {
    pub faces: Vec<Face>,
    dimensions: (u32, u32),
    ext_dim: (usize, usize),
    pub verts: Vec<Rc<RefCell<Vec3>>>,
}

impl Mesh {
    /// Main function for generating the whole mesh
    pub fn generate(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let mut middle = Mesh::generate_mesh_from_img(img, settings);
        println!("Middle part generated");
        middle.compute_out_of_range_mesh(settings);
        middle.compute_faces();
        middle
    }

    /// Export mesh data in obj. format.
    pub fn export(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        let mut data = String::from("");
        for point in self.verts.iter() {
            let p = point.borrow();
            data.push_str(&format!("v {} {} {}\n", p.x * EXPORT_SCALE, p.y * EXPORT_SCALE, p.z * EXPORT_SCALE));
        }
        println!("All points written.");
        for (i, _face) in self.faces.iter().enumerate() {
            let row = (i/2) / self.ext_dim.0;
            let pos_in_row = i - row * self.ext_dim.0 * 2;
            match i % 2 {
                0 => {
                     data.push_str(&format!("f {} {} {}\n",
                        pos_in_row/2 + (row + 1) * (self.ext_dim.0 + 1) + 2,
                        pos_in_row/2 + (row + 1) * (self.ext_dim.0 + 1) + 1,
                        pos_in_row/2 + row * (self.ext_dim.0 + 1) + 1,
                        ));
                },
                1 => {
                     data.push_str(&format!("f {} {} {}\n",
                        pos_in_row/2 + (row + 1) * (self.ext_dim.0 + 1) + 2,
                        pos_in_row/2 + row * (self.ext_dim.0 + 1) + 1,
                        pos_in_row/2 + row * (self.ext_dim.0 + 1) + 2
                        ));
                },
                _ => ()
            };
        }
        println!("All faces written.");
        file.write_all(data.as_bytes()).unwrap();
    }

    /// Convert image coorinated to mesh coordinates
    pub fn image_to_mesh_coords(input: (u32, u32), dim: (u32, u32)) -> (f32, f32) {
        let x = (input.0 as f32) + 0.5;
        let y = (dim.1 as f32) - (input.1 as f32) - 0.5;
        (x, y)
    }

    /// Convert image coorinated to mesh coordinates
    pub fn mesh_to_image_coords(input: (u32, u32), dim: (u32, u32)) -> (f32, f32) {
        let x = (input.0 as f32) - 0.5;
        let y = (dim.1 as f32) + (input.1 as f32) + 0.5;
        (x, y)
    }

    /// Creates empty mesh
    pub fn empty_copy(other: &Mesh) -> Mesh {
        Mesh {
            faces: Vec::new(),
            dimensions: other.dimensions,
            ext_dim: other.ext_dim,
            verts: Vec::new(),
        }
    }

    /// Flipes mesh data along given axis.
    fn flip(&mut self, axis: (bool, bool)) {
        let coords = (self.dimensions.0 as f32, self.dimensions.1 as f32);
        for vert in self.verts.iter_mut() {
            let mut v = vert.borrow_mut();
            if axis.0 {
                v.x = coords.0 - v.x;
            }
            if axis.1 {
                v.y = coords.1 - v.y;
            }
        }
    }

    /// Moves mesh data by given cooridnates.
    fn translate(&mut self, coords: Vec3) {
        for vert in self.verts.iter_mut() {
            let mut v = vert.borrow_mut();
            v.x = v.x + coords.x;
            v.y = v.y + coords.y;
            v.z = v.z + coords.z;
        }
    }

    /// Return generated clippped version of the mesh.
    fn clamp(mesh: &Mesh, axis: MeshClamp) -> Mesh {
        let mut clamped = Mesh::empty_copy(mesh);
        let mut sub_vec: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        let border = (
            mesh.dimensions.0 as f32 - 0.5,
            mesh.dimensions.1 as f32 - 0.5,
        );
        // get points
        for point in mesh.verts.iter() {
            let p = point.borrow();
            match axis {
                MeshClamp::Left => {
                    if p.x == 0.5 && !sub_vec.contains(&point) {
                        sub_vec.push(Rc::clone(point));
                    }
                }
                MeshClamp::Up => {
                    if p.y == border.1 && !sub_vec.contains(&point) {
                        sub_vec.push(Rc::clone(point));
                    }
                }
                MeshClamp::Right => {
                    if p.x == border.0 && !sub_vec.contains(&point) {
                        sub_vec.push(Rc::clone(point));
                    }
                }
                MeshClamp::Down => {
                    if p.y == 0.5 && !sub_vec.contains(&point) {
                        sub_vec.push(Rc::clone(point));
                    }
                }
            }
        }
        // sort points
        sub_vec.sort_unstable_by(|a, b| {
            let a_borrowed = a.borrow();
            let b_borrowed = b.borrow();
            a_borrowed.cmp_x(&b_borrowed)
        });
        // flip coords, do we avoid out of bounds error
        let mut dim = (mesh.dimensions.1, mesh.dimensions.0);
        match axis {
            MeshClamp::Left | MeshClamp::Right => dim = (mesh.dimensions.0, mesh.dimensions.1),
            _ => (),
        }
        // generate mesh
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        for y in 0..(dim.0) {
            for x in 0..(dim.1) {
                verts.push(Rc::new(RefCell::new(Vec3::new((
                    x as f32 + 0.5,
                    y as f32 + 0.5,
                    sub_vec[x as usize].borrow().z,
                )))));
            }
        }
        clamped.verts = verts;
        // rotate mesh
        match axis {
            MeshClamp::Left | MeshClamp::Right => {
                for point in clamped.verts.iter_mut() {
                    let mut p = point.borrow_mut();
                    let temp = p.x;
                    p.x = p.y;
                    p.y = mesh.dimensions.1 as f32 - temp;
                }
            }
            MeshClamp::Up | MeshClamp::Down => (),
        };
        clamped
    }

    /// Merge two meshes together.
    fn append_data(&mut self, other: &mut Mesh) {
        self.faces.append(&mut other.faces);
        self.verts.append(&mut other.verts);
    }

    /// Remove unaccessible data from mesh.
    fn clean_far_verts(&mut self, settings: &GenSettings) {
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        let rad = settings.radius as f32;
        for vert in self.verts.iter() {
            let v = vert.borrow();
            let x = v.x < ((self.dimensions.0 as f32) + 10.0 + rad) && v.x > -(rad + 10.0);
            let y = v.y < ((self.dimensions.1 as f32) + 10.0 + rad) && v.y > -(rad + 10.0);
            if x && y {
                verts.push(Rc::clone(vert));
            }
        }
        self.verts = verts;
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
                let mut outer = Mesh::empty_copy(&self);
                // get corner coordinates
                // 0───1
                // │   │
                // 3───2
                let mut corners: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
                let borders = (
                    self.dimensions.0 as f32 - 0.5,
                    self.dimensions.1 as f32 - 0.5,
                );
                for point in self.verts.iter() {
                    let p = point.borrow();
                    if p.x == 0.5 && p.y == 0.5 {
                        corners[3] = p.z;
                    } else if p.x == borders.0 && p.y == 0.5 {
                        corners[2] = p.z;
                    } else if p.x == 0.5 && p.y == borders.1 {
                        corners[0] = p.z;
                    } else if p.x == borders.0 && p.y == borders.1 {
                        corners[1] = p.z;
                    }
                }
                for xy in MOVES.iter() {
                    let mut moved: Mesh = Mesh::empty_copy(&self);
                    // corners
                    if xy[0] == -1.0 && xy[1] == 1.0 {
                        moved = Mesh::generate_corner_from_height(self.dimensions, corners[0]);
                    } else if xy[0] == 1.0 && xy[1] == 1.0 {
                        moved = Mesh::generate_corner_from_height(self.dimensions, corners[1]);
                    } else if xy[0] == 1.0 && xy[1] == -1.0 {
                        moved = Mesh::generate_corner_from_height(self.dimensions, corners[2]);
                    } else if xy[0] == -1.0 && xy[1] == -1.0 {
                        moved = Mesh::generate_corner_from_height(self.dimensions, corners[3]);
                    } else if xy[0] == -1.0 && xy[1] == 0.0 {
                        moved = Mesh::clamp(self, MeshClamp::Left);
                    } else if xy[0] == 0.0 && xy[1] == 1.0 {
                        moved = Mesh::clamp(self, MeshClamp::Up);
                    } else if xy[0] == 1.0 && xy[1] == 0.0 {
                        moved = Mesh::clamp(self, MeshClamp::Right);
                    } else if xy[0] == 0.0 && xy[1] == -1.0 {
                        moved = Mesh::clamp(self, MeshClamp::Down);
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
        outer_mesh.clean_far_verts(&settings);
        println!("Far verts cleaned");
        self.append_data(&mut outer_mesh);
    }

    /// Computes faces for whole mesh with good orentation
    fn compute_faces(&mut self) {
        // get maximal coordinates
        // ┌─1─┐ 
        // 0   2 --->
        // └─3─┘ ------>
        let mut coords: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        for point in self.verts.iter() {
            let p = point.borrow();
            if p.x < coords[0] {
                coords[0] = p.x;
            } else if p.x > coords[2] {
                coords[2] = p.x;
            } else if p.y > coords[1] {
                coords[1] = p.y;
            } else if p.y < coords[3] {
                coords[3] = p.y;
            }
        }
        println!("{:?}", coords);
        let bounds = ((coords[2] - coords[0]) as usize, (coords[1] - coords[3]) as usize);
        // sort those points
        self.verts.sort_unstable_by(|a, b| {
            let a_borrowed = a.borrow();
            let b_borrowed = b.borrow();
            a_borrowed.cmp_xy(&b_borrowed)
        });
        // compute faces
        let mut faces: Vec<Face> = Vec::new();
        for x in 0..bounds.0 {
            for y in 0..bounds.1 {
                let lower_i = y * bounds.0 + x;
                let upper_i = (y + 1) * bounds.0 + x;
                faces.push(Face::new(
                    Rc::clone(&self.verts[upper_i + 1]),
                    Rc::clone(&self.verts[upper_i]),
                    Rc::clone(&self.verts[lower_i])
                ));
                faces.push(Face::new(
                    Rc::clone(&self.verts[upper_i + 1]),
                    Rc::clone(&self.verts[lower_i]),
                    Rc::clone(&self.verts[lower_i + 1])
                ));
            }
        }
        self.faces = faces;
        self.ext_dim = bounds;
    }

    /// Generate corner mesh
    fn generate_corner_from_height(dim: (u32, u32), height: f32) -> Mesh {
        // generate main part
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        for y in 0..(dim.1) {
            for x in 0..(dim.0) {
                // image axis y is positive on the way down, so we flip it
                let coords = Mesh::image_to_mesh_coords((x, y), dim);
                verts.push(Rc::new(RefCell::new(Vec3::new((coords.0, coords.1, height)))));
            }
        }
        Mesh {
            faces: Vec::new(),
            dimensions: dim,
            ext_dim: (dim.0 as usize + 1, dim.1 as usize + 1),
            verts: verts,
        }
    }

    /// Generate mesh data from given image.
    fn generate_mesh_from_img(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let dim = img.dimensions();
        // generate main part
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        for y in 0..(dim.1) {
            for x in 0..(dim.0) {
                // image axis y is positive on the way down, so we flip it
                let coords = Mesh::image_to_mesh_coords((x, y), dim);
                verts.push(Rc::new(RefCell::new(Vec3::new((
                    coords.0,
                    coords.1,
                    Mesh::compute_height(img.get_pixel(x, y).channels()[0], &settings),
                )))));
            }
        }
        Mesh {
            faces: Vec::new(),
            dimensions: dim,
            ext_dim: (dim.0 as usize, dim.1 as usize),
            verts: verts,
        }
    }

    /// Compute mesh height from given image value
    fn compute_height(pix: u8, settings: &GenSettings) -> f32 {
        (((pix as f32) / 255.0) * (settings.radius as f32) * (settings.img_height_mult))
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Mesh{
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        for vert in self.verts.iter() {
            verts.push(Rc::new(RefCell::new(vert.borrow().clone())));
        }
        Mesh {
            faces: self.faces.clone(),
            dimensions: self.dimensions,
            ext_dim: self.ext_dim,
            verts: verts,
        }
    }
}

#[derive(Debug)]
pub enum MeshClamp {
    Up,
    Down,
    Left,
    Right,
}
