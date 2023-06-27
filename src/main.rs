pub mod display;
pub mod math;
pub mod raytracing;

use crate::display::draw::draw;

fn main() {
    draw().write_file("out/8-5.ppm")
}
