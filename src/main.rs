extern crate sdl2;
extern crate gl;
use std::ops::{Add, Sub};
use std::ffi::{CString, CStr};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

#[derive(Copy, Clone)]
struct R3 {
    x: f64,
    y: f64,
    z: f64  
}

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

impl R3 {
    fn new(x: f64, y: f64, z: f64) -> R3 {
        R3 {x, y, z}
    }

    fn zero() -> R3 {
        R3 {x:0.0, y:0.0, z:0.0}
    }

    fn len(&self) -> f64 {
        let x = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        x.sqrt()
    }

    fn dot(&self, u: R3) -> f64 {
        self.x * u.x + self.y * u.y + self.z * u.z
    }

    fn normalize(&self) -> R3 {
        let l = self.len();
        R3 {
            x: self.x / l, 
            y: self.y / l,
            z: self.z / l
        }
    }
}

fn compile_shader(shader_type: gl::types::GLenum, shader_src: &CStr) 
    -> Result<gl::types::GLenum, String> 
{
    // let shader_src = CString::new(source).unwrap();
    let mut success: gl::types::GLint = 0;
    let mut err_len: gl::types::GLint = 0;
    let shader_id: gl::types::GLuint = unsafe { gl::CreateShader(shader_type) };
    unsafe { 
        gl::ShaderSource(shader_id, 1, &shader_src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader_id);
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut buffer: Vec<u8> = Vec::with_capacity(err_len as usize + 1);
        // buffer.extend([b' '].iter().cycle().take(err_len as usize));
        let error: CString = unsafe { CString::from_vec_unchecked(buffer) };
        unsafe {
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut err_len);
            gl::GetShaderInfoLog(
                shader_id,
                err_len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        Err(error.to_string_lossy().into_owned())
    }
    else {
        Ok(shader_id)
    }
    
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3,3);

    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Error messages don't seem to actually work atm
    match compile_shader(gl::VERTEX_SHADER, &CString::new(include_str!("vertex.glsl")).unwrap()) {
        Ok(_) => (),
        Err(msg) => {
            eprintln!("Error compiling vertex shader: {}", msg);
            std::process::exit(1);
        }
    }

    match compile_shader(gl::FRAGMENT_SHADER, &CString::new(include_str!("fragment.glsl")).unwrap()) {
        Ok(_) => (),
        Err(msg) => {
            eprintln!("Error compiling fragment shader: {}", msg);
            std::process::exit(1);
        }
    }

    let mut screen_tex: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut screen_tex);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, screen_tex);
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGBA, 
            160,
            144,
            0, 
            gl::RGBA,
            gl::FLOAT, 
            
        );
    }
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit {..} = event {
                break 'main;
            }
        }
    }
}
