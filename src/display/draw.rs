use crate::display::screen::{Screen, HEIGHT, WIDTH};
use crate::math::vec3::{Vec3, ORIGIN};
use crate::raytracing::camera::Camera;
use crate::raytracing::colour::ray_colour;
use crate::raytracing::hittable_list::HittableList;
use crate::raytracing::sphere::Sphere;

const SQRT_SAMPLES: u8 = 10;

pub fn draw() -> Screen {
    let mut hittable_list = HittableList::new();
    hittable_list.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    hittable_list.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let mut screen = Screen::new();
    let camera = Camera::new();

    for i in 0..WIDTH {
        for j in (0..HEIGHT).rev() {
            let mut pixel = ORIGIN;
            for x in 0..SQRT_SAMPLES {
                let u = (i as f64 + (x as f64 / SQRT_SAMPLES as f64)) / (WIDTH as f64 - 1.0);
                for y in 0..SQRT_SAMPLES {
                    let v = (j as f64 + (y as f64 / SQRT_SAMPLES as f64)) / (HEIGHT as f64 - 1.0);
                    pixel += ray_colour(&camera.get_ray(u, v), &hittable_list);
                }
            }
            screen[i][j] = pixel / SQRT_SAMPLES.pow(2) as f64;
        }
    }

    screen
}
