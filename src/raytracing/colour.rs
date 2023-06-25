use crate::math::ray::Ray;
use crate::math::vec3::Vec3;

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin() - *center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(&ray.direction());
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

pub fn ray_colour(ray: &Ray) -> Vec3 {
    if let Some(t) = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).into_normalised();
        return 0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let direction = ray.direction().into_normalised();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Vec3::new_diag(1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
