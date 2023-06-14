use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;

pub const WIDTH: usize = 2 << 7;
pub const HEIGHT: usize = 2 << 7;

use crate::display::pixel::Pixel;

pub struct Screen {
    data: [[Pixel; WIDTH]; HEIGHT],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            data: [[Pixel::new(0, 0, 0); WIDTH]; HEIGHT],
        }
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> &mut Pixel {
        &mut self.data[x][y]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.data[x][y] = pixel;
    }

    pub fn write_ppm(&self, path: &str) {
        File::create(path)
            .expect(&format!("Could not create file: {path}"))
            .write(format!("{self}").as_bytes())
            .expect(&format!("Could not write to file: {path}"));
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut data = format!("P3\n{WIDTH} {HEIGHT}\n255\n");
        for row in self.data {
            for pixel in row {
                data.push_str(&format!("{pixel}\n"))
            }
        }
        write!(f, "{data}")
    }
}
