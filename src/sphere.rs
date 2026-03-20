use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        // La raiz del discriminante
        let sqrtd = discriminant.sqrt();

        // Obtenemos la primera raiz
        let mut root: f64 = (h - sqrtd) / a;

        // Si la raiz mas cercana esta fuera de nuestro rango (atras de la camara o muy lejos)
        if !ray_t.surrounds(root) {
            // Probamos con la raiz de la "salida" (+)
            root = (h + sqrtd) / a;

            // Si ninguna de las dos sirve, no hubo choque valido
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        // Nuestro t pasa a ser la raiz que nos quedamos
        rec.t = root;
        // El punto pasa a ser el rayo en t, es decir P(t)
        rec.p = r.at(rec.t);

        // Calculamos la normal que apunta desde el centro al pto de impacto, al dividirla por el radio la normalizamos
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;

        // Ve si el rayo viene de afuera o adentro. Despues guarda si el rayo viene de afuera
        // y la normal que usara para calcular luces y colores.
        rec.set_face_normal(r, outward_normal);

        true
    }
}
