use barnacle_ray::{camera::Camera, hittable_list::HittableList, sphere::Sphere, vec3::Point3};

fn main() {
    // World
    let mut world: HittableList = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5))); // Esfera en el centro
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0))); // El "piso"

    // Camara
    let mut cam: Camera = Camera::new(16.0 / 9.0, 400, 100);

    cam.render(&world);
}
