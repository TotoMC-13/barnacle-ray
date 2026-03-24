use barnacle_ray::{camera::Camera, scene::Scene};

fn main() {
    let mut scene: Scene = Scene::random_spheres();
    let cam: &mut Camera = &mut scene.cam;

    cam.image_width = 400; // Use 1920 or whatever you prefer here, 400 is for fast renders.
    cam.samples_per_pixel = 50; // Put something like 400 here for good quality.
    cam.defocus_angle = 0.6; // Depends on your scene, put 0.0 for no blur.
    cam.focus_dist = 10.0; // Also depends on your scene.
    scene.render();
}
