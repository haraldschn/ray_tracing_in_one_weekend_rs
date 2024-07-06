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

use camera::Camera;
use color::color;
use hittable_list::HittableList;
use material::*;
use sphere::sphere;
use vec3::*;

fn main() {
    let material_ground = lambertian(color(0.8, 0.8, 0.0));
    let material_center = lambertian(color(0.1, 0.2, 0.5));
    let material_left = metal(color(0.8, 0.8, 0.8));
    let material_right = metal(color(0.8, 0.6, 0.2));

    let world = HittableList::new(vec![
        Box::new(sphere(&point3(0.0, -100.5, -1.0), 100.0, &material_ground)),
        Box::new(sphere(&point3(0.0, 0.0, -1.2), 0.5, &material_center)),
        Box::new(sphere(&point3(-1.0, 0.0, -1.0), 0.5, &material_left)),
        Box::new(sphere(&point3(1.0, 0.0, -1.0), 0.5, &material_right)),
    ]);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
