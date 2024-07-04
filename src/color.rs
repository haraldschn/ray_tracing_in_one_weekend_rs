use crate::vec3::Vec3;

pub fn color(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::builder().xyz(x, y, z).build()
}

pub fn write_color(pixel_color: &Vec3) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    print!("{} {} {}\n", rbyte, gbyte, bbyte);
}
