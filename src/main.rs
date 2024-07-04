pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use camera::Camera;
use hittable_list::HittableList;
use sphere::sphere;
use vec3::*;

fn main() {
    let world = HittableList::new(vec![
        Box::new(sphere(&point3(0.0, 0.0, -1.0), 0.5)),
        Box::new(sphere(&point3(0.0, -100.5, -1.0), 100.0)),
    ]);

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;

    cam.render(&world);
}
