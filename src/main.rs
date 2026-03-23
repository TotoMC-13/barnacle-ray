use std::sync::Arc;

use barnacle_ray::{
    camera::Camera,
    hittable_list::HittableList,
    material::Lambertian,
    sphere::Sphere,
    vec3::{Color, Point3},
};

fn main() {
    // --- MATERIALES ---
    // El suelo verde
    let material_suelo = Arc::new(Lambertian::new(Color::new(0.49, 0.99, 0.00)));

    // La pelota roja (Lambertiana)
    let material_pelota_r = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    // La pelota azul (Lambertiana)
    let material_pelota_a = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));

    // --- MUNDO (SCENE) ---
    let mut world = HittableList::new();

    // 1. Pelota Roja a la derecha
    world.add(Box::new(Sphere::new(
        Point3::new(0.5, 0.0, -1.0),
        0.5,
        material_pelota_r,
    )));

    // 2. Pelota Azul a la izquierda
    world.add(Box::new(Sphere::new(
        Point3::new(-0.5, 0.0, -1.0),
        0.5,
        material_pelota_a,
    )));

    // 3. El Suelo (Esfera gigante)
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_suelo,
    )));

    // Camara
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 50, 50, true, 90.0);

    cam.render(&world);
}
