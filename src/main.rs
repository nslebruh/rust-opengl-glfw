mod util;

extern crate glfw;
extern crate gl;

use std::ffi::CString;

use util::*;
use gl::{types::*, ARRAY_BUFFER, TRIANGLES};
use glfw::{Action, Context, Key};



fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let (screen_width, screen_height) = window.get_framebuffer_size();

    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    let vertices: Vec<f32> = vec![
        -0.5, 0.0, 0.0,
        0.5, 0.0, 0.0,
        0.0, 0.5, 0.0
    ];

    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenBuffers(1,  &mut vbo);
    }

    unsafe {
        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    
    let vert = include_str!("../triangle.vert");
    let frag = include_str!("../triangle.frag");

    let vert_shader = vert_shader_from_source(&CString::new(vert).unwrap()).unwrap();
    let frag_shader = frag_shader_from_source(&CString::new(frag).unwrap()).unwrap();

    let program: GLuint = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vert_shader);
        gl::AttachShader(program, frag_shader);

        gl::LinkProgram(program);

        gl::DetachShader(program, vert_shader);
        gl::DetachShader(program, frag_shader);
    }

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);

    unsafe{
        gl::Viewport(0, 0, screen_width, screen_height);
        gl_clear_color(255, 119, 110, 255);
    }

    let target_fps: f64 = 60.0;
    let mut w = DigitalInputState::new(Key::W);
    let mut last_time = glfw.get_time();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut w);
        }

        let (w, h) = window.get_framebuffer_size();
        let _ratio: f32 = w as f32 / h as f32;

        unsafe {
            gl::Viewport(0, 0, w, h);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        unsafe { gl::UseProgram(program); }
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(TRIANGLES, 0, 3)
        }

        while glfw.get_time() < last_time + 1.0 / target_fps {
        }
        last_time += 1.0 / target_fps;

        window.swap_buffers();

    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, w: &mut DigitalInputState) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        glfw::WindowEvent::Key(Key::W, _, action, _) if action != Action::Repeat => {
            w.toggle();
        },
        glfw::WindowEvent::CursorPos(x, y) => {
            println!("x: {}, y: {}", x, y)
        },
        glfw::WindowEvent::MouseButton(mouse_button, action, _) => {
            println!("mouse_button: {:?}, action: {:?}", mouse_button, action)
        },
        glfw::WindowEvent::Scroll(x, y) => {
            println!("x: {}, y: {}", x, y)
        }

        _ => {}
    }
}