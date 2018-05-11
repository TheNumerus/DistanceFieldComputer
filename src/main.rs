#[macro_use]
extern crate clap;
extern crate distance_field;
extern crate image;

use clap::App;
use distance_field::extrema::Extrema;
use distance_field::mesh::Mesh;
use distance_field::settings;
use image::GenericImage;
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
    let extrema = Extrema::get_image_extrema(&img);
    println!("Extrema: {:?}", extrema);
    let now = Instant::now();
    let mesh = Mesh::generate(&img, &settings);
    let time = now.elapsed();
    println!(
        "Mesh generated in {}",
        time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9
    );
    println!(
        "Faces: {:?}, Verts: {:?}",
        mesh.faces.iter().count(),
        mesh.verts.iter().count()
    );
    match matches.occurrences_of("e") {
        1 => {
            let now = Instant::now();
            mesh.export("output.obj", &settings);
            let time = now.elapsed();
            println!(
                "Mesh exported in {}",
                time.as_secs() as f64 + time.subsec_nanos() as f64 * 1e-9
            );
        },
        _ => ()
    }
    // separate image into buffers
    // compute buffer
    // save image
    match img.save(get_output_filename(&input)) {
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
