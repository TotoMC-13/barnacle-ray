use std::sync::Arc;

use barnacle_ray::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dialectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point3},
};

fn main() {
    // World
    let mut world: HittableList = HittableList::new();

    let material_suelo = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_pelota = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
    let material_vidrio = Arc::new(Dialectric::new(1.50));

    world.add(Box::new(Sphere::new(
        Point3::new(0.5, 0.0, -1.0),
        0.5,
        material_pelota,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-0.5, 0.0, -1.0),
        0.5,
        material_vidrio.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-0.5, 0.0, -1.0),
        -0.45,
        material_vidrio.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_suelo,
    ))); // El "piso"

    // Camara
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 400, 50);

    cam.render(&world);
}
