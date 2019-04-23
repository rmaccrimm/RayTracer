mod geometry;

use sdl2::{
    rect::Point, 
    video::Window, 
    pixels::Color, 
    render::Canvas
};
use geometry::{
    R3,
    Ray,
    Intersect,
    shape::Sphere,
    Viewport,
    Scene
};

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

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut canvas = get_canvas(&sdl_context);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let viewport = Viewport::new(800, 600, 1.0);
    let s = Sphere::new(R3::new(0.0, 0.0, 5.0), 2.0);
    let t = Sphere::new(R3::new(-3.0, 1.0, 8.0), 2.0);
    
    for i in 0..viewport.w {
        for j in 0..viewport.h {
            let r = viewport.get_ray(i, j);
            let mut color = match t.intersect(r) {
                Some(refl) => {
                    let mut i = refl.dir.dot(-1.0 * geometry::X);
                    i = if i < 0.0 {
                        0.0
                    }
                    else {
                        i
                    };
                    let r = (i * 255.0) as u8;
                    Color::RGB(r, 0, 0)
                }
                None => Color::RGB(255, 255, 255)
            };
            let r = viewport.get_ray(i, j);
            color = match s.intersect(r) {
                Some(refl) => {
                    let mut i = refl.dir.dot(-1.0 * geometry::X);
                    i = if i < 0.0 {
                        0.0
                    }
                    else {
                        i
                    };
                    let g = (i * 255.0) as u8;
                    Color::RGB(0, g, 0)
                }
                None => color
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
