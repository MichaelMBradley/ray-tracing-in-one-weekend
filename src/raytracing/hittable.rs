use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(intersect: Vec3, normal: Vec3, t: f64, ray: &Ray) -> Self {
        let front_face = ray.direction().dot(&normal) > 0.0;
        Self {
            p: intersect,
            normal: normal * if front_face { -1.0 } else { 1.0 },
            t,
            front_face,
        }
    }

    pub fn t(self) -> f64 {
        self.t
    }

    pub fn normal(self) -> Vec3 {
        self.normal
    }
}

pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, accept_t: Range<f64>) -> Option<HitRecord>;
}
