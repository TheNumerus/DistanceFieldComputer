use std::io;

const DEF_RADIUS: u32 = 64;

#[derive(Debug)]
pub struct GenSettings {
    pub radius: u32,
    pub repeat: ImgRepeat,
    pub height_setting: CaptureHeight,
    pub img_height_mult: f32,
}

impl GenSettings {
    pub fn new_from_input() -> GenSettings {
        let radius = GenSettings::get_radius_input();
        println!("-------------------------");
        let repeat = GenSettings::get_repeat_input();
        println!("-------------------------");
        let height_setting = GenSettings::get_height_input();
        println!("-------------------------");
        let height_mult = GenSettings::get_height_mult();
        println!("-------------------------");
        GenSettings::new_from_values((radius, repeat, height_setting, height_mult))
    }

    pub fn new_from_values(values: (u32, ImgRepeat, CaptureHeight, f32)) -> GenSettings {
        GenSettings {
            radius: values.0,
            repeat: values.1,
            height_setting: values.2,
            img_height_mult: values.3,
        }
    }

    pub fn new() -> GenSettings {
        GenSettings {
            radius: DEF_RADIUS,
            repeat: ImgRepeat::Repeat,
            height_setting: CaptureHeight::Generated,
            img_height_mult: 1.0,
        }
    }

    fn get_radius_input() -> u32 {
        let mut radius = String::new();
        println!(
            "Please input search radius (preferably power of two), default is {}.",
            DEF_RADIUS
        );
        io::stdin()
            .read_line(&mut radius)
            .expect("Failed to read input");
        if radius.trim().len() == 0 {
            println!("Setting {} as a radius.", DEF_RADIUS);
            return DEF_RADIUS;
        }
        match radius.trim().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid number, setting {} as a radius", DEF_RADIUS);
                DEF_RADIUS
            }
        }
    }

    fn get_repeat_input() -> ImgRepeat {
        println!("Please input image repeat option, default is Repeat.");
        println!("1 - Repeat, 2 - Clamp, 3 - Mirror");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim().len() == 0 {
            println!("Setting Repeat.");
            return ImgRepeat::Repeat;
        }
        match input.trim().parse::<u8>() {
            Ok(value) => match value {
                1 => ImgRepeat::Repeat,
                2 => ImgRepeat::Clamp,
                3 => ImgRepeat::Mirror,
                _ => {
                    eprintln!("Invalid option, setting Repeat");
                    ImgRepeat::Repeat
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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim().len() == 0 {
            println!("Setting automatic.");
            return CaptureHeight::Generated;
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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim().len() == 0 {
            println!("Setting 1.0x.");
            return 1.0;
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

#[derive(Debug)]
pub enum ImgRepeat {
    Repeat,
    Clamp,
    Mirror,
}

#[derive(Debug)]
pub enum CaptureHeight {
    UserDefined(u8),
    Generated,
}
