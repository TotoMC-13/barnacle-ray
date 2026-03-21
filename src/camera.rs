use std::{f64::INFINITY, time::Instant};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils::random_double,
    vec3::{Color, Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,      // 1.0 (Ej.)
    pub image_width: u32,       // 100 (Ej.)
    pub samples_per_pixel: u32, // 10  (Ej.)
    pub max_depth: i32,         // 10  (Ej.)
    image_height: u32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: i32,
    ) -> Camera {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            pixel_sample_scale: 1.0 / (samples_per_pixel as f64),
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }

    fn initialize(&mut self) {
        let im_height = (self.image_width as f64 / self.aspect_ratio) as u32;

        self.image_height = if im_height > 1 { im_height } else { 1 };

        self.center = Point3::default();

        // Dimensiones viewport

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

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
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

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
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        /*
            pixel00_loc es la ubicacion del pixel de la fila 0, columna 0.

            Simplemente nos movemos desde la esquina Q del viewport 0.5 para +x y 0.5 para -y, asi quedando
            en el centro del pixel 0,0.
        */
        self.pixel00_loc = vierpower_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn _old_ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec: HitRecord = HitRecord::default();

        /*
            Le tiramos el rayo al mundo. t_min es 0.001 para evitar bugs de precisión, t_max es infinito.
            El rec va del .hit del world al del objeto al que chequeamos la colision, el objeto devuelve si le pegamos
            o no y despues procede a modificar el valor a donde apunta el puntero de rec, asi recuperando la info (record)
            de la colision y pudiendo obtener la normal con rec.normal en el return
        */
        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            // Si choco con algo, calculamos el color en base a la normal de ese objeto ganador
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().unit_vector();

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

    pub fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();
        // Usamos 0.001 para ignorar choques muy cercanos
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            // 1. Generamos la direccion aleatoria
            let direction = rec.normal + Vec3::random_in_unit_sphere().unit_vector();

            // 2. El color es 50% el color de lo que sea que el rebote encuentre
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), world, depth - 1);
        }

        // Si no choca con nada, devuelve el fondo (cielo)
        let unit_direction = r.direction().unit_vector();

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

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        // Iniciar timer para ver cuanto tarda
        let start = Instant::now();

        // Headers para el .ppm
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Recorremos las filas
        for j in 0..self.image_height {
            eprint!("\rLineas restantes: {} ", self.image_height - j);
            // Recorremos las columnas
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();

                // Tomamos n samples de n rayos y vamos promediando el color
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world, self.max_depth);
                }

                // Calculamos el promedio de los colores obtenidos
                pixel_color = pixel_color * self.pixel_sample_scale;
                println!("{}", pixel_color)
            }
        }

        // Vemos cuanto tardo en renderizar
        let duration = start.elapsed();

        eprintln!("\rListo              \n");
        eprintln!("Tiempo de renderizado: {:.2?}", duration);
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        /*
            Construimos un rayo desde el origen y dirigido a un lugar random alrededor de la ubicacion del pixel i, j
            Es decir, hacemos u nrayo que apunte al pixel que queremos y le metemos el
            offset para que variar la parte del pixel
        */
        let offset = self.sample_square();

        let pixel_sample = self.pixel00_loc // Desde el pixel 00
            + ((i as f64 + offset.x()) * self.pixel_delta_u) // Nos movemos a la derecha i * offset veces
            + ((j as f64 + offset.y()) * self.pixel_delta_v); // Nos movemos para abajo j * offset veces

        // Armamos un rayo desde el centro de la camara hasta la posicion que
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    // Devuelve un vector para aplicar el offset necesario al rayo
    pub fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
