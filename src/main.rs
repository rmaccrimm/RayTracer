extern crate sdl2;
mod vec;
use sdl2::{
    rect::Point, 
    video::Window, 
    pixels::Color, 
    render::Canvas
};
use std::f64;

#[derive(Debug)]
struct Ray {
    dir: vec::R3,
    pt: vec::R3,
}

struct Sphere {
    c: vec::R3,
    r: f64,
}

fn intersect_sphere(r: Ray, s: &Sphere) -> Option<Ray> {
    let l = r.dir.normalize();
    let o = r.pt;
    let c = s.c;
    let rad = s.r;
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

    let reflected = Ray { dir: reflect_dir, pt: inter_pt };
    Some(reflected)
}

fn get_canvas(context: &sdl2::Sdl) -> Canvas<Window> {
    
    let video_subsystem = context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .build()
        .unwrap();
    let canvas = window.into_canvas()
        .build()
        .unwrap();
    canvas
}

#[derive(Debug)]
struct Viewport {
    // Position of camera in space
    eye: vec::R3,
    dir: vec::R3,
    // Horizontal field of view, in radians
    fov: f64,
    // Height and width in pixesl
    h: i32,
    w: i32,
    // Distance from eye to viewport
    clip: f64,
    // Actual width and height
    height: f64,
    width: f64,
    // Top left corner, relative eye, assumes static dir in z direction
    origin: vec::R3,
    dx: f64,
    dy: f64,
}

impl Viewport {
    fn new(w: i32, h: i32, clip: f64) -> Viewport {
        let eye = vec::R3::new(0.0, 0.0, 0.0);
        let dir = vec::R3::new(0.0, 0.0, 1.0);
        let fov: f64 = 90.0;
        let width = 2.0 * fov.to_radians().sin() * clip;
        let height = (h as f64) / (w as f64) * width;
        let x = vec::R3::new(1.0, 0.0, 0.0);
        let y = vec::R3::new(0.0, 1.0, 0.0);
        let origin = (eye + clip * dir) - (width / 2.0) * x + (height / 2.0) * y;
        let dx = width / (w as f64);
        let dy = height / (h as f64);
        Viewport { eye, dir, fov, h, w, clip, height, width, origin, dx, dy }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let o = self.origin + (self.dx * vec::X * (i as f64)) + (self.dy * vec::Y * (-j as f64));
        let dir = o.normalize() - self.eye;
        Ray {
            dir,
            pt: o,
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = get_canvas(&sdl_context);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let viewport = Viewport::new(800, 600, 1.0);
    let s = Sphere {
        r: 2.0,
        c: vec::R3::new(0.0, 0.0, 5.0)
    };

    'outer: for i in 0..viewport.w {
        'inner: for j in 0..viewport.h {
            let r = viewport.get_ray(i, j);
            let color = match intersect_sphere(r, &s) {
                Some(_) => Color::RGB(0, 0, 0),
                None => Color::RGB(255, 255, 255)
            };
            canvas.set_draw_color(color);
            canvas.draw_point(Point::new(i, j)).unwrap();
        }
    }
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit {..} = event {
                break 'main;
            }
        }
    }
}
