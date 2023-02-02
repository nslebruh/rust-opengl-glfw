use std::sync::mpsc::Receiver;

use glfw::{WindowEvent, InitError, Context};

#[derive(Debug)]
pub enum WindowInitError {
    InitError(InitError),
    CreationError(String)
}

#[allow(dead_code)]
pub struct Window {
    pub context: glfw::Glfw,
    pub window: glfw::Window,
    pub reciever: Receiver<(f64, WindowEvent)>,
    prev_height: u32,
    prev_width: u32,
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

}