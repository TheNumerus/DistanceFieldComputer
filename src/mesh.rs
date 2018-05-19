use extrema::Extrema;
use face::Face;
use image::{DynamicImage, GenericImage, Pixel};
use settings::{CaptureHeight, GenSettings, ImgRepeat};
use std::cell::RefCell;
use std::f32;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use vec3::Vec3;

const EXPORT_SCALE: f32 = 1.0 / 100.0;

#[macro_export]
macro_rules! new_vert {
    ($x:expr, $y:expr, $z:expr) => {
        Rc::new(RefCell::new(Vec3::new(($x, $y, $z))))
    };
}

#[derive(Debug)]
pub struct Mesh {
    pub faces: Vec<Face>,
    pub dimensions: (usize, usize),
    pub ext_dim: (usize, usize),
    pub usable_radius: usize,
    pub verts: Vec<Rc<RefCell<Vec3>>>,
}

impl Mesh {
    /// Main function for generating the whole mesh
    pub fn generate(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        // generate points in this order, so we don't have to sort them later
        // 7 │ 8 │ 9
        //───┼───┼───
        // 4 │ 5 │ 6
        //───┼───┼───
        // 1 │ 2 │ 3
        let mut middle = match settings.repeat {
            ImgRepeat::Repeat => Mesh::generate_mesh(img, settings),
            ImgRepeat::Clamp => Mesh::generate_mesh_clamp(img, settings),
        };
        println!("Points generated");
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

    /// Generate mesh data from given image.
    fn generate_mesh(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let dim = img.dimensions();
        let dim = (dim.0 as usize, dim.1 as usize);

        // get maximal usable radius
        let ext = Extrema::get_border_extrema(&img);
        let height = match settings.height_setting {
            CaptureHeight::Generated => ext.max,
            CaptureHeight::UserDefined(val) => val,
        };
        let max_radius = (settings.radius as f32 * ((height - ext.min) as f32 / 255.0)
            * settings.img_height_mult) as usize;
        let max_radius = max_radius.min(settings.radius);

        let vec_size = (dim.0 + max_radius) * (dim.1 + max_radius);
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::with_capacity(vec_size);

        // fix for images with radius bigger than image dimension
        let (x_low, x_high, y_low, y_high) = (
            (-(max_radius as i32)).max(-(dim.0 as i32)),
            (dim.0 as i32 + max_radius as i32).min(2 * (dim.0 as i32)),
            (-(max_radius as i32)).max(-(dim.1 as i32)),
            (dim.1 as i32 + max_radius as i32).min(2 * (dim.1 as i32)),
        );
        for y in y_low..y_high {
            for x in x_low..x_high {
                let coords =
                    Mesh::mesh_to_image_coords_repeat((x as f32 + 0.5, y as f32 + 0.5), dim);
                verts.push(new_vert!(
                    x as f32 + 0.5,
                    y as f32 + 0.5,
                    Mesh::compute_height(
                        img.get_pixel(coords.0, coords.1).channels()[0],
                        &settings
                    )
                ));
            }
        }
        Mesh {
            faces: Vec::new(),
            dimensions: dim,
            ext_dim: (dim.0 + 2 * max_radius, dim.1 + 2 * max_radius),
            verts: verts,
            usable_radius: max_radius.min((dim.0).min(dim.1)),
        }
    }

    fn generate_mesh_clamp(img: &DynamicImage, settings: &GenSettings) -> Mesh {
        let dim = img.dimensions();
        let dim = (dim.0 as usize, dim.1 as usize);
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::with_capacity((dim.0 + 2) * (dim.1 + 2));
        for y in -1..=(dim.1 as isize) {
            for x in -1..=(dim.0 as isize) {
                let img_coords =
                    Mesh::mesh_to_image_coords_clamped((x as f32 + 0.5, y as f32 + 0.5), dim);
                verts.push(new_vert!(
                    x as f32 + 0.5,
                    y as f32 + 0.5,
                    Mesh::compute_height(
                        img.get_pixel(img_coords.0, img_coords.1).channels()[0],
                        &settings
                    )
                ));
            }
        }
        Mesh {
            faces: Vec::new(),
            dimensions: dim,
            ext_dim: (dim.0 + 2, dim.1 + 2),
            verts: verts,
            usable_radius: 1,
        }
    }

    fn mesh_to_image_coords_clamped(coords: (f32, f32), dim: (usize, usize)) -> (u32, u32) {
        let x = clamp_to_range(coords.0 - 0.5, 0.0, (dim.0 - 1) as f32) as u32;
        let y = (-clamp_to_range(coords.1, 0.0, (dim.1 - 1) as f32) + (dim.1 as f32 - 0.5)) as u32;
        (x, y)
    }

    fn mesh_to_image_coords_repeat(coords: (f32, f32), dim: (usize, usize)) -> (u32, u32) {
        let x = if coords.0 > 0.0 && coords.0 < (dim.0 as f32) {
            (coords.0 - 0.5) as u32
        } else if coords.0 > (dim.0 as f32) {
            (coords.0 - 0.5 - (dim.0 as f32)) as u32
        } else {
            (coords.0 - 0.5 + (dim.0 as f32)) as u32
        };
        let y = if coords.1 > 0.0 && coords.1 < (dim.1 as f32) {
            (coords.1 - 0.5) as u32
        } else if coords.1 > (dim.1 as f32) {
            (coords.1 - 0.5 - (dim.1 as f32)) as u32
        } else {
            (coords.1 - 0.5 + (dim.1 as f32)) as u32
        };
        let y = (dim.1 - 1) as u32 - y;
        (x, y)
    }

    /// Computes faces for whole mesh with good orentation
    fn compute_faces(&mut self, settings: &GenSettings) {
        let bounds: (usize, usize) = match settings.repeat {
            ImgRepeat::Clamp => (self.dimensions.0 + 1, self.dimensions.1 + 1),
            _ => (
                self.dimensions.0 - 1 + 2 * self.usable_radius,
                self.dimensions.1 - 1 + 2 * self.usable_radius,
            ),
        };
        // compute faces
        println!("{:?}", bounds);
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
            usable_radius: self.usable_radius,
        }
    }
}

pub fn clamp_to_range(val: f32, min: f32, max: f32) -> f32 {
    val.min(max).max(min)
}
