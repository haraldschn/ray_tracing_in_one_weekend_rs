use std::f64::INFINITY;

use indicatif::ProgressBar;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;

use crate::color::{color, write_color};
use crate::interval::interval;
use crate::ray::Ray;

use crate::vec3::*;

#[derive(Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,

    image_height: u64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let bar = ProgressBar::new(self.image_height);
        for j in 0..self.image_height {
            bar.inc(1);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(&self.center, &ray_direction);

                let pixel_color = self.ray_color(&r, &world);
                write_color(&pixel_color);
            }
        }
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = point3(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = vec3(viewport_width, 0., 0.);
        let viewport_v = vec3(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}

impl Camera {
    fn ray_color(&self, r: &Ray, world: &HittableList) -> Vec3 {
        let mut rec: HitRecord = HitRecord::default();
        if world.hit(r, interval(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + color(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(&r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0)
    }
}
