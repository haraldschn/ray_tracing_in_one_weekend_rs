use crate::interval::*;
use crate::vec3::*;

pub fn color(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::builder().xyz(x, y, z).build()
}

fn linear_to_gamma(linear_component : f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt()
    }
    return 0.0
}

pub fn write_color(pixel_color: &Vec3) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity: Interval = interval(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    print!("{} {} {}\n", rbyte, gbyte, bbyte);
}
