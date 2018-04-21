extern crate distance_field;

use distance_field::{Vec3, Face};

#[test]
fn cross() {
    let v0 = Vec3::new((0.0, 0.0, 1.0));
    let v1 = Vec3::new((0.0, 1.0, 0.0));
    assert_eq!(Vec3::new((1.0, 0.0, 0.0)), v1.cross(&v0));
    assert_eq!(Vec3::new((-1.0, 0.0, 0.0)), v0.cross(&v1));
}

#[test]
fn normal() {
    let v0 = Vec3::new((0.0, 0.0, 1.0));
    let v1 = Vec3::new((1.0, 0.0, 0.0));
    let v2 = Vec3::new((0.0, 1.0, 0.0));
    let f0 = Face::new(v0, v1, v2);
    let n = ((100.0 * f0.normal.x).round() / 100.0,
            (100.0 * f0.normal.y).round() / 100.0,
            (100.0 * f0.normal.z).round() / 100.0);
    assert_eq!((0.58, 0.58, 0.58), n);
}