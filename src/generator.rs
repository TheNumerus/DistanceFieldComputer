use extrema::Extrema;
use image::{ImageBuffer, Luma};
use mesh::Mesh;
use settings::{CaptureHeight, GenSettings, ImgRepeat};
use std::cmp::Ordering;
use std::f32;
use vec3::Vec3;

pub fn generate_distances(mesh: &Mesh, settings: &GenSettings, ext: &Extrema) -> Vec<f32> {
    let mut distances: Vec<f32> = Vec::with_capacity(mesh.dimensions.0 * mesh.dimensions.1);
    let capture_height = match settings.height_setting {
        CaptureHeight::Generated => {
            (ext.max as f32 / 255.0) * settings.radius as f32 * settings.img_height_mult
        }
        CaptureHeight::UserDefined(val) => {
            (val as f32 / 255.0) * settings.radius as f32 * settings.img_height_mult
        }
    };
    // get minimal distances
    let mut floor_distances: Vec<f32> = Vec::with_capacity(mesh.dimensions.0 * mesh.dimensions.1);
    for point in mesh.verts.iter() {
        let point = point.borrow();
        if point.x > 0.0 && point.x < mesh.dimensions.0 as f32 && point.y > 0.0
            && point.y < mesh.dimensions.1 as f32
        {
            floor_distances.push(point.z);
        }
    }
    println!("Floor field done, {} points", floor_distances.len());
    //generate spiral for generating distances
    let mut spiral: Vec<(isize, isize)> = Vec::new();
    match settings.repeat {
        ImgRepeat::Repeat => {
            for y in -(mesh.usable_radius as isize)..=mesh.usable_radius as isize {
                for x in -(mesh.usable_radius as isize)..=mesh.usable_radius as isize {
                    spiral.push((x, y));
                }
            }
        }
        ImgRepeat::Clamp => {
            for y in -(settings.radius.min(mesh.dimensions.0) as isize)
                ..=settings.radius.min(mesh.dimensions.0) as isize
            {
                for x in -(settings.radius.min(mesh.dimensions.1) as isize)
                    ..=settings.radius.min(mesh.dimensions.1) as isize
                {
                    spiral.push((x, y));
                }
            }
        }
    }
    spiral.sort_unstable_by(|a, b| {
        let a_sqr = (a.0 as f32).powi(2) + (a.1 as f32).powi(2);
        let b_sqr = (b.0 as f32).powi(2) + (b.1 as f32).powi(2);
        if a_sqr > b_sqr {
            Ordering::Greater
        } else if b_sqr > a_sqr {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    println!("Spiral field done, {} points", spiral.len());

    let zero_index = (mesh.ext_dim.0 + 1) * mesh.usable_radius + mesh.usable_radius;
    for y in 0..mesh.dimensions.1 {
        for x in 0..mesh.dimensions.0 {
            let capture_point = Vec3::new((x as f32 + 0.5, y as f32 + 0.5, capture_height));
            let base_dst = floor_distances[y * (mesh.dimensions.0) + x];
            let dst_to_floor = capture_height - base_dst;
            let mut dst = dst_to_floor;
            for (x_sp, y_sp) in spiral.iter() {
                let x_act = x as isize + *x_sp;
                let y_act = y as isize + *y_sp;
                if (x_act as f32 - capture_point.x).abs() > dst
                    || (y_act as f32 - capture_point.y).abs() > dst
                {
                    break;
                }
                if let ImgRepeat::Clamp = settings.repeat {
                    if x_act < 0 || x_act > mesh.dimensions.0 as isize || y_act < 0
                        || y_act > mesh.dimensions.1 as isize
                    {
                        continue;
                    }
                }
                let index = (zero_index as isize + x_act as isize
                    + ((mesh.ext_dim.0 as isize + 1) * y_act)) as usize;
                let point = &(mesh.verts[index]).borrow();
                let dst_to_point = point.distance_to(&capture_point);
                if dst_to_point < dst {
                    dst = dst_to_point;
                }
            }
            distances.push(dst);
        }
        println!("{}", y as f32 / mesh.dimensions.1 as f32);
    }
    distances
}

pub fn generate_image(dim: (usize, usize), distances: &Vec<f32>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut imgbuf = ImageBuffer::new(dim.0 as u32, dim.1 as u32);
    let mut max = 0.0;
    for dst in distances.iter() {
        if *dst > max {
            max = *dst;
        }
    }
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let index = ((dim.1 - 1) - y as usize) as usize * (dim.0) + x as usize;
        *pixel = Luma([255 - (distances[index] as f32 / max * 255.0) as u8]);
    }
    imgbuf
}
