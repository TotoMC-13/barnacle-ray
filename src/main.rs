use barnacle_ray::{
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

fn main() {
    // Image configuration
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    let focal_length = 1.0;

    // Viewport configuration
    let viewport_height = 2.0;

    // Usamos la relacion real entre ancho/alto para que no se estire la imagen
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    // Calcular los vectores horizontales y verticales por los bordes del viewport (V_u y V_v)
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calcular los deltas horizontales y verticales entre pixeles
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calcular la posicion de el pixel de arriba a la izquierda
    let vierpower_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel100_loc = vierpower_upper_left + 0.5 * (pixel_delta_u * pixel_delta_v);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rLineas restantes: {} ", image_height - j);
        for i in 0..image_width {
            // TODO: Repasar todo esto...
            let pixel_center =
                pixel100_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(r);
            println!("{}", pixel_color)
        }
    }

    eprintln!("\rListo              \n");
}

pub fn ray_color(r: Ray) -> Color {
    let unit_direction: Vec3 = r.direction().unit_vector();

    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
