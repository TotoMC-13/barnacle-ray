use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t_min: f64, ray_t_max: f64, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    // NOTA: Se asume que el parametro outward_normal es un vector unitario
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Si el producto punto es < 0, el rayo y la normal externa van en
        // direcciones opuestas -> el rayo viene de afuera.
        let front_face = r.direction().dot(outward_normal) < 0.0;

        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
