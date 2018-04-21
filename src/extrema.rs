extern crate image;

use image::{DynamicImage, GenericImage, Pixel};

#[derive(Debug)]
pub struct Extrema {
    min: u8,
    max: u8,
}

impl Extrema {
    pub fn get_image_extrema(img: &DynamicImage) -> Extrema {
        let mut e = Extrema { min: 255, max: 0 };
        for pixel in img.pixels() {
            // only get the red channel, since all images should be monochrome
            let value = pixel.2.channels4().0;
            if value > e.max {
                e.max = value
            }
            if value < e.min {
                e.min = value
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
}
