use super::{R3, Ray};

pub trait Intersect {
    fn intersect(&self, r: Ray) -> Option<Ray>;
}

pub struct Sphere {
    c: R3,
    r: f64,
}

impl Sphere {
    pub fn new(center: R3, radius: f64) -> Sphere {
        Sphere {
            c: center,
            r: radius,
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, r: Ray) -> Option<Ray> {
        let l = r.dir.normalize();
        let o = r.pt;
        let c = self.c;
        let rad = self.r;
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