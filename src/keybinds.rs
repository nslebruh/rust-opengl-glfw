use glfw::{Key, Action};

use std::ops::Fn;

use crate::input_controller::InputFunctionArguments;

pub struct KeyBinding<F: Fn(InputFunctionArguments)> {
    key: Key,
    state: bool,
    callback: F,
    run_every_frame: bool
}

impl<F: Fn(InputFunctionArguments)> KeyBinding<F> {
    pub fn new(key: Key, run_every_frame: bool, callback: F) -> KeyBinding<F> {
        KeyBinding {
            key: key,
            state: false,
            callback: callback,
            run_every_frame
        }
    }

    pub fn update(&mut self, key: Key, action: Action, args: InputFunctionArguments) {
        if self.key == key {
            match action {
                Action::Press => {
                    self.state = true;
                    (self.callback)(args);
                },
                Action::Release => {
                    self.state = false;
                },
                _ => {}
            }
        }
    }
}
