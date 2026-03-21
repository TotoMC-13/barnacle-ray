use std::f64::INFINITY;

use barnacle_ray::{
    camera::Camera,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    sphere::Sphere,
    vec3::{Color, Point3, Vec3},
};

const PI: f64 = 3.1415926535897932385;

fn main() {
    // World
    let mut world: HittableList = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))); // Esfera en el centro
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))); // El "piso"

    // Camara
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400);

    cam.render(&world);
}

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = center - r.origin();
    let a = r.direction().length_squared();
    let h = r.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        // Resolvemos para obtener el primer punto donde choca con la esfera (raiz 1)
        (h - discriminant.sqrt()) / a
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default(); // El formulario en blanco

    /*
        Le tiramos el rayo al mundo. t_min es 0.001 para evitar bugs de precisión, t_max es infinito.
        el rec va del .hit del world al del objeto al que chequeamos la colision, el objeto devuelve si le pegamos
        o no y despues procede a modificar el valor a donde apunta el puntero de rec, asi recuperando la info (record)
        de la colision
    */
    if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
        // Si choco con algo, calculamos el color en base a la normal de ESE objeto ganador
        return 0.5
            * Color::new(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            );
    }

    let unit_direction = r.direction().unit_vector();

    /*
        Ahora como y esta entre -1 y 1, vamos a pasarlo al rango de
        entre 0 y 1 para luego calcular facilmente el color
    */
    let a = 0.5 * (unit_direction.y() + 1.0);

    /*
        Usamos Lerp (linear interpolation). Cuando a = 0 quiero blanco, cuando a = 1 quiero azul
        y cuando 0 < a < 1 quiero un color en el medio. Un lerp es de la forma:

        blendedValue = (1 - a) * startValue + a * endValue.

        En nuestro caso representa lo siguiente:

        blendedValue = (1 - a) * blanco * a * celeste

    */
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
