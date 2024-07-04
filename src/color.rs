use crate::interval::*;
use crate::vec3::*;

pub fn color(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::builder().xyz(x, y, z).build()
}

pub fn write_color(pixel_color: &Vec3) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity: Interval = interval(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    print!("{} {} {}\n", rbyte, gbyte, bbyte);
}
