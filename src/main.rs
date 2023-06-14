use crate::display::screen::{Screen, HEIGHT, WIDTH};

pub mod display;

fn main() {
    let mut screen = Screen::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            screen.get_pixel(x, y).set_colours(x as u8, y as u8, 0);
        }
    }

    screen.write_ppm("./out/grad.ppm")
}
