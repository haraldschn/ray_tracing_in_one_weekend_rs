use crate::color::color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::rtweekend::random_double;
use crate::{dot, random_unit_vector, reflect, refract, unit_vector, Vec3};

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
    OtherMaterial,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    albedo: Vec3,
    mat_type: MaterialType,
    fuzz: f64,
    refraction_index: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            albedo: Vec3::default(),
            mat_type: MaterialType::Lambertian,
            fuzz: 0.0,
            refraction_index: 0.0,
        }
    }
}

pub fn lambertian(albedo: Vec3) -> Material {
    Material {
        albedo: albedo,
        mat_type: MaterialType::Lambertian,
        fuzz: 0.0,
        refraction_index: 0.0,
    }
}

pub fn metal(albedo: Vec3, fuzz: f64) -> Material {
    if fuzz.abs() < 1.0 {
        Material {
            albedo: albedo,
            mat_type: MaterialType::Metal,
            fuzz: fuzz,
            refraction_index: 0.0,
        }
    } else {
        Material {
            albedo: albedo,
            mat_type: MaterialType::Metal,
            fuzz: 1.0,
            refraction_index: 0.0,
        }
    }
}

pub fn dielectric(refraction: f64) -> Material {
    Material {
        albedo: color(0.0, 0.0, 0.0),
        mat_type: MaterialType::Dielectric,
        fuzz: 0.0,
        refraction_index: refraction,
    }
}

impl Material {
    pub fn new(albedo: Vec3, mat_type: MaterialType, fuzz: f64, refr: f64) -> Material {
        Material {
            albedo: albedo,
            mat_type: mat_type,
            fuzz: fuzz,
            refraction_index: refr,
        }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        match self.mat_type {
            MaterialType::Lambertian => self.scatter_lambertian(rec),
            MaterialType::Metal => self.scatter_metal(r_in, rec),
            MaterialType::Dielectric => self.scatter_dielectric(r_in, rec),
            _ => (false, Vec3::default(), Ray::default()),
        }
    }

    fn scatter_lambertian(&self, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = &mut Ray::new(&rec.p, &scatter_direction);
        let attenuation = self.albedo;
        return (true, attenuation, *scattered);
    }

    fn scatter_metal(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());

        let scattered = &mut Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo;
        let scatter_bool = dot(&scattered.direction(), &rec.normal) > 0.0;
        return (scatter_bool, attenuation, *scattered);
    }

    fn scatter_dielectric(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        let attenuation = color(1.0, 1.0, 1.0);

        let ri;
        if rec.front_face {
            ri = 1.0 / self.refraction_index;
        } else {
            ri = self.refraction_index;
        }

        let unit_direction = unit_vector(&r_in.direction());
        let neg_unit_direction = -unit_direction;
        let cos_theta = dot(&neg_unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = ri*sin_theta > 1.0;
        let direction;

        if cannot_refract || reflectance(cos_theta, ri) > random_double() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, ri);
        }

        let scattered = &mut Ray::new(&rec.p, &direction);
        return (true, attenuation, *scattered);
    }
}


fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0*r0;
    
    r0 + (1.0-r0)*(1.0-cosine).powi(5)
}