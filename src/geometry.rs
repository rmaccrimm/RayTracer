// Re-export items in private sub-module 
pub use viewport::Viewport;
pub use vector::{ R3, X, Y, Z };
pub use scene::Scene;

pub mod shape;
pub mod light;
mod viewport;
mod scene;
mod vector;

// Trait for things that can be tested for ray intersection
pub trait Intersect {
    fn intersect(&self, r: Ray) -> Option<Ray>;
}

#[derive(Debug)]
pub struct Ray {
    pub pt: R3,     // origin of ray
    pub dir: R3,    // unit vector in direction of ray 
}

impl Ray {
    pub fn new(pt: R3, dir: R3) -> Ray {
        Ray { 
            pt, 
            dir: dir.normalize() 
        }
    }
}

