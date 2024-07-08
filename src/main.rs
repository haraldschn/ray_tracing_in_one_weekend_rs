pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use std::f64::consts::PI;

use camera::Camera;
use color::color;
use hittable_list::HittableList;
use material::*;
use sphere::sphere;
use vec3::*;

fn main() {

    let r = (PI/4.0).cos();

    let material_left = lambertian(color(0.0, 0.0, 1.0));
    let material_right = lambertian(color(1.0, 0.0, 0.0));

    let world = HittableList::new(vec![
        Box::new(sphere(&point3(-r, 0.0, -1.0), r, &material_left)),
        Box::new(sphere(&point3(r, 0.0, -1.0), r, &material_right)),
    ]);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
