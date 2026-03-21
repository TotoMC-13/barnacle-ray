use std::{f64::INFINITY, time::Instant};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64, // 1.0
    pub image_width: i32,  // 100
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }

    fn initialize(&mut self) {
        let im_height = (self.image_width as f64 / self.aspect_ratio) as i32;

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

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
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

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        // Iniciar timer para ver cuanto tarda
        let start = Instant::now();

        // Headers para el .ppm
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rLineas restantes: {} ", self.image_height - j);
            for i in 0..self.image_width {
                /*
                    pixel_center se obtiene moviendonos 1 pixel desde el centro del pixel 0,0.
                    Como ya estabamos en el centro, movernos para cualquier lado
                    exactamente un pixel nos dejara en el centro de otro.
                */
                let pixel_center = self.pixel00_loc
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);

                // Obtenemos el vector que va del centro de la camara al centro del pixel
                let ray_direction = pixel_center - self.center;
                let r: Ray = Ray::new(self.center, ray_direction);

                // Usamos ray_color con el rayo "r" para obtener su color
                let pixel_color: Color = self.ray_color(&r, world);

                println!("{}", pixel_color)
            }
        }

        // Vemos cuanto tardo en renderizar
        let duration = start.elapsed();

        eprintln!("\rListo              \n");
        eprintln!("Tiempo de renderizado: {:.2?}", duration);
    }
}
