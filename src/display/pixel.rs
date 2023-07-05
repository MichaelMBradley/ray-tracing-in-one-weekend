use crate::math::vec3::Vec3;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn grayscale(g: u8) -> Self {
        Self { r: g, g, b: g }
    }
}

const SCALING: f64 = 256.0 - f64::EPSILON;

impl From<Vec3> for Pixel {
    fn from(vec: Vec3) -> Self {
        Self {
            r: (vec.x() * SCALING) as u8,
            g: (vec.y() * SCALING) as u8,
            b: (vec.z() * SCALING) as u8,
        }
    }
}

impl Into<Vec3> for Pixel {
    fn into(self: Self) -> Vec3 {
        Vec3::new(
            self.r as f64 / SCALING,
            self.g as f64 / SCALING,
            self.b as f64 / SCALING,
        )
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:>3} {:>3} {:>3}", self.r, self.g, self.b)
    }
}

pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0 };
pub const WHITE: Pixel = Pixel {
    r: 255,
    g: 255,
    b: 255,
};
pub const RED: Pixel = Pixel { r: 255, g: 0, b: 0 };
pub const GREEN: Pixel = Pixel { r: 0, g: 255, b: 0 };
pub const BLUE: Pixel = Pixel { r: 0, g: 0, b: 255 };
