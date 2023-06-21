use crate::math::vec3::Vec3;
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
            data.push(Vec3::origin());
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
        for pixel in self.data.as_slice() {
            data.push_str(&format!("{}\n", *pixel))
        }
        write!(f, "{data}")
    }
}

impl ops::Index<i32> for Screen {
    type Output = [Vec3];

    fn index(&self, index: i32) -> &Self::Output {
        if index >= 0 {
            &self.data[(index as usize * WIDTH)..((index as usize + 1) * WIDTH)]
        } else {
            &self.data[((index + WIDTH as i32) as usize * WIDTH)
                ..(((index + WIDTH as i32) as usize + 1) * WIDTH)]
        }
    }
}

impl ops::IndexMut<i32> for Screen {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        if index >= 0 {
            &mut self.data[(index as usize * WIDTH)..((index as usize + 1) * WIDTH)]
        } else {
            &mut self.data[((index + WIDTH as i32) as usize * WIDTH)
                ..(((index + WIDTH as i32) as usize + 1) * WIDTH)]
        }
    }
}

impl ops::Index<usize> for Screen {
    type Output = [Vec3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[(index * WIDTH)..((index + 1) * WIDTH)]
    }
}

impl ops::IndexMut<usize> for Screen {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[(index * WIDTH)..((index + 1) * WIDTH)]
    }
}
