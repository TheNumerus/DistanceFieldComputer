extern crate image;

use image::{DynamicImage, GenericImage, Pixel};

#[derive(Debug)]
pub struct Extrema {
    pub min: u8,
    pub max: u8,
}

impl Extrema {
    pub fn get_image_extrema(img: &DynamicImage) -> Extrema {
        let mut e = Extrema { min: 255, max: 0 };
        let mut found_min = false;
        let mut found_max = false;
        for pixel in img.pixels() {
            // only get the red channel, since all images should be monochrome
            let value = pixel.2.channels()[0];
            if value > e.max {
                e.max = value
            }
            if value < e.min {
                e.min = value
            }
            if value == 255 {
                found_max = true;
            } else if value == 0 {
                found_min = true;
            }
            if found_max && found_min {
                break;
            }
        }
        e
    }

    pub fn get_capture_height(ext: &Extrema) -> u8 {
        if 255 > ext.max {
            return ext.max;
        }
        255
    }

    pub fn get_border_extrema(img: &DynamicImage) -> Extrema {
        let mut e = Extrema { min: 255, max: 0 };
        let mut found_min = false;
        let mut found_max = false;
        let dim = img.dimensions();
        // generate coord tupples
        let mut coords: Vec<(u32, u32)> = Vec::new();
        for x in 0..dim.0 {
            coords.push((x, 0));
            coords.push((x, dim.1 - 1));
        }
        for y in 1..(dim.1 - 1) {
            coords.push((0, y));
            coords.push((dim.0 - 1, y));
        }

        for (x, y) in &coords {
            // only get the red channel, since all images should be monochrome
            let value = img.get_pixel(*x, *y).channels()[0];
            if value > e.max {
                e.max = value
            }
            if value < e.min {
                e.min = value
            }
            if value == 255 {
                found_max = true;
            } else if value == 0 {
                found_min = true;
            }
            if found_max && found_min {
                break;
            }
        }
        e
    }
}
