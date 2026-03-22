use std::{f64::consts::PI, sync::atomic::AtomicU64, sync::atomic::Ordering};

static SEED: AtomicU64 = AtomicU64::new(1);

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let mut current = SEED.load(Ordering::Relaxed);
    loop {
        // Hacemos el Xorshift en una variable temporal
        let mut next = current;
        if next == 0 {
            next = 1;
        }
        next ^= next << 13;
        next ^= next >> 7;
        next ^= next << 17;

        /*
            Aca decimos: Yo empece diciendoque la semilla valia current, ese valor sigue en seed?
            Se parte en 2 casos:
                Ok(_): Esto quiere decir que SEED sigue siendo igual a current, es decir. Nadie modifico nada (otro hilo)
                entonces cambio la semilla global (SEED) por mi nuevo valor de next y devuelvo el numero random.

                Err(v): Esto quiere decir que la SEED cambio y no es igual a current
                (otro hilo genero un numero, se me adelanto) entonces hago que current = v, es decir, que current sea
                el valor nuevo, despues de eso sigue el loop e intenta generar el numero de nuevo.
        */
        match SEED.compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return (next as f64) / (u64::MAX as f64),
            Err(v) => current = v,
        }
    }
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Lerp para meter el numero en el rango
    min + (max - min) * random_double()
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}

pub fn format_with_dots(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().rev().collect();

    for (i, c) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push('.');
        }
        result.push(*c);
    }
    result.chars().rev().collect()
}
