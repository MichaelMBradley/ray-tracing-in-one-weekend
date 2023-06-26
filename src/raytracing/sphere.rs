use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use crate::raytracing::hittable::{HitRecord, Hittable};
use std::ops::Range;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(self: &Self, ray: &Ray, accept_t: Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let d_sqrt = discriminant.sqrt();
        let mut root = (-half_b - d_sqrt) / a;
        // Determine if either discriminant is in range
        if !accept_t.contains(&root) {
            root = (-half_b + d_sqrt) / a;
            if !accept_t.contains(&root) {
                return None;
            }
        }

        let intersect = ray.at(root);
        Some(HitRecord::new(
            intersect,
            (intersect - self.center) / self.radius,
            root,
            ray,
        ))
    }
}
