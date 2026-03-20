use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // hittable_list() {}
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn with_object(object: Box<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    // Recibimos un Box (el equivalente al smart pointer en este caso)
    // y lo metemos en el vector.
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            // Le pasamos 'closest_so_far' como el nuevo t_max.
            // Si el objeto choca mas lejos que eso, el metodo 'hit' del objeto va a devolver false.
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t; // Actualizamos el record de cercania
                *rec = temp_rec.clone(); // Copiamos los datos al record definitivo
            }
        }

        hit_anything
    }
}
