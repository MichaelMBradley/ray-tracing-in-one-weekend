use crate::display::screen::{Screen, HEIGHT, WIDTH};
use crate::math::vec3::Vec3;

pub mod display;
pub mod math;

fn main() {
    let mut screen = Screen::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let vec = Vec3::new(x as f64, y as f64, 0f64);
            screen
                .get_pixel(x, y)
                .set_colours(vec.x() as u8, vec.y() as u8, vec.length() as u8);
        }
    }

    screen.write_ppm("./out/grad.ppm");

    let vec1 = Vec3::new(0f64, 3f64, 4f64);
    let vec2 = Vec3::new(3f64, 4f64, 5f64);

    println!(
        "{:?}\n{:?}\n{:?}\n{}\n{}",
        vec1,
        vec2,
        vec1 + vec2,
        vec1.dot(&vec2),
        vec1.length()
    )
}
