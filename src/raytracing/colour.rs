use crate::math::ray::Ray;
use crate::math::vec3::Vec3;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - *center;
    let a = ray.direction().length_squared();
    let b = 2.0 * oc.dot(&ray.direction());
    let c = oc.length_squared() - radius.powi(2);
    b * b - 4.0 * a * c > 0.0
}

pub fn ray_colour(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let direction = ray.direction().into_normalised();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Vec3::new_diag(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
