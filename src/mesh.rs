use face::Face;
use image::{DynamicImage, GenericImage, Pixel};
use settings::{GenSettings, ImgRepeat};
use std::cell::RefCell;
use std::f32;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use vec3::Vec3;

const MOVES: [([f32; 2], Dir); 8] = [
    ([-1.0, 0.0], Dir::Left),
    ([-1.0, 1.0], Dir::LeftUp),
    ([0.0, 1.0], Dir::Up),
    ([1.0, 1.0], Dir::RightUp),
    ([1.0, 0.0], Dir::Right),
    ([1.0, -1.0], Dir::RightDown),
    ([0.0, -1.0], Dir::Down),
    ([-1.0, -1.0], Dir::LeftDown),
];
const EXPORT_SCALE: f32 = 1.0 / 100.0;

#[macro_export]
macro_rules! new_vert {
    ($x:expr, $y:expr, $z:expr) => (
        Rc::new(RefCell::new(Vec3::new((
            $x,
            $y,
            $z
        ))))
    )
}

#[derive(Debug)]
pub struct Mesh {
    pub faces: Vec<Face>,
    dimensions: (usize, usize),
    ext_dim: (usize, usize),
    pub verts: Vec<Rc<RefCell<Vec3>>>,
}

impl Mesh {
    /// Main function for generating the whole mesh
    pub fn generate(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let mut middle = match settings.repeat {
            ImgRepeat::Clamp => Mesh::generate_mesh_clamp(img, settings),
            _ => Mesh::generate_mesh_from_img(img, settings)
        };
        println!("Middle part generated");
        match settings.repeat {
            ImgRepeat::Clamp => (),
            _ => middle.compute_out_of_range_mesh(settings)
        }
        middle.compute_faces(settings);
        middle
    }

    /// Export mesh data in obj. format.
    pub fn export(&self, filename: &str, settings: &GenSettings) {
        let mut file = File::create(filename).unwrap();
        let mut data = String::from("");
        for point in self.verts.iter() {
            let p = point.borrow();
            data.push_str(&format!(
                "v {} {} {}\n",
                p.x * EXPORT_SCALE,
                p.y * EXPORT_SCALE,
                p.z * EXPORT_SCALE
            ));
        }
        println!("All points written.");
        let points_in_row = match settings.repeat {
            ImgRepeat::Clamp => (self.dimensions.0 + 2) as usize,
            _ => self.ext_dim.0 + 1,
        };
        println!("{:?}", points_in_row);
        for (i, _face) in self.faces.iter().enumerate() {
            let row = (i / 2) / (points_in_row - 1);
            let pos_in_row = i - row * (points_in_row - 1) * 2;
            match i % 2 {
                0 => {
                    data.push_str(&format!(
                        "f {} {} {}\n",
                        pos_in_row / 2 + (row + 1) * (points_in_row) + 2,
                        pos_in_row / 2 + (row + 1) * (points_in_row) + 1,
                        pos_in_row / 2 + row * (points_in_row) + 1,
                    ));
                }
                1 => {
                    data.push_str(&format!(
                        "f {} {} {}\n",
                        pos_in_row / 2 + (row + 1) * (points_in_row) + 2,
                        pos_in_row / 2 + row * (points_in_row) + 1,
                        pos_in_row / 2 + row * (points_in_row) + 2
                    ));
                }
                _ => (),
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
    fn flipped(&self, axis: (bool, bool), settings: &GenSettings, dir: &Dir) -> Mesh {
        let cut = 10.0;
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        let rad = settings.radius as f32;
        let dim = (self.dimensions.0 as f32, self.dimensions.1 as f32);
        let (x_low, x_high, y_low, y_high) =
            (rad + cut, dim.0 - rad - cut, rad + cut, dim.1 - rad - cut);
        // ┌─3─┐
        // 0   1
        // └─2─┘
        let borders: (f32, f32, f32, f32) = match dir {
            Dir::Right => (x_high, dim.0, 0.0, dim.1),
            Dir::RightDown => (x_high, dim.0, 0.0, y_low),
            Dir::Down => (0.0, dim.0, 0.0, y_low),
            Dir::LeftDown => (0.0, x_low, 0.0, y_low),
            Dir::Left => (0.0, x_low, 0.0, dim.1),
            Dir::LeftUp => (0.0, x_low, y_high, dim.1),
            Dir::Up => (0.0, dim.0, y_high, dim.1),
            Dir::RightUp => (x_high, dim.0, y_high, dim.1),
        };
        for vert in self.verts.iter() {
            let vert = vert.borrow();
            let x = vert.x < borders.1 && vert.x > borders.0;
            let y = vert.y < borders.3 && vert.y > borders.2;
            if x && y {
                let mut v = vert.clone();
                if axis.0 {
                    v.x = dim.0 - v.x;
                }
                if axis.1 {
                    v.y = dim.1 - v.y;
                }
                verts.push(Rc::new(RefCell::new(v)));
            }
        }
        Mesh {
            faces: Vec::new(),
            dimensions: self.dimensions,
            ext_dim: self.ext_dim,
            verts: verts,
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

    /// Returns cut version of the mesh.
    fn cutted(&self, settings: &GenSettings, dir: &Dir) -> Mesh {
        let cut = 10.0;
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::new();
        let rad = settings.radius as f32;
        let dim = (self.dimensions.0 as f32, self.dimensions.1 as f32);
        let (x_low, x_high, y_low, y_high) =
            (rad + cut, dim.0 - rad - cut, rad + cut, dim.1 - rad - cut);
        // ┌─3─┐
        // 0   1
        // └─2─┘
        let borders: (f32, f32, f32, f32) = match dir {
            Dir::Left => (x_high, dim.0, 0.0, dim.1),
            Dir::LeftUp => (x_high, dim.0, 0.0, y_low),
            Dir::Up => (0.0, dim.0, 0.0, y_low),
            Dir::RightUp => (0.0, x_low, 0.0, y_low),
            Dir::Right => (0.0, x_low, 0.0, dim.1),
            Dir::RightDown => (0.0, x_low, y_high, dim.1),
            Dir::Down => (0.0, dim.0, y_high, dim.1),
            Dir::LeftDown => (x_high, dim.0, y_high, dim.1),
        };
        for vert in self.verts.iter() {
            let v = vert.borrow();
            let x = v.x < borders.1 && v.x > borders.0;
            let y = v.y < borders.3 && v.y > borders.2;
            if x && y {
                verts.push(Rc::new(RefCell::new(vert.borrow().clone())));
            }
        }
        Mesh {
            faces: Vec::new(),
            dimensions: self.dimensions,
            ext_dim: self.ext_dim,
            verts: verts,
        }
    }

    fn generate_mesh_clamp(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let dim = img.dimensions();
        let dim = (dim.0 as usize, dim.1 as usize);
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::with_capacity((dim.0 + 2) * (dim.1 + 2));
        // generate points in this order, so we don't have to sort them later
        // 7 │ 8 │ 9
        //───┼───┼───
        // 4 │ 5 │ 6
        //───┼───┼───
        // 1 │ 2 │ 3

        // bottom left
        let corner_coords = (
            -((settings.radius + 10) as f32) + 0.5,
            -((settings.radius + 10) as f32) + 0.5
        );
        let img_corner_coords = Mesh::mesh_to_image_coords_clamped(corner_coords, dim);
        verts.push(new_vert!(
            corner_coords.0,
            corner_coords.1,
            Mesh::compute_height(img.get_pixel(img_corner_coords.0, img_corner_coords.1).channels()[0], &settings)
        ));

        // bottom center
        for x in 0..(dim.0) {
            let coords = Mesh::mesh_to_image_coords_clamped((x as f32 + 0.5, 0.5), dim);
            verts.push(new_vert!(
                x as f32 + 0.5,
                -((settings.radius + 10) as f32) + 0.5,
                Mesh::compute_height(img.get_pixel(coords.0, coords.1).channels()[0], &settings)
            ));
        }

        // bottom right
        let corner_coords = (
            (dim.0 + settings.radius + 10) as f32 - 0.5,
            -((settings.radius + 10) as f32) + 0.5
        );
        let img_corner_coords = Mesh::mesh_to_image_coords_clamped(corner_coords, dim);
        verts.push(new_vert!(
            corner_coords.0,
            corner_coords.1,
            Mesh::compute_height(img.get_pixel(img_corner_coords.0, img_corner_coords.1).channels()[0], &settings)
        ));

        // center row
        for y in 0..(dim.1) {
            // center left
            let coords = (
                -((settings.radius + 10) as f32) + 0.5,
                (y as f32) + 0.5
            );
            let img_coords = Mesh::mesh_to_image_coords_clamped(coords, dim);
            verts.push(new_vert!(
                coords.0,
                coords.1,
                Mesh::compute_height(img.get_pixel(img_coords.0, img_coords.1).channels()[0], &settings)
            ));

            // center
            for x in 0..(dim.0) {
                let img_coords = Mesh::mesh_to_image_coords_clamped((x as f32 + 0.5, y as f32 + 0.5), dim);
                verts.push(new_vert!(
                    x as f32 + 0.5,
                    y as f32 + 0.5,
                    Mesh::compute_height(img.get_pixel(img_coords.0, img_coords.1).channels()[0], &settings)
                ));
            }

            // center right
            let coords = (
                (dim.0 + settings.radius + 10) as f32 - 0.5,
                (y as f32) + 0.5
            );
            let img_coords = Mesh::mesh_to_image_coords_clamped(coords, dim);
            verts.push(new_vert!(
                coords.0,
                coords.1,
                Mesh::compute_height(img.get_pixel(img_coords.0, img_coords.1).channels()[0], &settings)
            ));
        }

        // top left
        let corner_coords = (
            -((settings.radius + 10) as f32) + 0.5,
            (dim.1 + settings.radius + 10) as f32 - 0.5,
        );
        let img_corner_coords = Mesh::mesh_to_image_coords_clamped(corner_coords, dim);
        verts.push(new_vert!(
            corner_coords.0,
            corner_coords.1,
            Mesh::compute_height(img.get_pixel(img_corner_coords.0, img_corner_coords.1).channels()[0], &settings)
        ));

        // top center
        for x in 0..(dim.0) {
            let coords = Mesh::mesh_to_image_coords_clamped((x as f32 + 0.5, dim.1 as f32 - 0.5), dim);
            verts.push(new_vert!(
                x as f32 + 0.5,
                (dim.1 + settings.radius + 10) as f32 - 0.5,
                Mesh::compute_height(img.get_pixel(coords.0, coords.1).channels()[0], &settings)
            ));
        }

        // top right
        let corner_coords = (
            (dim.0 + settings.radius + 10) as f32 - 0.5,
            (dim.1 + settings.radius + 10) as f32 - 0.5,
        );
        let img_corner_coords = Mesh::mesh_to_image_coords_clamped(corner_coords, dim);
        verts.push(new_vert!(
            corner_coords.0,
            corner_coords.1,
            Mesh::compute_height(img.get_pixel(img_corner_coords.0, img_corner_coords.1).channels()[0], &settings)
        ));

        Mesh {
            faces: Vec::new(),
            dimensions: dim,
            ext_dim: (dim.0 + 2, dim.1 + 2),
            verts: verts,
        }
    }

    fn mesh_to_image_coords_clamped(coords:(f32, f32), dim: (usize, usize)) -> (u32, u32) {
        let x = clamp_to_range(coords.0 - 0.5, 0.0, (dim.0 - 1) as f32) as u32;
        let y = (-clamp_to_range(coords.1, 0.0, (dim.1 - 1) as f32) + (dim.1 as f32 - 0.5)) as u32;
        (x,y)
    }

    /// Merge two meshes together.
    fn append_data(&mut self, other: &mut Mesh) {
        self.faces.append(&mut other.faces);
        self.verts.append(&mut other.verts);
    }

    /// Generate mesh data out of given image frame.
    fn compute_out_of_range_mesh(&mut self, settings: &GenSettings) {
        let mut outer_mesh: Mesh = match settings.repeat {
            ImgRepeat::Repeat => {
                let mut outer = Mesh::empty_copy(&self);
                for mov in MOVES.iter() {
                    let (xy, dir) = mov;
                    let mut moved = self.cutted(&settings, dir);
                    moved.translate(Vec3::new((
                        self.dimensions.0 as f32 * xy[0],
                        self.dimensions.1 as f32 * xy[1],
                        0.0,
                    )));
                    outer.append_data(&mut moved);
                }
                outer
            }
            ImgRepeat::Clamp => panic!("Wrong option, cannot even happen anyway"),
            ImgRepeat::Mirror => {
                let mut outer = Mesh::empty_copy(&self);
                for (xy, dir) in MOVES.iter() {
                    let mut moved: Mesh;
                    if f32::abs(xy[0]) + f32::abs(xy[1]) == 2.0 {
                        moved = self.flipped((true, true), &settings, dir);
                    } else if f32::abs(xy[0]) == 1.0 && f32::abs(xy[1]) == 0.0 {
                        moved = self.flipped((true, false), &settings, dir);
                    } else {
                        moved = self.flipped((false, true), &settings, dir);
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
        self.append_data(&mut outer_mesh);
    }

    /// Computes faces for whole mesh with good orentation
    fn compute_faces(&mut self, settings: &GenSettings) {
        // sort all points, no need for sorting clamped meshes
        match settings.repeat {
            ImgRepeat::Clamp => (),
            _ => {
                self.verts.sort_unstable_by(|a, b| {
                    let a_borrowed = a.borrow();
                    let b_borrowed = b.borrow();
                    a_borrowed.cmp_xy(&b_borrowed)
                });
            }
        };
        // get maximal coordinates
        // ┌─1─┐
        // 0   2
        // └─3─┘
        let coords: [f32; 4] = [
            self.verts[0].borrow().x,
            self.verts[self.verts.len() - 1].borrow().y,
            self.verts[self.verts.len() - 1].borrow().x,
            self.verts[0].borrow().y,
        ];
        println!("{:?}", coords);
        let bounds: (usize, usize) = match settings.repeat {
            ImgRepeat::Clamp => {
                ((self.dimensions.0 + 1) as usize, (self.dimensions.1 + 1) as usize)
            },
            _ => {
            ((coords[2] - coords[0]) as usize, (coords[1] - coords[3]) as usize)
            }
        };
        // compute faces
        let mut faces: Vec<Face> = Vec::with_capacity(bounds.0 * bounds.1 * 2);
        for x in 0..bounds.0 {
            for y in 0..bounds.1 {
                let lower_i = y * bounds.0 + x;
                let upper_i = (y + 1) * bounds.0 + x;
                faces.push(Face::new(
                    Rc::clone(&self.verts[upper_i + 1]),
                    Rc::clone(&self.verts[upper_i]),
                    Rc::clone(&self.verts[lower_i]),
                ));
                faces.push(Face::new(
                    Rc::clone(&self.verts[upper_i + 1]),
                    Rc::clone(&self.verts[lower_i]),
                    Rc::clone(&self.verts[lower_i + 1]),
                ));
            }
        }
        self.faces = faces;
        self.ext_dim = bounds;
    }

    /// Generate mesh data from given image.
    fn generate_mesh_from_img(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let dim = img.dimensions();
        // generate main part
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::with_capacity((dim.0 * dim.1) as usize);
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
            dimensions: (dim.0 as usize, dim.1 as usize),
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
    fn clone(&self) -> Mesh {
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

fn clamp_to_range(val: f32, min: f32, max: f32) -> f32 {
    val.min(max).max(min)
}

#[derive(Debug)]
pub enum Dir {
    Left,
    LeftUp,
    Up,
    RightUp,
    Right,
    RightDown,
    Down,
    LeftDown,
}
