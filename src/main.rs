use barnacle_ray::{
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

fn main() {
    // Configuracion de la imagen
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    let focal_length = 1.0;

    // Configuracion del viewport
    let viewport_height = 2.0;

    // Usamos la relacion real entre ancho/alto para que no se estire la imagen
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    /*
        Calcular los vectores horizontales y verticales por los bordes del viewport (V_u y V_v)
        Estos son los vectores que nos ayudaran a movernos a lo largo del viewport

        V_u va para el +x
        V_v va para el -y
    */
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0); // V_u
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // V_v

    /*
        Calcular los deltas horizontales y verticales entre pixeles
        Al dividir los vectores por el alto/ancho de la imagen, tenemos un nuevo vector que al sumarlo/restarlo
        nos movera exactamente 1 pixel
    */
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    /*
        Calcular la posicion de el pixel de arriba a la izquierda

        El eje +z es el que viene directo a la camara, entonces para "ir" al viewport tenemos que
        ir hacia -z, sabiendo que la distancia del viewport a la camara es focal_length, hacemos un vector
        cuya coordenada z sea focal_length y se lo restamos al centro de la camara. Ahora estamos parados en
        el centro del viewport.

        Ahora nos faltaria ir arriba a la izquierda para llegar al inicio del viewport (Q). Lo que hacemos es
        Movernos la mitad de -V_u y la mitad de -V_v. Los valores son negativos porque nos queremos mover hacia
        -x y +y siendo que V_u apunta a +x y V_v apunta a -y. Luego de todo esto, ya tenemos las coordenadas de
        vierpower_upper_left.
    */
    let vierpower_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    /*
        pixel00_loc es la ubicacion del pixel de la fila 0, columna 0.

        Simplemente nos movemos desde la esquina Q del viewport 0.5 para +x y 0.5 para -y, asi quedando
        en el centro del pixel 0,0.
    */
    let pixel00_loc = vierpower_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rLineas restantes: {} ", image_height - j);
        for i in 0..image_width {
            /*
                pixel_center se obtiene moviendonos 1 pixel desde el centro del pixel 0,0.
                Como ya estabamos en el centro, movernos para cualquier lado exactamente un pixel nos dejara en el centro de otro.
            */
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            // Obtenemos el vector que va del centro de la camara al centro del pixel
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            // Usamos ray_color con el rayo "r" para obtener su color
            let pixel_color: Color = ray_color(r);

            println!("{}", pixel_color)
        }
    }

    eprintln!("\rListo              \n");
}

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = center - r.origin();
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

pub fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r);

    if t > 0.0 {
        let n: Vec3 = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();

        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    // Convertimos r en un vector unitario, ahora  tiene una longitud de 1 unidad.
    let unit_direction: Vec3 = r.direction().unit_vector();

    /*
        Ahora como y esta entre -1 y 1, vamos a pasarlo al rango de
        entre 0 y 1 para luego calcular facilmente el color
    */
    let a = 0.5 * (unit_direction.y() + 1.0);

    /*
        Usamos Lerp (linear interpolation). Cuando a = 0 quiero blanco, cuando a = 1 quiero azul
        y cuando 0 < a < 1 quiero un color en el medio. Un lerp es de la forma:

        blendedValue = (1 - a) * startValue + a * endValue.

        En nuestro caso representa lo siguiente:

        blendedValue = (1 - a) * blanco * a * celeste

    */
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
