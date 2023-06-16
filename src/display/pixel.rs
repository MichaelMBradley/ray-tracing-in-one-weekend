use crate::math::vec3::Vec3;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_vec3(vec: &Vec3) -> Self {
        let n = vec.get_normalised() * 255.999f64;
        Self::new(n.x() as u8, n.y() as u8, n.z() as u8)
    }

    pub fn set_colours(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>3} {:>3} {:>3}", self.r, self.g, self.b)
    }
}
