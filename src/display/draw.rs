use crate::display::screen::{Screen, ASPECT_RATIO, HEIGHT, WIDTH};
use crate::math::ray::Ray;
use crate::math::vec3::{Vec3, ORIGIN};
use crate::raytracing::colour::ray_colour;
use crate::raytracing::hittable_list::HittableList;
use crate::raytracing::sphere::Sphere;

pub fn draw() -> Screen {
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = ORIGIN;
    let horizontal = Vec3::new(viewport_width, 0f64, 0f64);
    let vertical = Vec3::new(0f64, viewport_height, 0f64);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0f64, 0f64, focal_length);

    let mut hittable_list = HittableList::new();
    hittable_list.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    hittable_list.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let mut screen = Screen::new();
    for i in 0..WIDTH {
        let u = (i as f64) / (WIDTH as f64 - 1.0);
        for j in (0..HEIGHT).rev() {
            let v = (j as f64) / (HEIGHT as f64 - 1.0);
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            screen[i][j] = ray_colour(&r, &hittable_list);
        }
    }

    screen
}
