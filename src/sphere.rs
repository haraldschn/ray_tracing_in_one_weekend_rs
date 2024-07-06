use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::*;
use crate::Material;
use crate::Vec3;

pub fn sphere(center: &Vec3, radius: f64, material: &Material) -> Sphere {
    Sphere::new(center, radius, material)
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Material,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64, material: &Material) -> Sphere {
        Sphere {
            center: center.clone(),
            radius: radius,
            mat: material.clone()
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = dot(&r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat;

        return true;
    }
}
