use gl::types::*;
use std::ffi::CString;
use std::{fmt, fs, ptr};

pub struct Shader {
    id: GLuint,
}

#[derive(Debug)]
pub struct ShaderError(String);

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ShaderError {}

impl From<std::io::Error> for ShaderError {
    fn from(e: std::io::Error) -> Self {
        Self(format!("failed to open file: {}", e))
    }
}

impl From<std::ffi::NulError> for ShaderError {
    fn from(e: std::ffi::NulError) -> Self {
        Self(format!("{}", e))
    }
}

impl Shader {
    pub fn new(vert_path: &str, frag_path: &str) -> Result<Self, ShaderError> {
        let vert_code = fs::read_to_string(vert_path)?;
        let frag_code = fs::read_to_string(frag_path)?;

        let vert = Self::compile_shader(&vert_code, gl::VERTEX_SHADER)?;
        let frag = Self::compile_shader(&frag_code, gl::FRAGMENT_SHADER)?;

        let id = Self::link_program(vert, frag)?;

        unsafe {
            gl::DeleteShader(vert);
            gl::DeleteShader(frag);
        }

        Ok(Self { id })
    }

    fn compile_shader(src: &str, t: GLenum) -> Result<GLuint, ShaderError> {
        unsafe {
            let c_str = CString::new(src)?;
            let shader = gl::CreateShader(t);
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            if status != (gl::TRUE as GLint) {
                let mut buf = [0u8; 512];
                let mut len = 0;
                gl::GetShaderInfoLog(shader, 512, &mut len, buf.as_mut_ptr() as *mut GLchar);
                return Err(ShaderError(format!(
                    "error compiling shader: {}",
                    String::from_utf8_lossy(&buf[..len as usize]),
                )));
            }
            Ok(shader)
        }
    }

    fn link_program(vs: GLuint, fs: GLuint) -> Result<GLuint, ShaderError> {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
            if status != (gl::TRUE as GLint) {
                let mut buf = [0u8; 512];
                let mut len = 0;
                gl::GetProgramInfoLog(program, 512, &mut len, buf.as_mut_ptr() as *mut GLchar);
                return Err(ShaderError(format!(
                    "error linking program: {}",
                    String::from_utf8_lossy(&buf[..len as usize]).replace("\\n", "\n"),
                )));
            }

            Ok(program)
        }
    }

    #[inline(always)]
    pub fn enable(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    #[inline(always)]
    pub fn get_uniform(&self, name: &str) -> GLint {
        let c_str = CString::new(name).unwrap();
        self.enable();
        unsafe { gl::GetUniformLocation(self.id, c_str.as_ptr()) }
    }

    #[inline(always)]
    pub fn set_vec1(&self, name: &str, value: &glm::Vec1) {
        unsafe { gl::Uniform1f(self.get_uniform(name), value.x) }
    }
    #[inline(always)]
    pub fn set_vec2(&self, name: &str, value: &glm::Vec2) {
        unsafe { gl::Uniform2f(self.get_uniform(name), value.x, value.y) }
    }
    #[inline(always)]
    pub fn set_vec3(&self, name: &str, value: &glm::Vec3) {
        unsafe { gl::Uniform3f(self.get_uniform(name), value.x, value.y, value.z) }
    }
    #[inline(always)]
    pub fn set_mat4(&self, name: &str, mat: &glm::Mat4) {
        unsafe { gl::UniformMatrix4fv(self.get_uniform(name), 1, gl::FALSE, mat.as_ptr()) }
    }
}

impl Drop for Shader {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
