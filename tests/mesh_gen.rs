extern crate distance_field;
extern crate image;

use distance_field::mesh::Mesh;
use distance_field::settings;

#[test]
fn grey_bmp() {
    let input = String::from("./tests/grey.bmp");
    let img = image::open(&input);
    let img = match img {
        Ok(file) => file,
        Err(error) => panic!("Error with opening file {} :, {:?}", input, error),
    };
    let settings = settings::GenSettings::new();
    let mesh = Mesh::generate_mesh(&img, &settings);
}
