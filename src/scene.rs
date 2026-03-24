use std::sync::Arc;

use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    sphere::Sphere,
    utils::{random_double, random_double_range},
    vec3::{Color, Point3, Vec3},
};

pub struct Scene {
    pub cam: Camera,
    pub world: HittableList,
}

impl Scene {
    pub fn new(cam: Camera, world: HittableList) -> Scene {
        Scene { cam, world }
    }

    pub fn render(&mut self) {
        self.cam.render(&self.world);
    }

    // ESCENA 1: Tu escena base
    pub fn basic_spheres() -> Self {
        let mut world = HittableList::new();

        let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let material_left = Arc::new(Dielectric::new(1.50, Color::new(1.0, 1.0, 1.0)));
        let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50, Color::new(1.0, 1.0, 1.0)));
        let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

        world.add(Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -1.2),
            0.5,
            material_center,
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.4,
            material_bubble,
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )));

        let mut cam = Camera::default();
        cam.aspect_ratio = 16.0 / 9.0;
        cam.image_width = 400;
        cam.samples_per_pixel = 100;
        cam.max_depth = 50;
        cam.vfov = 90.0;
        cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
        cam.lookat = Point3::new(0.0, 0.0, -1.0);
        cam.vup = Vec3::new(0.0, 1.0, 0.0);
        cam.sky_emits_light = true;

        Self::new(cam, world)
    }

    // ESCENA 2: Dos esferas gigantes
    pub fn two_spheres() -> Self {
        let mut world = HittableList::new();

        let mat_bottom = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
        let mat_top = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
        let mat_floor = Arc::new(Lambertian::new(Color::new(0.1, 0.9, 0.0)));

        world.add(Box::new(Sphere::new(
            Point3::new(0.5, 0.0, -1.0),
            0.5,
            mat_bottom,
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(-0.5, 0.0, -1.0),
            0.5,
            mat_top,
        )));
        world.add(Box::new(Sphere::new(
            Point3::new(0.0, -100.5, 0.0),
            100.0,
            mat_floor,
        )));

        let mut cam = Camera::default();
        cam.aspect_ratio = 16.0 / 9.0;
        cam.image_width = 1920;
        cam.samples_per_pixel = 200;
        cam.max_depth = 50;
        cam.vfov = 90.0;
        cam.lookfrom = Point3::default();
        cam.lookat = Point3::new(0.0, 0.0, -1.0);
        cam.vup = Vec3::new(0.0, 1.0, 0.0);
        cam.sky_emits_light = true;

        Self::new(cam, world)
    }

    // ESCENA 3: Escena random (Portada del libro)
    pub fn random_spheres() -> Self {
        let mut world = HittableList::new();

        // Suelo gigante
        let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        world.add(Box::new(Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_double();
                let center = Point3::new(
                    a as f64 + 0.9 * random_double(),
                    0.2,
                    b as f64 + 0.9 * random_double(),
                );

                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // Difuso
                        let albedo = Color::random() * Color::random();
                        let sphere_material = Arc::new(Lambertian::new(albedo));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    } else if choose_mat < 0.95 {
                        // Metal
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_double_range(0.0, 0.5);
                        let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    } else {
                        // Vidrio
                        let sphere_material =
                            Arc::new(Dielectric::new(1.5, Color::new(1.0, 1.0, 1.0)));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }

        let mat1 = Arc::new(Dielectric::new(1.5, Color::new(1.0, 1.0, 1.0)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1)));

        let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.add(Box::new(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            mat2,
        )));

        let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3)));

        let mut cam = Camera::default();
        cam.aspect_ratio = 16.0 / 9.0;
        cam.image_width = 1920;
        cam.samples_per_pixel = 400;
        cam.max_depth = 50;
        cam.vfov = 20.0; // Zoom in
        cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
        cam.lookat = Point3::new(0.0, 0.0, 0.0);
        cam.vup = Vec3::new(0.0, 1.0, 0.0);
        cam.sky_emits_light = true;

        Self::new(cam, world)
    }

    // ESCENA 4: Escena con iluminacion custom y burbuja de vidrio 
pub fn custom_light_scene() -> Self {
        let mut world = HittableList::new();

        let material_suelo = Arc::new(Lambertian::new(Color::new(0.49, 0.99, 0.00)));
        let material_pelota = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));
        let material_vidrio = Arc::new(Dielectric::new(1.50, Color::new(0.2, 0.8, 1.0)));
        let luz_naranja = Arc::new(DiffuseLight::new(Color::new(1.0, 0.6, 0.2), 50.0));
        let luz_azul = Arc::new(DiffuseLight::new(Color::new(0.3, 0.5, 1.0), 30.0));

        world.add(Box::new(Sphere::new(Point3::new(1.25, 0.0, -1.5), 0.5, material_pelota)));
        world.add(Box::new(Sphere::new(Point3::new(0.0, 2.5, -1.5), 0.5, luz_naranja)));
        world.add(Box::new(Sphere::new(Point3::new(3.0, 1.0, -1.0), 0.3, luz_azul)));
        world.add(Box::new(Sphere::new(Point3::new(-1.25, 0.0, -1.5), 0.5, material_vidrio.clone())));
        world.add(Box::new(Sphere::new(Point3::new(-1.25, 0.0, -1.5), -0.45, material_vidrio.clone())));
        world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_suelo)));

        let mut cam = Camera::default();
        cam.aspect_ratio = 16.0 / 9.0;
        cam.image_width = 1920;
        cam.samples_per_pixel = 400;
        cam.max_depth = 50;
        cam.vfov = 90.0;
        cam.lookfrom = Point3::new(0.0, 0.0, 0.0);
        cam.lookat = Point3::new(0.0, 0.0, -1.0);
        cam.vup = Vec3::new(0.0, 1.0, 0.0);
        cam.sky_emits_light = false;

        Self::new(cam, world)
    }
}
