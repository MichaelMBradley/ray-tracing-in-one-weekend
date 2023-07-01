use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use std::ops::Range;

#[derive(Debug)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    // front_face: bool,
}

impl<'a> HitRecord {
    pub fn new(intersect: Vec3, normal: Vec3, t: f64, ray: &Ray) -> Self {
        let front_face = ray.direction().dot(&normal) > 0.0;
        Self {
            p: intersect,
            normal: normal * if front_face { -1.0 } else { 1.0 },
            t,
            // front_face,
        }
    }

    pub fn p(self) -> &'a Vec3 {
        &self.p
    }

    pub fn normal(self) -> &'a Vec3 {
        &self.normal
    }

    pub fn t(self) -> f64 {
        self.t
    }
}

pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, accept_t: Range<f64>) -> Option<HitRecord>;
}
