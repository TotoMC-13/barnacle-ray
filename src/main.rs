use barnacle_ray::vec3::Color;

fn main() {
    let image_width: u16 = 256;
    let image_height: u16 = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rLineas restantes: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_color: Color = Color::new(
                i as f64 / (image_height - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.0,
            );

            println!("{}", pixel_color)
        }
    }

    eprintln!("\rListo              \n");
}
