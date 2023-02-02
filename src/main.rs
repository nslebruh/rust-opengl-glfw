mod game_controller;
mod shader;
mod util;
mod engine;

extern crate glfw;
extern crate gl;
extern crate lazy_static;
extern crate image;

use crate::engine::generation::*;
use std::{mem::size_of, sync::mpsc::Receiver, path::Path, ffi::c_void};
use cgmath::{Matrix4, vec3, Rad, perspective, Deg, InnerSpace, Vector3, Point3};
use game_controller::GameController;

use shader::Shader;
use util::*;
use gl::{types::*, ARRAY_BUFFER, TRIANGLES};
use glfw::Key;

use engine::{keybinds::{KeyBinding, InputFunctionArguments}, input_functions::*, camera::Camera, window::Window};
fn main() {
    let scr_width: u32 = 1280;
    let scr_height: u32 = 720;

    let img = image::open(&Path::new("images.png")).unwrap().to_rgba();
    let data = img.to_vec();

    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        ..Default::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = scr_width as f32 / 2.0;
    let mut last_y: f32 = scr_height as f32 / 2.0;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    let mut window: Window = Window::init(scr_width, scr_height, "test title", glfw::WindowMode::Windowed, vec![glfw::WindowHint::ContextVersion(3, 3), glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)]).unwrap(); 

    //let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    //glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    //glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    //#[cfg(target_os = "macos")]
    //glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    //let (mut window, events) = glfw.create_window(scr_width, scr_height, "Hello this is window", glfw::WindowMode::Windowed)
    //   .expect("Failed to create GLFW window.");


    gl::load_with(|ptr| window.window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        //gl::Enable(gl::CULL_FACE);

    }

    window.make_current();
    window.window.set_cursor_pos_polling(true);
    window.window.set_scroll_polling(true);
    window.window.set_framebuffer_size_polling(true);
    window.window.set_cursor_mode(glfw::CursorMode::Disabled);


    let mut keybindings: Vec<KeyBinding> = vec![
        KeyBinding::new(Key::Escape, false, set_window_should_close),
        KeyBinding::new(Key::W, true, camera_forward),
        KeyBinding::new(Key::A, true, camera_left),
        KeyBinding::new(Key::S, true, camera_backward),
        KeyBinding::new(Key::D, true, camera_right),
        KeyBinding::new(Key::Space, true, camera_up),
        KeyBinding::new(Key::LeftShift, true, camera_down),
        KeyBinding::new(Key::RightShift, false, toggle_cursor_mode),
        KeyBinding::new(Key::Enter, false, toggle_cursor_mode_2),
        KeyBinding::new(Key::LeftControl, false, print_camera_pos)
    ];
    //let vertices: Vec<f32> = vec![
    //    -0.5, -0.5, -0.5,  0.0, 0.0,
    //     0.5, -0.5, -0.5,  1.0, 0.0,
    //     0.5,  0.5, -0.5,  1.0, 1.0,
    //     0.5,  0.5, -0.5,  1.0, 1.0,
    //    -0.5,  0.5, -0.5,  0.0, 1.0,
    //    -0.5, -0.5, -0.5,  0.0, 0.0,
//
    //    -0.5, -0.5,  0.5,  0.0, 0.0,
    //     0.5, -0.5,  0.5,  1.0, 0.0,
    //     0.5,  0.5,  0.5,  1.0, 1.0,
    //     0.5,  0.5,  0.5,  1.0, 1.0,
    //    -0.5,  0.5,  0.5,  0.0, 1.0,
    //    -0.5, -0.5,  0.5,  0.0, 0.0,
//
    //    -0.5,  0.5,  0.5,  1.0, 0.0,
    //    -0.5,  0.5, -0.5,  1.0, 1.0,
    //    -0.5, -0.5, -0.5,  0.0, 1.0,
    //    -0.5, -0.5, -0.5,  0.0, 1.0,
    //    -0.5, -0.5,  0.5,  0.0, 0.0,
    //    -0.5,  0.5,  0.5,  1.0, 0.0,
//
    //     0.5,  0.5,  0.5,  1.0, 0.0,
    //     0.5,  0.5, -0.5,  1.0, 1.0,
    //     0.5, -0.5, -0.5,  0.0, 1.0,
    //     0.5, -0.5, -0.5,  0.0, 1.0,
    //     0.5, -0.5,  0.5,  0.0, 0.0,
    //     0.5,  0.5,  0.5,  1.0, 0.0,
//
    //    -0.5, -0.5, -0.5,  0.0, 1.0,
    //     0.5, -0.5, -0.5,  1.0, 1.0,
    //     0.5, -0.5,  0.5,  1.0, 0.0,
    //     0.5, -0.5,  0.5,  1.0, 0.0,
    //    -0.5, -0.5,  0.5,  0.0, 0.0,
    //    -0.5, -0.5, -0.5,  0.0, 1.0,
//
    //    -0.5,  0.5, -0.5,  0.0, 1.0,
    //     0.5,  0.5, -0.5,  1.0, 1.0,
    //     0.5,  0.5,  0.5,  1.0, 0.0,
    //     0.5,  0.5,  0.5,  1.0, 0.0,
    //    -0.5,  0.5,  0.5,  0.0, 0.0,
    //    -0.5,  0.5, -0.5,  0.0, 1.0
//   ];
   let vertices: Vec<f32> = vec![
        0.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 1.0, 0.0,
        1.0, 1.0, 0.0, 1.0, 1.0,
        1.0, 1.0, 0.0, 1.0, 1.0,
        0.0, 1.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 0.0, 0.0,

        0.0, 0.0, 1.0, 0.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0,
        0.0, 1.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 0.0, 0.0,

        0.0, 1.0, 1.0, 1.0, 0.0,
        0.0, 1.0, 0.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 0.0, 0.0,
        0.0, 1.0, 1.0, 1.0, 0.0,

        1.0, 1.0, 1.0, 1.0, 0.0,
        1.0, 1.0, 0.0, 1.0, 1.0,
        1.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 1.0, 0.0, 0.0,
        1.0, 1.0, 1.0, 1.0, 0.0,

        0.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 1.0, 1.0,
        1.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 1.0,

        0.0, 1.0, 0.0, 0.0, 1.0,
        1.0, 1.0, 0.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 0.0,
        1.0, 1.0, 1.0, 1.0, 0.0,
        0.0, 1.0, 1.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0, 1.0
   ];

    //let vertices: Vec<f32> = vec![
    //     0.5,  0.5, 0.0, 1.0, 1.0,   // 0 front top right
    //     0.5, -0.5, 0.0, 1.0, 0.0,   // 1 front bottom right
    //    -0.5, -0.5, 0.0, 0.0, 0.0,   // 2 front bottom left
    //    -0.5,  0.5, 0.0, 0.0, 1.0,   // 3 front top left
    //     0.5,  0.5, 1.0, 1.0, 1.0,   // 4 back top right
    //     0.5, -0.5, 1.0, 1.0, 0.0,   // 5 back bottom right
    //    -0.5, -0.5, 1.0, 0.0, 0.0,   // 6 back bottom left
    //    -0.5,  0.5, 1.0, 0.0, 1.0    // 7 back top left
    //];
    //let indices = [
    //    0, 1, 3,    //
    //    1, 2, 3,    // bottom face
    //    4, 5, 0,    //
    //    5, 1, 0,    // left face
    //    7, 6, 4,    //
    //    6, 5, 4,    // top face
    //    7, 6, 3,    //
    //    6, 2, 3,    // right face
    //    4, 0, 7,    //
    //    0, 3, 7,    // top face
    //    2, 6, 1,    //
    //    6, 5, 1     // bottom face
    //];

    let cube_positions: Vec<Vector3<f32>> = create_chunk(16);
    //let cube_positions: Vec<Vector3<f32>> = vec![
    //    vec3(0.0, 0.0, -5.0),
    //    vec3(2.0, 5.0, -15.0),
    //    vec3(-1.5, -2.2, -2.5),
    //    vec3(-3.8, -2.0, -12.3),
    //    vec3(2.4, -0.4, -3.5),
    //    vec3(-1.7, 3.0, -7.5),
    //    vec3(1.3, -2.0, -2.5),
    //    vec3(1.5, 2.0, -2.5),
    //    vec3(1.5, 0.2, -1.5),
    //    vec3(-1.3, 1.0, -1.5)
    //];
    let cubes_to_render = has_six_adjacent_vector3s(&cube_positions);


    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;
    //let mut ebo: GLuint = 0;
    let mut texture: GLuint = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1,  &mut vbo);
        //gl::GenBuffers(1,  &mut ebo);
        gl::BindVertexArray(vao);
    }

    unsafe {
        gl::BindBuffer(ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
    }

    //unsafe {
    //    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    //    gl::BufferData(
    //        gl::ELEMENT_ARRAY_BUFFER,
    //        size_of_val(&indices) as GLsizeiptr,
    //        indices.as_ptr().cast(),
    //        gl::STATIC_DRAW
    //    )
    //}
    unsafe {
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()).try_into().unwrap(),
            std::ptr::null()
        );

        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            (5 * size_of::<f32>()).try_into().unwrap(),
            (3 * size_of::<f32>()) as *const c_void
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    let shader_program = Shader::new("triangle.vert", "triangle.frag");

    unsafe {
        gl::GenTextures(1, &mut texture);
    }

    unsafe  {
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    }

    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &data[0] as *const u8 as *const std::ffi::c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D)
    }


    unsafe {
        gl::BindVertexArray(vao);
    }

    let mut game_controller = GameController::init();

    while !window.window.should_close() {

        let current_frame = window.context.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_events(&window.reciever, &mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        process_input(&mut window, &delta_time, &mut keybindings, &mut camera);

        game_controller.run_loop(InputFunctionArguments::new().camera(&mut camera).cube_positions(&cube_positions));

        unsafe {
            gl_clear_color(255, 119, 110, 255);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = window.window.get_framebuffer_size();

        let model: Matrix4<f32> = Matrix4::from_axis_angle(
            vec3(0.5, 1.0, 0.0).normalize(),
            Rad(window.context.get_time() as f32)
        );
        let view = camera.get_view_matrix();
        let projection: Matrix4<f32> = perspective(Deg(camera.zoom), width as f32 / height as f32, 0.1, 100.0);

        unsafe {
            shader_program.set_mat4("model", &model);
            shader_program.set_mat4("view", &view);
            shader_program.set_mat4("projection", &projection);
            shader_program.set_vec4("ourColor", 1.0, 1.0, 1.0, 1.0);
        }
        unsafe {
            shader_program.use_program();
            gl::BindTexture(gl::TEXTURE_2D, texture);
            //gl::DrawArrays(TRIANGLES, 0, 3 as GLsizei);

            for (i, position) in cube_positions.iter().enumerate() {
                if cubes_to_render[i] {
                    let  model: Matrix4<f32> = Matrix4::from_translation(*position);
                //let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
                //let angle = 20.0 * i as f32;
                //model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));

                shader_program.set_mat4("model", &model);
                //gl::DrawElements(TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null())
                gl::DrawArrays(TRIANGLES, 0, (vertices.len() / 3) as i32)
                }
            }
        }

        window.swap_buffers();
        window.poll_events();
    }
}

fn process_events(events: &Receiver<(f64, glfw::WindowEvent)>, first_mouse: &mut bool, last_x: &mut f32, last_y: &mut f32, camera: &mut Camera) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            },
            glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                let (xpos, ypos) = (x_pos as f32, y_pos as f32);
                if *first_mouse {
                    *last_x = xpos;
                    *last_y = ypos;
                    *first_mouse = false;
                }

                let x_offset = xpos - *last_x;
                let y_offset = *last_y - ypos;

                *last_x = xpos;
                *last_y = ypos;
                camera.process_mouse_input(x_offset, y_offset, true)
            },
            glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                camera.process_scroll_input(y_offset as f32);
            },
            _ => {}
        }
    }
}

fn process_input(window: &mut Window, delta_time: &f32, bindings: &mut [KeyBinding], camera: &mut Camera) {
    for binding in bindings.iter_mut() {
        binding.update(binding.key, window.get_key(binding.key), InputFunctionArguments::new().camera(camera).window(window).delta_time(delta_time))
    }
}

