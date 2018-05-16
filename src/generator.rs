use extrema::Extrema;
use mesh::{clamp_to_range, Mesh};
use image::{ImageBuffer, Luma};
use vec3::Vec3;
use settings::{GenSettings, CaptureHeight};
use std::f32;

pub fn generate_distances(mesh: &Mesh, settings: &GenSettings, ext: &Extrema) -> Vec<f32> {
    let mut distances: Vec<f32> = Vec::new();
    let capture_height = match settings.height_setting {
        CaptureHeight::Generated => (ext.max as f32 / 255.0) * settings.radius as f32 * settings.img_height_mult,
        CaptureHeight::UserDefined(val) => (val as f32 / 255.0) * settings.radius as f32 * settings.img_height_mult
    };
    let mut floor_distances: Vec<f32> = Vec::with_capacity(mesh.dimensions.0 * mesh.dimensions.1);
    for point in mesh.verts.iter() {
        let point = point.borrow();
        if point.x > 0.0 && point.x < mesh.dimensions.0 as f32 && point.y > 0.0 && point.y < mesh.dimensions.1 as f32 {
            floor_distances.push(point.z);
        }
    }
    println!("Floor field done, {} points", floor_distances.len());
    // so far, im using only approx. algorithm which only considers points
    for y in 0..mesh.dimensions.1 {
        for x in 0..mesh.dimensions.0 {
            let capture_point = Vec3::new((x as f32 + 0.5, y as f32 + 0.5, capture_height));
            let dst_to_floor = capture_height - floor_distances[y * (mesh.dimensions.0) + x];
            let mut dst = dst_to_floor;
            for point in mesh.verts.iter() {
                let point = point.borrow();
                // early exit is possible, because points are sorted
                if point.y > (capture_point.y + dst_to_floor as f32) {
                    break;
                }
                if (point.x - capture_point.x).abs() > dst || (point.y - capture_point.y).abs() > dst {
                    continue;
                }
                if ((point.x - capture_point.x).powi(2) + (point.y - capture_point.y).powi(2)) < dst.powi(2) {
                    let dst_to_point = point.distance_to(&capture_point);
                    if dst_to_point < dst {
                        dst = dst_to_point;
                    }
                }
            }
            distances.push(dst);
        }
        println!("{}", y);
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