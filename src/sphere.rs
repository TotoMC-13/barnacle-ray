use crate::{
    hittable::HitRecord,
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant: f64 = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root: f64 = (h - sqrtd) / a;

        // Si la raiz más cercana esta fuera de nuestro rango (atras de la cámara o muy lejos)
        if root <= ray_tmin || ray_tmax <= root {
            // Probamos con la raíz de la "salida" (+)
            root = (h + sqrtd) / a;

            // Si ninguna de las dos sirve, no hubo choque valido
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;

        rec.set_face_normal(r, outward_normal);

        true
    }
}
