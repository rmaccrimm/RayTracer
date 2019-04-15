#![allow(dead_code)]
use std::ops::{Add, Sub, Mul};

pub mod shape;
pub mod viewport;

#[derive(Copy, Clone, Debug)]
pub struct R3 {
    pub x: f64,
    pub y: f64,
    pub z: f64, 
}

#[derive(Debug)]
pub struct Ray {
    pub pt: R3,
    pub dir: R3,
}

impl Ray {
    pub fn new(pt: R3, dir: R3) -> Ray {
        Ray { pt, dir }
    }
}

pub const X: R3 = R3 { x: 1.0, y: 0.0, z: 0.0 };
pub const Y: R3 = R3 { x: 0.0, y: 1.0, z: 0.0 };
pub const Z: R3 = R3 { x: 0.0, y: 0.0, z: 1.0 };

impl Add for R3 {
    type Output = R3;

    fn add(self, rhs: R3) -> R3 {
        R3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for R3 {
    type Output = R3;

    fn sub(self, rhs: R3) -> R3 {
        R3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for R3 {
    type Output = R3;

    fn mul(self, rhs: f64) -> R3 {
        R3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<R3> for f64 {
    type Output = R3;

    fn mul(self, rhs: R3) -> R3 {
        R3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl R3 {
    pub fn new(x: f64, y: f64, z: f64) -> R3 {
        R3 { x, y, z }
    }

    pub fn zero() -> R3 {
        R3 {x:0.0, y:0.0, z:0.0}
    }

    pub fn norm(&self) -> f64 {
        let c = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        c.sqrt()
    }

    pub fn dot(&self, u: R3) -> f64 {
        self.x * u.x + self.y * u.y + self.z * u.z
    }

    pub fn normalize(&self) -> R3 {
        let l = self.norm();
        R3 {
            x: self.x / l, 
            y: self.y / l,
            z: self.z / l
        }
    }
}