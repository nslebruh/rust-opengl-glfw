use std::sync::mpsc::Receiver;
use thiserror::Error;

use glfw::{WindowEvent, InitError, Context};

#[derive(Debug, Error)]
pub enum WindowInitError {
    #[error("Failed to initialise glfw")]
    InitError(InitError),
    #[error("Failed to create glfw window")]
    CreationError(String)
}

#[allow(dead_code)]
pub struct Window {
    pub context: glfw::Glfw,
    pub window: glfw::Window,
    pub reciever: Receiver<(f64, WindowEvent)>,
    pub prev_height: u32,
    pub prev_width: u32,
}

impl Window {
    pub fn init(width: u32, height: u32, title: &str, mode: glfw::WindowMode, hints: Vec<glfw::WindowHint>) -> Result<Self, WindowInitError> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS);
        match glfw {
            Ok(mut gres) => {
                for hint in hints {
                    gres.window_hint(hint)
                }
                let window = gres.create_window(width, height, title, mode);
                match window {
                    Some(res) => {
                        Ok(Self {
                            context: gres,
                            window: res.0,
                            reciever: res.1,
                            prev_height: height,
                            prev_width: width
                        })
                    },
                    None => {
                        Err(WindowInitError::CreationError(String::from("Unable to create window")))
                    }
                }
            },
            Err(err) => Err(WindowInitError::InitError(err)),
        }
    }

    pub fn poll_events(&mut self) {
        self.context.poll_events()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    #[allow(dead_code)]
    pub fn set_should_close(&mut self, value: bool) {
        self.window.set_should_close(value)
    }

    pub fn make_current(&mut self) {
        self.window.make_current()
    }

    #[allow(dead_code)]
    pub fn toggle_fullscreen(&mut self) {

    }

    pub fn get_key(&mut self, key: glfw::Key) -> glfw::Action {
        self.window.get_key(key)
    }

    pub fn set_cursor_mode(&mut self, mode: glfw::CursorMode) {
        self.window.set_cursor_mode(mode)
    }

    pub fn get_framebuffer_size(&mut self) -> (i32, i32) {
        self.window.get_framebuffer_size()
    }

    pub fn set_framebuffer_size_polling(&mut self, val: bool) {
        self.window.set_framebuffer_size_polling(val)
    }
    
    pub fn set_cursor_pos_polling(&mut self, val: bool) {
        self.window.set_cursor_pos_polling(val)
    }
    pub fn set_scroll_polling(&mut self, val: bool) {
        self.window.set_scroll_polling(val)
    }

    pub fn get_proc_address(&mut self, procname: &str) -> glfw::GLProc  {
        self.window.get_proc_address(procname)
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

}