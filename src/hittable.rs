use std::sync::Arc;

use crate::{
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    // NOTA: Se asume que el parametro outward_normal es un vector unitario
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        /*
            Si el producto punto es negativo: El rayo y la normal apuntan en direcciones opuestas.
            Eso significa que el rayo viene de afuera y golpea la cara exterior.

            Si el producto punto es positivo: El rayo y la normal van en la misma direccion.
            Eso significa que el rayo se origino dentro de la esfera y esta golpeando la cara interna.
        */
        let front_face = r.direction().dot(outward_normal) < 0.0;

        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        self.front_face = front_face;
    }
}
