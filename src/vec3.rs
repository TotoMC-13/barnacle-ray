use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::interval::Interval;
use crate::utils::{linear_to_gamma, random_double, random_double_range};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    // Constructor con parametros
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    // Getters
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self[0] * v[0] + self[1] * v[1] + self[2] * v[2]
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            self.e[2] * v.e[0] - self.e[0] * v.e[2],
            self.e[0] * v.e[1] - self.e[1] * v.e[0],
        )
    }

    pub fn length(&self) -> f64 {
        (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn unit_vector(&self) -> Vec3 {
        let norm = self.length();

        Vec3::new(self[0] / norm, self[1] / norm, self[2] / norm)
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            // Generamos un vector en el cubo [-1, 1]
            let p = Vec3::random_range(-1.0, 1.0);

            // Si el cuadrado de su longitud es < 1, está dentro de la esfera
            if p.length_squared() < 1.0 {
                return p;
            }
            // Si no, el loop sigue y "rechaza" el anterior
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        // v - 2 * dot(v, n) * n
        v - 2.0 * v.dot(n) * n
    }

    pub fn near_zero(&self) -> bool {
        // Definimos un valor muy pequeño (epsilon)
        let s = 1e-8;
        // Retorna true si las tres componentes están muy cerca de cero
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }

    pub fn refract(r: Vec3, n: Vec3, refraction_ratio: f64) -> Vec3 {
        // Calculamos el componente perpendicular de r
        let r_perp: Vec3 = refraction_ratio * (r + r.dot(n) * -1.0 * n);
        // Calculamos el componente paralelo de r
        let r_paralell: Vec3 = (1.0 - r_perp.length_squared()).abs().sqrt() * -n;

        r_perp + r_paralell
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

// Trait Default
impl Default for Vec3 {
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

// Para lectura: v[i]
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

// Para escritura: v[i] = val
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

// Suma de vectores: v1 + v2
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

// Resta de vectores: v1 - v2
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

// Multiplicación por escalar: v * s
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3::new(self[0] * scalar, self[1] * scalar, self[2] * scalar)
    }
}

// Multiplicación componente a componente: v1 * v2
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

// Multiplicación por escalar (conmutativa): s * v
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// División por escalar: v / s
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        Vec3::new(self[0] / scalar, self[1] / scalar, self[2] / scalar)
    }
}

// Negación: -v
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

// Suma y asignación: v1 += v2
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self[0] = self[0] + other[0];
        self[1] = self[1] + other[1];
        self[2] = self[2] + other[2];
    }
}

// Resta y asignación: v1 -= v2
impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self[0] = self[0] - other[0];
        self[1] = self[1] - other[1];
        self[2] = self[2] - other[2];
    }
}

// Multiplicación por escalar y asignación: v *= s
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self[0] = self[0] * scalar;
        self[1] = self[1] * scalar;
        self[2] = self[2] * scalar;
    }
}

// Multiplicación componente a componente y asignación: v1 *= v2
impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self[0] = self[0] * other[0];
        self[1] = self[1] * other[1];
        self[2] = self[2] * other[2];
    }
}

// División por escalar y asignación: v /= s
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        self[0] = self[0] / scalar;
        self[1] = self[1] / scalar;
        self[2] = self[2] / scalar;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

// Formateo para impresión (PPM)
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);

        let interval: Interval = Interval::new(0.000, 0.999);

        let rbyte = (256.0 * interval.clamp(r as f64)) as u8;
        let gbyte = (256.0 * interval.clamp(g as f64)) as u8;
        let bbyte = (256.0 * interval.clamp(b as f64)) as u8;

        write!(f, "{} {} {}", rbyte, gbyte, bbyte)
    }
}
