use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Self { r, g, b }
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
