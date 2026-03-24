use barnacle_ray::{camera::Camera, scene::Scene};

fn main() {
    let mut scene: Scene = Scene::random_spheres();
    let cam: &mut Camera = &mut scene.cam;

    cam.image_width = 1920;
    cam.samples_per_pixel = 400;
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    scene.render();
}
