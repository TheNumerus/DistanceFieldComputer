extern crate distance_field;
extern crate image;

use distance_field::extrema::Extrema;
use distance_field::mesh::Mesh;
use distance_field::settings;
use image::GenericImage;
use std::env;
use std::path::PathBuf;
use std::process;
use std::time::{Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("You haven't inputed a filename, please pass it as an argument.");
        process::exit(1);
    }
    let input = &args[1];
    println!("Input filename is {}", input);
    let img = image::open(input);
    let img = match img {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error with opening file {} :, {:?}", input, error);
            process::exit(1);
        }
    };
    println!("Image dimensions are {:?}", img.dimensions());
    let settings = settings::GenSettings::new_from_input();
    println!("Settings: {:?}", settings);
    let extrema = Extrema::get_image_extrema(&img);
    println!("Extrema: {:?}", extrema);
    let now = Instant::now();
    let mesh = Mesh::generate(&img, &settings);
    let time = now.elapsed();
    println!("Mesh generated in {}", time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9);
    let now = Instant::now();
    mesh.export("output.obj");
    let time = now.elapsed();
    println!("Mesh exported in {}", time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9);
    println!("Mesh data: {:?}", mesh.faces.iter().count());
    // separate image into buffers
    // compute buffer
    // save image
    match img.save(get_output_filename(input)) {
        Ok(_) => {
            println!("Image saved successfully");
        }
        Err(error) => {
            eprintln!("Error with saving file: {}", error);
            process::exit(1);
        }
    };
}

fn get_output_filename(input: &String) -> String {
    let path = PathBuf::from(input);
    let extension = path.extension().unwrap().to_str().unwrap();
    let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
    let index = file_name.rfind(&extension).unwrap();
    let file_name = String::from(format!(
        "{}_output.{}",
        &file_name[0..index - 1],
        &extension
    ));
    String::from(path.with_file_name(&file_name).to_str().unwrap())
}
