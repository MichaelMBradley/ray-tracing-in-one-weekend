use crate::math::ray::Ray;
use crate::math::vec3::Vec3;

pub fn ray_colour(ray: &Ray) -> Vec3 {
    let direction = ray.direction().into_normalised();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Vec3::new_diag(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
