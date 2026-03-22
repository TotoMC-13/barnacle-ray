use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;

    fn emitted(&self) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // Normal + Vector Aleatorio en toda la esfera unitaria
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().unit_vector();

        // Evitamos que la direccio sea cero si el vector aleatorio es opuesto a la normal
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo; // Usamos el color propio de esta esfera
        true
    }
}
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        // Aseguramos que el fuzz no sea mayor a 1
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,              // El rayo que choca
        rec: &HitRecord,         // La informacion del choque (Punto P, Normal N)
        attenuation: &mut Color, // Cuanto "absorbe" el metal
        scattered: &mut Ray,     // Salida, el nuevo rayo que sale reflejado
    ) -> bool {
        // Calculamos la reflexion especular perfecta
        let reflected = Vec3::reflect(r_in.direction().unit_vector(), rec.normal);

        // Cuanto mayor sea self.fuzz (rugosidad), mas se alejara el rayo de una reflexion perfecta.
        let fuzzy_direction = reflected + self.fuzz * Vec3::random_in_unit_sphere();

        // Creamos el rayo que sale disparado desde el punto P
        *scattered = Ray::new(rec.p, fuzzy_direction);

        // Le asignamos al rayo el color del metal
        *attenuation = self.albedo;

        // Verificamos que el rayo reflejado salga hacia afuera de la superficie
        scattered.direction().dot(rec.normal) > 0.0
    }
}

pub struct Dialectric {
    ior: f64,
    albedo: Color,
}

impl Dialectric {
    pub fn new(ior: f64, albedo: Color) -> Dialectric {
        Dialectric { ior, albedo }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        // Calculamos el indice de refraccion interno
        let refraction_ratio = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let ray_direction: Vec3 = r_in.direction().unit_vector();

        // Calculamos el angulo con el que pega el rayo
        let cos_theta: f64 = (ray_direction * -1.0).dot(rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        // Calculamos si hay reflexion interna
        let scatter_direction: Vec3;
        let cannot_refract: bool = (refraction_ratio * sin_theta) > 1.0;

        if cannot_refract || Dialectric::reflectance(cos_theta, refraction_ratio) > random_double()
        {
            scatter_direction = Vec3::reflect(ray_direction, rec.normal);
        } else {
            // Calculamos la refraccion
            scatter_direction = Vec3::refract(ray_direction, rec.normal, refraction_ratio);
        }

        // Creamos el rayo que sale disparado desde el punto P
        *scattered = Ray::new(rec.p, scatter_direction);

        // Le asignamos al rayo el color del vidrio
        *attenuation = self.albedo;

        // Devolvemos que generamos el rayo exitosamente
        true
    }
}

pub struct DiffuseLight {
    emit_color: Color,
    intensity: f64,
}

impl DiffuseLight {
    pub fn new(emit_color: Color, intensity: f64) -> DiffuseLight {
        DiffuseLight { emit_color, intensity }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false // Una lampara no rebota la luz, corta la recursion.
    }
    fn emitted(&self) -> Color {
        self.emit_color * self.intensity // Multilplicamos la intensidad
    }
}
