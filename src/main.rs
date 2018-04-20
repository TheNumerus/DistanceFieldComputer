extern crate image;

use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::env;
use std::process;
use image::GenericImage;

const DEF_RADIUS: u32 = 64;

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
        },
    };
    println!("Image dimensions are {:?}", img.dimensions());
    let settings = GenSettings::new_from_input();
    println!("{:?}", settings);
    // separate image into buffers
    // compute buffer
    // save image
    let ref mut out_img = File::create(get_output_filename(input)).unwrap();
    match img.save(out_img, image::PNG) {
        Ok(_) => {
            println!("Image saved successfully");
            },
        Err(error) => {
            eprintln!("Error with saving file: {}", error);
            process::exit(1);
        }
    };
}

fn get_output_filename(input: &String) -> String {
    let path = PathBuf::from(input);
    let extension = path.extension().unwrap().to_str().unwrap();
    let file_name =  String::from(path.file_name().unwrap().to_str().unwrap());
    let index = file_name.rfind(&extension).unwrap();
    let file_name = String::from(format!("{}_output.{}", &file_name[0..index-1], &extension));
    String::from(path.with_file_name(&file_name).to_str().unwrap())
}

#[derive(Debug)]
enum ImgRepeat {
    Repeat,
    Clamp,
    Mirror
}

#[derive(Debug)]
enum CaptureHeight {
    UserDefined(u8),
    Generated
}

#[derive(Debug)]
struct GenSettings {
    radius: u32,
    repeat: ImgRepeat,
    height_setting: CaptureHeight,
    img_height_mult: f32
}

impl GenSettings {
    fn new_from_input() -> GenSettings{
        let radius = GenSettings::get_radius_input();
        let repeat = GenSettings::get_repeat_input();
        let height_setting = GenSettings::get_height_input();
        let height_mult = GenSettings::get_height_mult();
        GenSettings::new_from_values((radius, repeat, height_setting, height_mult))
    }

    fn new_from_values(values: (u32, ImgRepeat, CaptureHeight, f32)) -> GenSettings{
        GenSettings{
            radius: values.0,
            repeat: values.1,
            height_setting: values.2,
            img_height_mult: values.3
        }
    }

    fn get_radius_input() -> u32 {
        let mut radius = String::new();
        println!("Please input search radius (preferably power of two), default is {}.", DEF_RADIUS);
        io::stdin().read_line(&mut radius).expect("Failed to read input");
        match radius.trim().len() {
            0 => {
                println!("Setting {} as a radius.", DEF_RADIUS);
                DEF_RADIUS
            },
            _ => match radius.trim().parse::<u32>() {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Invalid number, setting {} as a radius", DEF_RADIUS);
                    DEF_RADIUS
                }
            }
        }
    }

    fn get_repeat_input() -> ImgRepeat {
        println!("Please input image repeat option, default is Repeat.");
        println!("1 - Repeat, 2 - Clamp, 3 - Mirror");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim().len() == 0 {
            println!("Setting Repeat.");
            return ImgRepeat::Repeat
        }
        match input.trim().parse::<u8>() {
            Ok(value) => {
                match value {
                    1 => ImgRepeat::Repeat,
                    2 => ImgRepeat::Clamp,
                    3 => ImgRepeat::Mirror,
                    _ => {
                        eprintln!("Invalid option, setting Repeat");
                        ImgRepeat::Repeat
                    }
                }
            },
            Err(_) => {
                eprintln!("Invalid input, setting Repeat");
                ImgRepeat::Repeat
            }
        }
    }

    fn get_height_input() -> CaptureHeight {
        println!("Please input capture height, or press enter for automatic computation.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim().len() == 0 {
            println!("Setting automatic.");
            return CaptureHeight::Generated
        }
        match input.trim().parse::<u8>() {
            Ok(value) => CaptureHeight::UserDefined(value),
            Err(_) => {
                eprintln!("Invalid input, setting automatic");
                CaptureHeight::Generated
            }
        }
    }

    fn get_height_mult() -> f32 {
        println!("Please input height multipiler, 1.0 is radius height and default.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim().len() == 0 {
            println!("Setting 1.0x.");
            return 1.0
        }
        match input.trim().parse::<f32>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid value, setting 1.0x.");
                1.0
            }
        }
    }
}