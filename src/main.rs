mod game;
mod engine;

extern crate nalgebra_glm as glm;
extern crate glfw;
extern crate gl;
extern crate lazy_static;
extern crate image;

use crate::engine::generation::*;
use std::{mem::size_of, path::Path, ffi::c_void};
use cgmath::{Matrix4, vec3, Rad, perspective, Deg, InnerSpace, Point3};

use engine::shader::Shader;
use engine::util::*;
use game::chunk::Chunk;
use gl::{types::*, ARRAY_BUFFER, TRIANGLES};
use glfw::Key;

use engine::{
    keybinds::*,
    input_functions::*,
    camera::Camera,
    window::Window
};
use glm::vec3 as glm_vec3;
use noise::{Perlin, Fbm, Seedable};

fn main() {
    let scr_width: u32 = 1280;
    let scr_height: u32 = 720;

    let img = image::open(&Path::new("dirt.png")).unwrap().to_rgba8();
    let data = img.to_vec();

    let mut camera = Camera {
        position: Point3::new(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut first_mouse = true;
    let mut last_x: f32 = scr_width as f32 / 2.0;
    let mut last_y: f32 = scr_height as f32 / 2.0;

    let mut delta_time: f32;
    let mut last_frame: f32 = 0.0;

    let mut window: Window = Window::init(
        scr_width,
        scr_height,
        "test title",
        glfw::WindowMode::Windowed,
        vec![
            glfw::WindowHint::ContextVersion(3, 3),
            glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core)
            ]
    ).unwrap(); 

    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        //gl::Enable(gl::CULL_FACE);

    }

    window.make_current();
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);


    let mut keybindings: Vec<KeyBinding> = vec![
        KeyBinding::new(Key::Escape, false, set_window_should_close),
        KeyBinding::new(Key::W, true, camera_forward),
        KeyBinding::new(Key::A, true, camera_left),
        KeyBinding::new(Key::S, true, camera_backward),
        KeyBinding::new(Key::D, true, camera_right),
        KeyBinding::new(Key::Space, true, camera_up),
        KeyBinding::new(Key::LeftShift, true, camera_down),
        KeyBinding::new(Key::RightShift, false, toggle_cursor_mode),
        KeyBinding::new(Key::RightControl, false, print_camera_pos),
        KeyBinding::new(Key::LeftControl, false, increase_movement_speed),
        KeyBinding::new(Key::F11, false, toggle_fullscreen)
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

    let world = World::new(4, 1);
    let noise_fn = Fbm::<Perlin>::default().set_seed(1 as u32);

    let test_chunk = Chunk::gen(glm_vec3(0, 0, 0), &noise_fn);

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

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
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
        //gl::BindVertexArray(vao);
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    while !window.should_close() {

        let current_frame = window.context.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        window.process_events(&mut first_mouse, &mut last_x, &mut last_y, &mut camera);

        process_input(&mut window, &delta_time, &mut keybindings, &mut camera);


        unsafe {
            gl_clear_color(128, 128, 128, 255);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let (width, height) = window.get_framebuffer_size();

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
            shader_program.set_vec4("ourColor", 1.0, 1.0, 1.0, 0.0);
        }
        unsafe {
            shader_program.use_program();
            //gl::DrawArrays(TRIANGLES, 0, 3 as GLsizei);
            test_chunk.mesh.draw();
            //for chunk_vec in &world.chunks {
            //    for chunk in chunk_vec {
            //        for (pos, block) in chunk.blocks.iter() {
            //            if block.1 {
            //                let model: Matrix4<f32> = Matrix4::from_translation(block_pos_to_f32(pos + (chunk.position * 16)));
            //                shader_program.set_mat4("model", &model);
            //                gl::DrawArrays(TRIANGLES, 0, (vertices.len() / 3) as i32)
            //            }
            //        }
            //    }
            //}
        }

        window.swap_buffers();
        window.poll_events();
    }
}

//fn process_events(events: &Receiver<(f64, WindowEvent)>, window: &mut Window, first_mouse: &mut bool, last_x: &mut f32, last_y: &mut f32, camera: &mut Camera) {
//    for (_, event) in glfw::flush_messages(events) {
//        match event {
//            glfw::WindowEvent::FramebufferSize(width, height) => {
//                unsafe { gl::Viewport(0, 0, width, height) }
//            },
//            glfw::WindowEvent::Pos(xpos, ypos) => {
//                window.set_last_pos(xpos, ypos);
//            },
//            glfw::WindowEvent::Size(width, height) => {
//                window.set_last_size(width, height);
//            },
//            glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
//                let (xpos, ypos) = (x_pos as f32, y_pos as f32);
//                if *first_mouse {
//                    *last_x = xpos;
//                    *last_y = ypos;
//                    *first_mouse = false;
//                }
//
//                let x_offset = xpos - *last_x;
//                let y_offset = *last_y - ypos;
//
//                *last_x = xpos;
//                *last_y = ypos;
//                camera.process_mouse_input(x_offset, y_offset, true)
//            },
//            glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
//                camera.process_scroll_input(y_offset as f32);
//            },
//            _ => {}
//        }
//    }
//}

fn process_input(window: &mut Window, delta_time: &f32, bindings: &mut [KeyBinding], camera: &mut Camera) {
    for binding in bindings.iter_mut() {
        let action = window.get_key(binding.key);
        binding.update(action, InputFunctionArguments::new().camera(camera).window(window).delta_time(delta_time).action(&action))
    }
}

fn multiply_the_values(lhs: &cgmath::Vector3<i32>, rhs: &cgmath::Vector3<i32>) -> cgmath::Vector3<i32>{
    vec3(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z)
}