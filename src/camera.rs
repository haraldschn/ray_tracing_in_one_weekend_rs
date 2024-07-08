use std::f64::INFINITY;

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;

use crate::color::{color, write_color};
use crate::interval::interval;
use crate::ray::Ray;

use crate::rtweekend::{degrees_to_radians, random_double};
use crate::vec3::*;

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_width: u64,         // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: usize,         // Maximum number of ray bounces into scene

    pub vfov: usize, // Vertical view angle (field of view)
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub v_up: Vec3,

    image_height: u64,
    pixel_samples_scale: f64,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90,
            look_from: vec3(0.0, 0.0, 0.0),
            look_at: vec3(0.0, 0.0, -1.0),
            v_up: vec3(0.0, 1.0, 0.0),

            image_height: 100,
            pixel_samples_scale: 0.1,
            center: Vec3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),

            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", (self.image_height - 1) - j);
            for i in 0..self.image_width {
                let mut pixel_color = color(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color = pixel_color + self.ray_color(&r, self.max_depth, world);
                }

                write_color(&(pixel_color * self.pixel_samples_scale));
            }
        }
        eprintln!("\nDone");
    }

    fn initialize(&mut self) {
        // Calculate the image height, and ensure that it's at least 1
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.look_from;

        // Determine viewport dimensions.
        let focal_length = (self.look_from - self.look_at).length();
        let theta = degrees_to_radians(self.vfov as f64);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = unit_vector(&(self.look_from - self.look_at));
        self.u = unit_vector(&cross(&self.v_up, &self.w));
        self.v = cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.center - (focal_length * self.w) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return vec3(random_double() - 0.5, random_double() - 0.5, 0.0);
    }

    fn ray_color(&self, r: &Ray, depth: usize, world: &HittableList) -> Vec3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return color(0.0, 0.0, 0.0);
        }

        let mut rec: HitRecord = HitRecord::default();

        if world.hit(r, interval(0.001, INFINITY), &mut rec) {
            let (hit_bool, attenuation, scattered) = rec.mat.scatter(r, &rec);

            if hit_bool {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return color(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(&r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0)
    }
}
