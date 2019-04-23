use super::{
    R3, 
    Ray, 
    Intersect
};

pub struct Sphere {
    pub center: R3,
    pub radius: f64,
}

pub struct Plane {
    pub center: R3,
    pub normal: R3,
    pub perp: R3, // Points in direction of width
    pub width: f64,
    pub height: f64,
}

pub struct Cube {
    pub center: R3,
    pub up: R3,
    pub width: f64
}

impl Sphere {
    pub fn new(center: R3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, r: Ray) -> Option<Ray> {
        let l = r.dir.normalize();
        let o = r.pt;
        let c = self.center;
        let rad = self.radius;
        let x = (l.dot(o - c)).powi(2) - ((o - c).norm().powi(2) - rad.powi(2));
        if x < 0.0 {
            return None;
        }
        // Take closest (or only if x = 0) intersection point, with dist along r given by
        let dist = -2.0 * l.dot(o - c) - x.sqrt() / (2.0 * l.norm().powi(2));
        let inter_pt = o + (dist * l);
        // Normal to sphere at intersection point
        let n = (inter_pt - c).normalize();
        let reflect_dir = 2.0 * (n.dot(l) * n) - l;

        let reflected = Ray::new(inter_pt, reflect_dir);
        Some(reflected)
    }
}