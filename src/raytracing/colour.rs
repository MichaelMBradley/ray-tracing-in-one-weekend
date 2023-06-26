use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use crate::raytracing::hittable::Hittable;

pub fn ray_colour<H: Hittable>(ray: &Ray, hittable: &H) -> Vec3 {
    if let Some(t) = hittable.hit(ray, 0.0..f64::INFINITY) {
        return 0.5 * (t.normal() + Vec3::new_diag(1.0));
    }
    let direction = ray.direction().into_normalised();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Vec3::new_diag(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
