#[macro_use]
extern crate clap;
extern crate distance_field;
extern crate image;

use clap::App;
use distance_field::extrema::Extrema;
use distance_field::generator;
use distance_field::mesh::Mesh;
use distance_field::settings;
use image::{GenericImage, ImageLuma8};
use std::io;
use std::path::PathBuf;
use std::process;
use std::time::Instant;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut whatever = String::from("");
    let input = String::from(matches.value_of("INPUT").unwrap());
    println!("Input filename is {}", &input);
    let img = image::open(&input);
    let img = match img {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error with opening file {} :, {:?}", &input, error);
            io::stdin().read_line(&mut whatever).unwrap();
            process::exit(1);
        }
    };
    println!("Image dimensions are {:?}", img.dimensions());
    let settings = settings::GenSettings::new_from_input(&matches);
    println!("Settings: {:?}", settings);
    let now = Instant::now();
    let mesh = Mesh::generate(&img, &settings);
    let time = now.elapsed();
    println!("Mesh generated in {}", time.as_secs() as f64 + f64::from(time.subsec_nanos()) * 1e-9);
    println!("Verts: {:?}", mesh.verts.iter().count());
    let ext = Extrema::get_image_extrema(&img);
    let now = Instant::now();
    let distances = generator::generate_distances(&mesh, &settings, &ext);
    let time = now.elapsed();
    println!("Distances computed in {}", time.as_secs() as f64 + f64::from(time.subsec_nanos()) * 1e-9);
    if let 1 = matches.occurrences_of("export") {
        let now = Instant::now();
        mesh.export("output.obj", &settings);
        let time = now.elapsed();
        println!("Mesh exported in {}", time.as_secs() as f64 + f64::from(time.subsec_nanos()) * 1e-9);
    }
    // separate image into buffers
    // compute buffer
    // save image
    let out_img = generator::generate_image(mesh.dimensions, &distances);
    match ImageLuma8(out_img).save(get_output_filename(&input)) {
        Ok(_) => {
            println!("Image saved successfully");
        }
        Err(error) => {
            eprintln!("Error with saving file: {}", error);
            io::stdin().read_line(&mut whatever).unwrap();
            process::exit(1);
        }
    };
}

fn get_output_filename(input: &str) -> String {
    let path = PathBuf::from(input);
    let extension = path.extension().unwrap().to_str().unwrap();
    let file_name = String::from(path.file_name().unwrap().to_str().unwrap());
    let index = file_name.rfind(&extension).unwrap();
    let file_name = format!("{}_output.{}", &file_name[0..index - 1], &extension);
    String::from(path.with_file_name(&file_name).to_str().unwrap())
}
