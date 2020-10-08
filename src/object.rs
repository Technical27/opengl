use super::shader::*;
use gl::types::*;
use std::{fmt, mem, ptr};

#[derive(Debug, Clone)]
pub struct ObjectError(String);

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ObjectError {}

impl From<image::ImageError> for ObjectError {
    fn from(_: image::ImageError) -> ObjectError {
        Self("failed to open image".to_string())
    }
}

impl From<ShaderError> for ObjectError {
    fn from(e: ShaderError) -> ObjectError {
        Self(format!("failed to compile shader: {}", e))
    }
}

pub struct Object {
    pub shader: Shader,
    vert_count: GLint,
    tex: GLuint,
    vbo: GLuint,
    vao: GLuint,
    ebo: GLuint,
}

const GL_FLOAT_SIZE: usize = mem::size_of::<GLfloat>();
const GL_UINT_SIZE: usize = mem::size_of::<GLuint>();

impl Object {
    pub fn new(
        vertices: &[GLfloat],
        indices: &[GLuint],
        vert_path: &str,
        frag_path: &str,
        tex_path: &str,
    ) -> Result<Self, ObjectError> {
        unsafe {
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * GL_FLOAT_SIZE) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * GL_UINT_SIZE) as GLsizeiptr,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            let img = image::open(tex_path)?.into_rgba();
            let width = img.width();
            let height = img.height();
            let pixels = img.as_flat_samples().samples;

            let mut tex = 0;
            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                pixels as *const _ as *const GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            let shader = Shader::new(vert_path, frag_path)?;
            shader.enable();

            let vert_count = indices.len() as i32;

            Ok(Self {
                shader,
                vert_count,
                tex,
                vbo,
                vao,
                ebo,
            })
        }
    }

    pub fn vertex_attrib(&self, location: GLuint, size: GLint, stride: usize, offset: usize) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::VertexAttribPointer(
                location,
                size,
                gl::FLOAT,
                gl::FALSE,
                (stride * GL_FLOAT_SIZE) as GLint,
                (offset * GL_FLOAT_SIZE) as *mut GLvoid,
            );
            gl::EnableVertexAttribArray(location);
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.shader.enable();
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::DrawElements(
                gl::TRIANGLES,
                self.vert_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.tex);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
