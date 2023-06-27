use crate::display::screen::ASPECT_RATIO;
use crate::math::ray::Ray;
use crate::math::vec3::{Vec3, ORIGIN};

pub struct Camera {
    origin: Vec3,
    ll_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ASPECT_RATIO;
        let focal_length = 1.0;

        let origin = ORIGIN;
        let horizontal = Vec3::new(viewport_width, 0f64, 0f64);
        let vertical = Vec3::new(0f64, viewport_height, 0f64);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0f64, 0f64, focal_length);
        Self {
            origin,
            ll_corner: lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.ll_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
