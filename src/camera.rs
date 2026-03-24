use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::{f64::INFINITY, time::Instant};

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils::{degrees_to_radians, format_with_dots, random_double},
    vec3::{Color, Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f64,      // 1.0 (Ej.)
    pub image_width: u32,       // 100 (Ej.)
    pub samples_per_pixel: u32, // 10  (Ej.)
    pub max_depth: i32,         // 10  (Ej.)
    pub sky_emits_light: bool,
    pub vfov: f64, // 90.0 (Ej.)
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: u32,
    pixel_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    fn initialize(&mut self) {
        let im_height = (self.image_width as f64 / self.aspect_ratio) as u32;

        self.image_height = if im_height > 1 { im_height } else { 1 };

        self.center = self.lookfrom;

        self.pixel_sample_scale = 1.0 / (self.samples_per_pixel as f64);

        // Sistema de coordenadas local a la camara
        self.w = (self.lookfrom - self.lookat).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        // Dimensiones viewport
        let theta = degrees_to_radians(self.vfov);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        /*
            Calcular los vectores horizontales y verticales por los bordes del viewport (V_u y V_v)
            Estos son los vectores que nos ayudaran a movernos a lo largo del viewport

            V_u va para el +x
            V_v va para el -y
        */
        let viewport_u = viewport_width * self.u; // V_u
        let viewport_v = viewport_height * (-self.v); // V_v

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
            viewport_upper_left.
        */
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        // Calculamos los vectores base del disco de desenfoque
        let defocus_radius: f64 = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0);
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        /*
            pixel00_loc es la ubicacion del pixel de la fila 0, columna 0.

            Simplemente nos movemos desde la esquina Q del viewport 0.5 para +x y 0.5 para -y, asi quedando
            en el centro del pixel 0,0.
        */
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn _old_ray_color(
        &self,
        r: &Ray,
        world: &dyn Hittable,
        _depth: i32,
        _total_rays: &mut u64,
    ) -> Color {
        let mut rec: HitRecord = HitRecord::default();

        /*
            Le tiramos el rayo al mundo. t_min es 0.001 para evitar bugs de precisión, t_max es infinito.
            El rec va del .hit del world al del objeto al que chequeamos la colision, el objeto devuelve si le pegamos
            o no y despues procede a modificar el valor a donde apunta el puntero de rec, asi recuperando la info (record)
            de la colision y pudiendo obtener la normal con rec.normal en el return
        */
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
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

    pub fn ray_color(
        &self,
        r: &Ray,
        world: &dyn Hittable,
        depth: i32,
        total_rays: &mut u64,
    ) -> Color {
        *total_rays += 1;

        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();

        // 1. Intentamos el choque con el mundo
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            // 2. Extraemos el material del Option y llamamos a scatter
            if let Some(mat) = &rec.mat {
                let emited_color = mat.emitted();

                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    // 3. Color = Atenuación * Color del siguiente rebote
                    return emited_color
                        + attenuation * self.ray_color(&scattered, world, depth - 1, total_rays);
                }

                return emited_color;
            }

            // 4. Si el material absorbe el rayo o no hay material, devolvemos negro
            return Color::new(0.0, 0.0, 0.0);
        }

        if self.sky_emits_light {
            // Si no choca con nada, devuelve el fondo (cielo)
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
        } else {
            Color::default()
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let start = Instant::now();
        let total_primary_rays =
            self.image_width as u64 * self.image_height as u64 * self.samples_per_pixel as u64;

        // Contadores atomicos compartidos entre todos los hilos
        let lines_remaining = AtomicU32::new(self.image_height);
        let total_rays_atomic = AtomicU64::new(0);

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        // Creamos un vector con los numeros de fila (0, 1, 2... image_height - 1)
        let rows: Vec<u32> = (0..self.image_height).collect();

        // Procesamos FILA por FILA en paralelo
        let buffer_colores: Vec<Color> = rows
            .into_par_iter()
            .flat_map(|j| {
                let mut row_colors = Vec::with_capacity(self.image_width as usize);
                let mut local_row_rays = 0; // Contador exclusivo de este hilo/fila

                for i in 0..self.image_width {
                    let mut pixel_color = Color::default();
                    for _ in 0..self.samples_per_pixel {
                        let r: Ray = self.get_ray(i, j);
                        // Le pasamos nuestro contador local a la función existente
                        pixel_color +=
                            self.ray_color(&r, world, self.max_depth, &mut local_row_rays);
                    }
                    row_colors.push(pixel_color * self.pixel_sample_scale);
                }

                // Al terminar la fila, volcamos los rayos locales al total atómico global de forma segura
                let current_total_rays =
                    total_rays_atomic.fetch_add(local_row_rays, Ordering::Relaxed) + local_row_rays;
                let remaining = lines_remaining.fetch_sub(1, Ordering::Relaxed) - 1;

                // Actualizamos la consola en tiempo real
                let elapsed_secs = start.elapsed().as_secs_f64();
                let mrps = if elapsed_secs > 0.0 {
                    (current_total_rays as f64 / elapsed_secs) / 1_000_000.0
                } else {
                    0.0
                };

                eprint!(
                    "\rLineas restantes: {:<4} | MRays/s: {:.2} | Elapsed: {:.2?}    ",
                    remaining,
                    mrps,
                    start.elapsed()
                );

                // Devolvemos el array de colores de esta fila
                row_colors
            })
            .collect();

        // Imprimimos el resultado final secuencialmente para que el PPM no se rompa
        for color in buffer_colores {
            println!("{}", color);
        }

        // Recuperamos el total exacto final y calculamos stats
        let total_rays = total_rays_atomic.load(Ordering::Relaxed);
        let duration = start.elapsed();
        let final_mrps = (total_rays as f64 / duration.as_secs_f64()) / 1_000_000.0;
        let total_bounces = total_rays - total_primary_rays;

        eprintln!("\rListo                                                    ");
        eprintln!("Tiempo de renderizado: {:.2?}", duration);
        eprintln!(
            "Rayos primarios totales: {}",
            format_with_dots(total_primary_rays)
        );
        eprintln!("Rebotes totales: {}", format_with_dots(total_bounces));
        eprintln!("Rayos procesados totales: {}", format_with_dots(total_rays));
        eprintln!("Promedio final de MRays/s: {:.2}", final_mrps);
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        /*
            Construimos un rayo originado en el radio de desenfoque y dirigido a un lugar
            random alrededor de la ubicacion del pixel i, j
            Es decir, hacemos un rayo que apunte al pixel que queremos y le metemos el
            offset para que varie la parte del pixel al que llega
        */
        let offset = self.sample_square();

        let pixel_sample = self.pixel00_loc // Desde el pixel 00
            + ((i as f64 + offset.x()) * self.pixel_delta_u) // Nos movemos a la derecha i * offset veces
            + ((j as f64 + offset.y()) * self.pixel_delta_v); // Nos movemos para abajo j * offset veces

        // Armamos un rayo desde el centro de la camara hasta la posicion que
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    // Devuelve un vector para aplicar el offset necesario al rayo
    pub fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    pub fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            sky_emits_light: true,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 0.0,
            // Variables internas de la camara (se calculan en initialize)
            image_height: 0,
            pixel_sample_scale: 0.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}
