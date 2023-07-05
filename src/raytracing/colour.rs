use crate::math::ray::Ray;
use crate::math::vec3::{Vec3, ORIGIN};
use crate::raytracing::hittable::Hittable;

pub fn ray_colour<H: Hittable>(ray: &Ray, hittable: &H, depth: u8) -> Vec3 {
    if depth <= 0 {
        return ORIGIN;
    }

    if let Some(rec) = hittable.hit(ray, 0.001..f64::INFINITY) {
        let target = rec.normal() + Vec3::random_unit_vector();
        return 0.5 * ray_colour(&Ray::new(rec.p().clone(), target), hittable, depth - 1);
    }

    let direction = ray.direction().into_normalised();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Vec3::new_diag(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
