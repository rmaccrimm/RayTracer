use super::{Ray, R3, X, Y, Z};

#[derive(Debug)]
pub struct Viewport {
    // Position of camera in space
    pub eye: R3,
    pub dir: R3,
    // Height and width in pixesl
    pub h: i32,
    pub w: i32,
    // Horizontal field of view, in radians
    fov: f64,
    // Distance from eye to viewport
    clip: f64,
    // Actual width and height
    height: f64,
    width: f64,
    // Top left corner, relative eye, assumes static dir in z direction
    origin: R3,
    dx: f64,
    dy: f64,
}

impl Viewport {
    pub fn new(w: i32, h: i32, clip: f64) -> Viewport {
        let eye = R3::new(0.0, 0.0, 0.0);
        let dir = R3::new(0.0, 0.0, 1.0);
        let fov: f64 = 90.0;
        let width = 2.0 * fov.to_radians().sin() * clip;
        let height = (h as f64) / (w as f64) * width;
        let x = R3::new(1.0, 0.0, 0.0);
        let y = R3::new(0.0, 1.0, 0.0);
        let origin = (eye + clip * dir) - (width / 2.0) * x + (height / 2.0) * y;
        let dx = width / (w as f64);
        let dy = height / (h as f64);
        Viewport { eye, dir, fov, h, w, clip, height, width, origin, dx, dy }
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        let o = self.origin + (self.dx * X * (i as f64)) + (self.dy * Y * (-j as f64));
        let dir = o.normalize() - self.eye;
        Ray {
            dir,
            pt: o,
        }
    }
}