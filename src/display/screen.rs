use crate::display::pixel::Pixel;
use crate::math::vec3::{Vec3, ORIGIN};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::ops;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub const HEIGHT: usize = 400;
pub const WIDTH: usize = (HEIGHT as f64 * ASPECT_RATIO) as usize;

pub struct Screen {
    data: Vec<Vec3>,
}

impl Screen {
    pub fn new() -> Screen {
        let mut data = Vec::with_capacity(WIDTH * HEIGHT);
        for _ in 0..WIDTH * HEIGHT {
            data.push(ORIGIN);
        }
        Screen { data }
    }

    pub fn write_file(&self, path: &str) {
        File::create(path)
            .expect(&format!("Could not create file: {path}"))
            .write(format!("{self}").as_bytes())
            .expect(&format!("Could not write to file: {path}"));
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut data = format!("P3\n{WIDTH} {HEIGHT}\n255\n");
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                data.push_str(&format!("{}\n", Pixel::from(self.data[i * HEIGHT + j])))
            }
        }
        write!(f, "{data}")
    }
}

impl ops::Index<i32> for Screen {
    type Output = [Vec3];

    fn index(&self, index: i32) -> &Self::Output {
        if index >= 0 {
            &self.data[(index as usize * HEIGHT)..((index as usize + 1) * HEIGHT)]
        } else {
            &self.data[((index + HEIGHT as i32) as usize * HEIGHT)
                ..(((index + HEIGHT as i32) as usize + 1) * HEIGHT)]
        }
    }
}

impl ops::IndexMut<i32> for Screen {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        if index >= 0 {
            &mut self.data[(index as usize * HEIGHT)..((index as usize + 1) * HEIGHT)]
        } else {
            &mut self.data[((index + HEIGHT as i32) as usize * HEIGHT)
                ..(((index + HEIGHT as i32) as usize + 1) * HEIGHT)]
        }
    }
}

impl ops::Index<usize> for Screen {
    type Output = [Vec3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[(index * HEIGHT)..((index + 1) * HEIGHT)]
    }
}

impl ops::IndexMut<usize> for Screen {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[(index * HEIGHT)..((index + 1) * HEIGHT)]
    }
}
