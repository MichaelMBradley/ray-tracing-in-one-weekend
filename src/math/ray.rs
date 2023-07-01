use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        Vec3::new(
            self.orig.x() + t * self.dir.x(),
            self.orig.y() + t * self.dir.y(),
            self.orig.z() + t * self.dir.z(),
        )
    }
}
