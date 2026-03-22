use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    origin: Point3, // Punto de paso
    dir: Vec3,      // Vector director
}

impl Ray {
    // Constructor
    pub fn new(origin: Point3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    // Getters
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // x(t) = t * vector_director + punto_de_paso
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.dir
    }
}
