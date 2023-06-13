use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

#[derive(Copy, Clone)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    fn new(r: u8, g: u8, b: u8) -> Pixel {
        Self { r, g, b }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>3} {:>3} {:>3}", self.r, self.g, self.b)
    }
}

type Screen = [[Pixel; WIDTH]; HEIGHT];

fn main() {
    let mut screen: Screen = [[Pixel::new(0, 0, 0); WIDTH]; HEIGHT];
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            screen[x][y] = Pixel::new(x as u8, y as u8, 0);
        }
    }

    write_ppm("./out/grad.ppm", screen)
}

fn write_ppm(path: &str, screen: Screen) {
    let mut data = format!("P3\n{WIDTH} {HEIGHT}\n255\n").to_owned();
    for row in screen {
        for pixel in row {
            data.push_str(&format!("{pixel}\n"))
        }
    }

    File::create(path)
        .expect(&format!("Could not create file: {path}"))
        .write(data.as_bytes())
        .expect(&format!("Could not write to file: {path}"));
}
