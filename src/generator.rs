use extrema::Extrema;
use image::{ImageBuffer, Luma};
use mesh::Mesh;
use rayon::prelude::*;
use settings::{CaptureHeight, GenSettings, ImgRepeat};
use std::cmp::Ordering;
use std::f32;
use vec3::Vec3;

pub fn generate_distances(mesh: &Mesh, settings: &GenSettings, ext: &Extrema) -> Vec<Dist> {
    let capture_height = match settings.height_setting {
        CaptureHeight::Generated => (ext.max as f32 / 255.0) * settings.radius as f32 * settings.img_height_mult,
        CaptureHeight::UserDefined(val) => (val as f32 / 255.0) * settings.radius as f32 * settings.img_height_mult,
    };
    //generate spiral for generating distances
    let mut spiral: Vec<(isize, isize)> = Vec::new();
    for y in -(mesh.usable_radius as isize)..=mesh.usable_radius as isize {
        for x in -(mesh.usable_radius as isize)..=mesh.usable_radius as isize {
            spiral.push((x, y));
        }
    }
    spiral.sort_unstable_by(|a, b| {
        let a_sqr = (a.0 * a.0) + (a.1 * a.1);
        let b_sqr = (b.0 * b.0) + (b.1 * b.1);
        if a_sqr > b_sqr {
            Ordering::Greater
        } else if b_sqr > a_sqr {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    while spiral.len() % 8 != 0 {
        spiral.pop();
    }
    println!("Spiral field done, {} points", spiral.len());
    
    let zero_index = match settings.repeat {
        ImgRepeat::Repeat => ((mesh.ext_dim.0 + 1) * mesh.usable_radius + mesh.usable_radius) as isize,
        ImgRepeat::Clamp => (mesh.ext_dim.0 + 2) as isize,
    };
    
    let get_distance = |x: isize, y: isize| {
        let capture_point = Vec3::new((x as f32 + 0.5, y as f32 + 0.5, capture_height));
        let mut dst = f32::MAX;
        if let ImgRepeat::Clamp = settings.repeat {
            for (x_sp, y_sp) in &spiral {
                let x_act = x + *x_sp;
                let y_act = y + *y_sp;
                if x_act < 0 || x_act > mesh.dimensions.0 as isize || y_act < 0 || y_act > mesh.dimensions.1 as isize {
                    continue;
                }
                if (x_act as f32 - capture_point.x).abs() > dst || (y_act as f32 - capture_point.y).abs() > dst {
                    break;
                }
                let index = (zero_index + x_act + ((mesh.ext_dim.0 as isize + 1) * y_act)) as usize;
                let point = &(mesh.verts[index]);
                let dst_to_point = point.distance_to(&capture_point);
                if dst_to_point < dst {
                    dst = dst_to_point;
                }
            }
        } else if let ImgRepeat::Repeat = settings.repeat {
            for (x_sp, y_sp) in &spiral {
                let x_act = x + *x_sp;
                let y_act = y + *y_sp;
                if (x_act as f32 - capture_point.x).abs() > dst || (y_act as f32 - capture_point.y).abs() > dst {
                    break;
                }
                let index = (zero_index + x_act + ((mesh.ext_dim.0 as isize + 1) * y_act)) as usize;
                let point = &(mesh.verts[index]);
                let dst_to_point = point.distance_to(&capture_point);
                if dst_to_point < dst {
                    dst = dst_to_point;
                }
            }
        }
        dst
    };

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "avx2")]
    unsafe fn get_distance_avx2_repeat(x: isize, y: isize, capture_height: f32, spiral: &Vec<(isize, isize)>, mesh: &Mesh, zero_index: isize) -> f32 {
        let capture_point = Vec3::new((x as f32 + 0.5, y as f32 + 0.5, capture_height));
        let mut dst = f32::MAX;
        for (x_sp, y_sp) in spiral {
            let x_act = x + *x_sp;
            let y_act = y + *y_sp;
            if (x_act as f32 - capture_point.x).abs() > dst || (y_act as f32 - capture_point.y).abs() > dst {
                break;
            }
            let index = (zero_index + x_act + ((mesh.ext_dim.0 as isize + 1) * y_act)) as usize;
            let point = &(mesh.verts[index]);
            let dst_to_point = point.distance_to(&capture_point);
            if dst_to_point < dst {
                dst = dst_to_point;
            }
        }
        //println!("{:?}", dst);
        dst
    };

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "avx2")]
    unsafe fn get_distance_avx2_clamp(x: isize, y: isize, capture_height: f32, spiral: &Vec<(isize, isize)>, mesh: &Mesh, zero_index: isize) -> f32 {
        let capture_point = Vec3::new((x as f32 + 0.5, y as f32 + 0.5, capture_height));
        let mut dst = f32::MAX;
        for (x_sp, y_sp) in spiral {
            let x_act = x + *x_sp;
            let y_act = y + *y_sp;
            if x_act < 0 || x_act > mesh.dimensions.0 as isize || y_act < 0 || y_act > mesh.dimensions.1 as isize {
                continue;
            }
            if (x_act as f32 - capture_point.x).abs() > dst || (y_act as f32 - capture_point.y).abs() > dst {
                break;
            }
            let index = (zero_index + x_act + ((mesh.ext_dim.0 as isize + 1) * y_act)) as usize;
            let point = &(mesh.verts[index]);
            let dst_to_point = point.distance_to(&capture_point);
            if dst_to_point < dst {
                dst = dst_to_point;
            }
        }
        dst
    };

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "avx2")]
    let get_distance_avx2 = |x: isize, y: isize| {
        match settings.repeat {
            ImgRepeat::Clamp => {
                unsafe { get_distance_avx2_clamp(x, y, capture_height, &spiral, &mesh, zero_index) }
            },
            ImgRepeat::Repeat => {
                unsafe { get_distance_avx2_repeat(x, y, capture_height, &spiral, &mesh, zero_index) }
            }
        }
    };

    // generate chunks
    let mut chunks: Vec<Vec<(isize, isize)>> = vec!();
    for x in 0..((mesh.dimensions.0 as f32 / 64.0).ceil() as isize) {
        for y in 0..((mesh.dimensions.1 as f32 / 64.0).ceil() as isize) {
            let mut chunk = vec!();
            for px in (x * 64)..(x * 64 + 64) {
                for py in (y * 64)..(y * 64 + 64) {
                    chunk.push((px, py))
                }
            }
            chunks.push(chunk);
        }
    }

    // generate distances
    let mut distances: Vec<Dist> = chunks.par_iter().map(|chunk| {
        let mut distances: Vec<Dist> = vec!();
        for coords in chunk {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                if is_x86_feature_detected!("avx2") {
                    distances.push(Dist{x: coords.0, y: coords.1, dst: get_distance_avx2(coords.0, coords.1)});
                    continue;
                }
            }
            distances.push(Dist{x: coords.0, y: coords.1, dst: get_distance(coords.0, coords.1)});
        }
        distances
    }).flatten().collect();
    
    // sort distances
    distances.sort_by(|a,b| {
        if a.y > b.y {
            Ordering::Greater
        } else if a.y == b.y {
            if a.x > b.x {
                Ordering::Greater
            } else if a.x == b.x {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        }
    });
    distances
}

pub fn generate_image(dim: (usize, usize), distances: &Vec<Dist>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut imgbuf = ImageBuffer::new(dim.0 as u32, dim.1 as u32);
    let max = distances.par_iter().max_by(|x, y| {
        if x.dst > y.dst {
            return Ordering::Greater
        }
        Ordering::Less
    }).unwrap().dst;
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = ((dim.1 - 1) - y as usize) as usize * (dim.0) + x as usize;
        *pixel = Luma([255 - (distances[index].dst as f32 / max * 255.0) as u8]);
    }
    imgbuf
}

pub struct Dist {
    pub x: isize,
    pub y: isize,
    pub dst: f32
}