use std::sync::Arc;

use barnacle_ray::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dialectric, DiffuseLight, Lambertian},
    sphere::Sphere,
    vec3::{Color, Point3},
};

fn main() {
    // --- MATERIALES ---
    // El suelo verde
    let material_suelo = Arc::new(Lambertian::new(Color::new(0.49, 0.99, 0.00)));

    // La pelota roja (Lambertiana)
    let material_pelota = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    // El vidrio cian (Dielectrico)
    let material_vidrio = Arc::new(Dialectric::new(1.50, Color::new(0.2, 0.8, 1.0)));

    // Luz 1: Calida (Naranja) - Intensidad 50.0
    let luz_naranja = Arc::new(DiffuseLight::new(Color::new(1.0, 0.6, 0.2), 50.0));

    // Luz 2: Fria (Azul/Blanca) - Intensidad 30.0 para contraste
    let luz_azul = Arc::new(DiffuseLight::new(Color::new(0.3, 0.5, 1.0), 30.0));

    // --- MUNDO (SCENE) ---
    let mut world = HittableList::new();

    // 1. Pelota Roja a la derecha
    world.add(Box::new(Sphere::new(
        Point3::new(1.25, 0.0, -1.5),
        0.5,
        material_pelota,
    )));

    // 2. Luz Naranja (arriba al centro)
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 2.5, -1.5),
        0.5,
        luz_naranja,
    )));

    // 3. Luz Azul (lateral derecha, más baja)
    // Esto va a crear reflejos azules en el borde derecho de las esferas
    world.add(Box::new(Sphere::new(
        Point3::new(3.0, 1.0, -1.0),
        0.3,
        luz_azul,
    )));

    // 4. Esfera de Vidrio (Exterior)
    world.add(Box::new(Sphere::new(
        Point3::new(-1.25, 0.0, -1.5),
        0.5,
        material_vidrio.clone(),
    )));

    // 5. Esfera de Vidrio (Interior / Burbuja de aire)
    // El radio negativo hace que la normal apunte hacia adentro
    world.add(Box::new(Sphere::new(
        Point3::new(-1.25, 0.0, -1.5),
        -0.45,
        material_vidrio.clone(),
    )));

    // 6. El Suelo (Esfera gigante)
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_suelo,
    )));

    // Camara
    let mut cam: Camera = Camera::new(16.0 / 9.0, 1920, 400, 50, false);

    cam.render(&world);
}
