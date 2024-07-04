use crate::vec3::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3
}

impl Ray {

    pub fn new(origin: &Vec3, direction: &Vec3) -> Ray {
        Ray{
            orig: origin.clone(),
            dir : direction.clone(),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t : f64) -> Vec3 {
        self.orig + t*self.dir
    }

}

#[test]
fn ray_test() {
    let u = Vec3::builder().xyz(0., 0., 0.).build();
    let v = Vec3::builder().xyz(2., 2., 2.).build();
    
    let ray1 = Ray::new(&u, &v);
    let result = ray1.at(0.5);

    let result_expected = Vec3::builder().xyz(1., 1., 1.).build();

    assert_eq!(result, result_expected);
}