use barnacle_ray::scene::Scene;

fn main() {
    let mut scene: Scene = Scene::custom_light_scene();
    scene.cam.image_width = 400;
    scene.cam.samples_per_pixel = 50;
    scene.render();
}
