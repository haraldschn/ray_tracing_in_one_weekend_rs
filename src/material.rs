use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::{random_unit_vector, reflect, Vec3};

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Lambertian,
    Metal,
    OtherMaterial,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    albedo: Vec3,
    mat_type: MaterialType,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            albedo: Vec3::default(),
            mat_type: MaterialType::Lambertian,
        }
    }
}

pub fn lambertian(albedo: Vec3) -> Material {
    Material { albedo: albedo, mat_type: MaterialType::Lambertian }
}

pub fn metal(albedo: Vec3) -> Material {
    Material { albedo: albedo, mat_type: MaterialType::Metal }
}

impl Material {
    pub fn new(albedo: Vec3, mat_type: MaterialType) -> Material {
        Material {
            albedo: albedo,
            mat_type: mat_type,
        }
    }

    pub fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {

        match self.mat_type {
            MaterialType::Lambertian => self.scatter_lambertian(rec),
            MaterialType::Metal => self.scatter_metal(r_in, rec),
            _ => (false, Vec3::default(), Ray::default())
        }
    }

    fn scatter_lambertian(&self, rec: &HitRecord) -> (bool, Vec3, Ray) {
        
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = &mut Ray::new(&rec.p, &scatter_direction);
        let attenuation = self.albedo;
        return (true, attenuation, *scattered)
    }

    fn scatter_metal(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Vec3, Ray) {
        
        let reflected = reflect(&r_in.direction(), &rec.normal);

        let scattered = &mut Ray::new(&rec.p, &reflected);
        let attenuation = self.albedo;
        return (true, attenuation, *scattered)
    }


}

