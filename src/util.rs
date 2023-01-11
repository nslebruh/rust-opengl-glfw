//?https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html

extern crate gl;
use std::ffi::{CString, CStr};

use gl::{types::*, VERTEX_SHADER, FRAGMENT_SHADER};


pub fn vert_shader_from_source(source: &CStr) -> Result<GLuint, String> {
    shader_from_source(source, VERTEX_SHADER)
}

pub fn frag_shader_from_source(source: &CStr) -> Result<GLuint, String> {
    shader_from_source(source, FRAGMENT_SHADER)
}


fn shader_from_source(source: &CStr, type_: GLenum) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(type_) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id)
    };

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        
        buffer.extend([b' '].iter().cycle().take(len as usize));
        
        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(id)
}


fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}