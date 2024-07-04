pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use indicatif::ProgressBar;

use color::{color, write_color};
use interval::interval;
use ray::Ray;
use sphere::sphere;
use std::f64::INFINITY;
use vec3::*;

fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec: HitRecord = HitRecord::default();
    if world.hit(r, interval(0.0, INFINITY), &mut rec) {
        return 0.5 * (rec.normal + color(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(&r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u64 = 400;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height = (image_width as f64 / aspect_ratio) as u64;
    if image_height < 1 {
        image_height = 1;
    }

    // World

    let world = HittableList::new(vec![
        Box::new(sphere(&point3(0.0, 0.0, -1.0), 0.5)),
        Box::new(sphere(&point3(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = point3(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3(viewport_width, 0., 0.);
    let viewport_v = vec3(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let bar = ProgressBar::new(image_height);
    for j in 0..image_height {
        bar.inc(1);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r, &world);
            write_color(&pixel_color);
        }
    }
}
