use crate::math::ray::Ray;
use crate::raytracing::hittable::{HitRecord, Hittable};
use std::ops::Range;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add<H: Hittable + 'static>(self: &mut Self, object: H) {
        self.objects.push(Box::new(object))
    }

    pub fn clear(self: &mut Self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(self: &Self, ray: &Ray, accept_t: Range<f64>) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut t_max = accept_t.end;

        for object in self.objects.as_slice() {
            if let Some(hit) = object.hit(ray, accept_t.start..t_max) {
                t_max = hit.t();
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}
