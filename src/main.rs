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
use hittable::Hittable;
use hittable_list::HittableList;
use material::*;
use rtweekend::{random_double, random_double_intv};
use sphere::sphere;
use vec3::*;

fn main() {
    let material_ground = lambertian(color(0.5, 0.5, 0.5));
    let mut world_vec: Vec<Box<dyn Hittable>> = vec![
        Box::new(sphere(&point3(0.0, -1000.0, -1.0), 1000.0, &material_ground)),
    ];

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = point3(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - point3(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // difuse
                    let albedo = Vec3::random() * Vec3::random();
                    let sphere_material = lambertian(albedo);
                    world_vec.push(Box::new(sphere(&center, 0.2, &sphere_material)));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::random_intv(0.5, 1.0);
                    let fuzz = random_double_intv(0.0, 0.5);    
                    let sphere_material = metal(albedo,fuzz);
                    world_vec.push(Box::new(sphere(&center, 0.2, &sphere_material)));
                } else {
                    // glass
                    let sphere_material = dielectric(1.5);
                    world_vec.push(Box::new(sphere(&center, 0.2, &sphere_material)));
                }

            }
        }
    }

    let material_1 = dielectric(1.5);
    world_vec.push(Box::new(sphere(&point3(0.0, 1.0, 0.0), 1.0, &material_1)));

    let material_2 = lambertian(color(0.4, 0.2, 0.1));
    world_vec.push(Box::new(sphere(&point3(-4.0, 1.0, 0.0), 1.0, &material_2)));

    let material_3 = metal(color(0.7, 0.6, 0.5), 0.0);
    world_vec.push(Box::new(sphere(&point3(4.0, 1.0, 0.0), 1.0, &material_3)));

    let world = HittableList::new(world_vec);

    let mut cam = Camera::default();

    cam.aspect_ratio      = 16.0 / 9.0;
    cam.image_width       = 1200;
    cam.samples_per_pixel = 10;
    cam.max_depth         = 50;

    cam.vfov      = 20.0;
    cam.look_from = point3(13.0,2.0,3.0);
    cam.look_at   = point3(0.0,0.0,0.0);
    cam.v_up      = vec3(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.render(&world);
}
