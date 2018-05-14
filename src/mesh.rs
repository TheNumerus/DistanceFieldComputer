use face::Face;
use image::{DynamicImage, GenericImage, Pixel};
use settings::{GenSettings, ImgRepeat};
use std::cell::RefCell;
use std::f32;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use vec3::Vec3;

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
            ImgRepeat::Repeat | ImgRepeat::Mirror => Mesh::generate_mesh(img, settings),
            ImgRepeat::Clamp => Mesh::generate_mesh_clamp(img, settings)
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
        // generate main part
        let vec_size = ((dim.0 + settings.radius + 10) * (dim.1 + settings.radius + 10)) as usize;
        let mut verts: Vec<Rc<RefCell<Vec3>>> = Vec::with_capacity(vec_size);
        // fix for images with radius bigger than image dimension
        let (x_low, x_high, y_low, y_high) = (
            (-(settings.radius as i32) - 10).max(-(dim.0 as i32)),
            (dim.0 as i32 + settings.radius as i32 + 10).min(2 * (dim.0 as i32)),
            (-(settings.radius as i32) - 10).max(-(dim.1 as i32)),
            (dim.1 as i32 + settings.radius as i32 + 10).min(2 * (dim.1 as i32))
        );
        for y in y_low..y_high {
            for x in x_low..x_high {
                // image axis y is positive on the way down, so we flip it
                let coords = match settings.repeat {
                    ImgRepeat::Repeat => Mesh::mesh_to_image_coords_repeat((x as f32 + 0.5, y as f32 + 0.5), dim),
                    ImgRepeat::Mirror => Mesh::mesh_to_image_coords_mirror((x as f32 + 0.5, y as f32 + 0.5), dim),
                    _ => panic!()
                };
                verts.push(new_vert!(
                    x as f32 + 0.5,
                    y as f32 + 0.5,
                    Mesh::compute_height(img.get_pixel(coords.0, coords.1).channels()[0], &settings)
                ));
            }
        }
        Mesh {
            faces: Vec::new(),
            dimensions: dim,
            ext_dim: dim,
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

    fn mesh_to_image_coords_repeat(coords:(f32, f32), dim: (usize, usize)) -> (u32, u32) {
        let x = if coords.0 < 0.0 {
            (coords.0 - 0.5 + (dim.0 as f32)) as u32
        } else if coords.0 > (dim.0 as f32) {
            (coords.0 - 0.5 - (dim.0 as f32)) as u32
        } else {
            (coords.0 - 0.5) as u32
        };
        let y = if coords.1 < 0.0 {
            (coords.1 - 0.5 + (dim.1 as f32)) as u32
        } else if coords.1 > (dim.1 as f32) {
            (coords.1 - 0.5 - (dim.1 as f32)) as u32
        } else {
            (coords.1 - 0.5) as u32
        };
        let y = (dim.1 - 1) as u32 - y;
        (x,y)
    }

    fn mesh_to_image_coords_mirror(coords:(f32, f32), dim: (usize, usize)) -> (u32, u32) {
        let x = if coords.0 < 0.0 {
            (coords.0.abs() - 0.5) as u32
        } else if coords.0 > (dim.0 as f32) {
            (-coords.0 + (2 * dim.0) as f32 - 0.5) as u32
        } else {
            (coords.0 - 0.5) as u32
        };
        let y = if coords.1 < 0.0 {
            (coords.1.abs() - 0.5) as u32
        } else if coords.1 > (dim.1 as f32) {
            (-coords.1 + (2 * dim.1) as f32 - 0.5) as u32
        } else {
            (coords.1 - 0.5) as u32
        };
        let y = (dim.1 - 1) as u32 - y;
        (x,y)
    }

    /// Computes faces for whole mesh with good orentation
    fn compute_faces(&mut self, settings: &GenSettings) {
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
