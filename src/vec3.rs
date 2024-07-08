use std::ops;

use crate::rtweekend::{random_double, random_double_intv};

// Alias constructor functions
pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::builder().xyz(x, y, z).build()
}

pub fn point3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::builder().xyz(x, y, z).build()
}

/// Vec3 struct
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    // This method will help users to discover the builder
    pub fn builder() -> Vec3Builder {
        Vec3Builder::default()
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &0.,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

// Vector functions for Unit Operations

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x - _rhs,
            y: self.y - _rhs,
            z: self.z - _rhs,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * (1. / _rhs),
            y: self.y * (1. / _rhs),
            z: self.z * (1. / _rhs),
        }
    }
}

// Traits for commutative Operations
macro_rules! vec3_op_for {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        impl $($path)::+<$ty> for Vec3 {
            type Output = Vec3;
            fn $fn(self, other: $ty) -> Self::Output {
                Vec3 {
                    x: self.x.$fn(other),
                    y: self.y.$fn(other),
                    z: self.z.$fn(other),
                }
            }
        }
        impl $($path)::+<Vec3> for $ty {
            type Output = Vec3;
            fn $fn(self, other: Vec3) -> Self::Output {
                Vec3 {
                    x: other.x.$fn(self),
                    y: other.y.$fn(self),
                    z: other.z.$fn(self),
                }
            }
        }
    };
}

macro_rules! vec3_for {
    ($ty:ty) => {
        vec3_op_for!(ops::Add, add, $ty);
        vec3_op_for!(ops::Mul, mul, $ty);
    };
}

vec3_for!(f64);

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double(),
        }
    }

    pub fn random_intv(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_double_intv(min, max),
            y: random_double_intv(min, max),
            z: random_double_intv(min, max),
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

#[inline(always)]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

#[inline(always)]
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3 {
            x: random_double_intv(-1.0, 1.0),
            y: random_double_intv(-1.0, 1.0),
            z: 0.0,
        };
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline(always)]
pub fn random_in_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_intv(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline(always)]
pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_vector())
}

#[inline(always)]
pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[inline(always)]
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    (*v) - 2.0 * dot(v, n) * (*n)
}

#[inline(always)]
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let uv_b = *uv;
    let cos_theta = dot(&-uv_b, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv_b + cos_theta * *n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
    r_out_perp + r_out_parallel
}

// Vector Builder (for using Builder Pattern)

#[derive(Default)]
pub struct Vec3Builder {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3Builder {
    pub fn new(/* ... */) -> Vec3Builder {
        // Set the minimally required fields of Vec3.
        Vec3Builder {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn x(mut self, x: f64) -> Vec3Builder {
        self.x = x;
        self
    }

    pub fn y(mut self, y: f64) -> Vec3Builder {
        self.y = y;
        self
    }

    pub fn z(mut self, z: f64) -> Vec3Builder {
        self.z = z;
        self
    }

    pub fn xy(mut self, x: f64, y: f64) -> Vec3Builder {
        self.x = x;
        self.y = y;
        self
    }

    pub fn yz(mut self, y: f64, z: f64) -> Vec3Builder {
        self.y = y;
        self.z = z;
        self
    }

    pub fn xyz(mut self, x: f64, y: f64, z: f64) -> Vec3Builder {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn build(self) -> Vec3 {
        // Create a Vec3 from the Vec3Builder, applying all settings in Vec3Builder
        // to Vec3.
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[test]
fn builder_test() {
    let foo = Vec3 {
        x: 1.,
        y: 2.,
        z: 3.,
    };
    let foo_from_builder: Vec3 = Vec3Builder::new().x(1.).y(2.).z(3.).build();
    assert_eq!(foo, foo_from_builder);
    let foo2_from_builder = Vec3::builder().xyz(1., 2., 3.).build();
    assert_eq!(foo, foo2_from_builder);
}

#[test]
fn default_test() {
    let foo = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let foo_from_builder: Vec3 = Vec3::builder().build();
    assert_eq!(foo, foo_from_builder);
}

#[test]
fn ops_add_f64_test() {
    let foo = Vec3 {
        x: 11.,
        y: 12.,
        z: 13.,
    };
    let vec_2 = Vec3::builder().xyz(1., 2., 3.).build();
    let vec_3 = vec_2 + 10.;
    assert_eq!(foo, vec_3);
    let vec_4 = 10. + vec_2;
    assert_eq!(foo, vec_4);
}

#[test]
fn ops_sub_f64_test() {
    let foo = Vec3 {
        x: 11.,
        y: 12.,
        z: 13.,
    };
    let vec_2 = Vec3::builder().xyz(21., 22., 23.).build();
    let vec_3 = vec_2 - 10.;
    assert_eq!(foo, vec_3);
}

#[test]
fn ops_negation_test() {
    let vec_1: Vec3 = Vec3 {
        x: -1.,
        y: -2.,
        z: -3.,
    };
    let vec_2 = Vec3::builder().xyz(1., 2., 3.).build();
    let vec_2 = -vec_2;
    assert_eq!(vec_1, vec_2);
}

#[test]
fn ops_indexing_test() {
    let foo = Vec3 {
        x: 1.,
        y: 2.,
        z: 3.,
    };
    assert_eq!(foo[0], 1.);
    assert_eq!(foo[1], 2.);
    assert_eq!(foo[2], 3.);
}

#[test]
fn dot_product_test() {
    let u = Vec3::builder().xyz(1., 3., -5.).build();
    let v = Vec3::builder().xyz(4., -2., -1.).build();
    let result = dot(&u, &v);
    assert_eq!(result, 3.);
}

#[test]
fn length_test() {
    let u = Vec3::builder().xyz(4., 0., 0.).build();
    let result = u.length();
    assert_ne!(result, 1.);
    assert_eq!(result, 4.);
    let result_squared = u.length_squared();
    assert_eq!(result_squared, 16.);
}

#[test]
fn unit_vector_test() {
    let mut u = Vec3::builder().xyz(4., 0., 0.).build();
    u = unit_vector(&u);

    let v = Vec3::builder().xyz(1., 0., 0.).build();

    assert_eq!(u, v);
}
