use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::interval::Interval;

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
        let r = self.x();
        let g = self.y();
        let b = self.z();

        let interval: Interval = Interval::new(0.000, 0.999);

        let rbyte = (256.0 * interval.clamp(r as f64)) as u8;
        let gbyte = (256.0 * interval.clamp(g as f64)) as u8;
        let bbyte = (256.0 * interval.clamp(b as f64)) as u8;

        write!(f, "{} {} {}", rbyte, gbyte, bbyte)
    }
}
