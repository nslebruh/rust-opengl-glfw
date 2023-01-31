use glfw::{Key, Action};

use std::ops::Fn;

use crate::input_controller::InputFunctionArguments;

pub struct KeyBinding<F: Fn(InputFunctionArguments)> {
    pub key: Key,
    pub state: bool,
    pub callback: F,
    run_every_frame: bool
}

impl<F: Fn(InputFunctionArguments)> KeyBinding<F> {
    pub fn new(key: Key, run_every_frame: bool, callback: F) -> KeyBinding<F> {
        KeyBinding {
            key,
            state: false,
            callback,
            run_every_frame
        }
    }

    pub fn update(&mut self, key: Key, action: Action, args: InputFunctionArguments) {
        if self.key == key {
            if self.run_every_frame {
                match action {
                    Action::Press  => {
                        (self.callback)(args);
                    },
                    _ => {}
                }
            } else {
                match action {
                    Action::Press if self.state != true => {
                        self.state = true;
                        (self.callback)(args);
                    },
                    Action::Release => {
                        self.state = false;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct KeyBindingConfig {
    config: Vec<KeyBinding<Box<dyn Fn(InputFunctionArguments)>>>
}

impl KeyBindingConfig {
    pub fn new_empty() -> Self {
        Self {
            config: vec![]
        }
    }

    pub fn from_file() -> Self {
        Self {
            config: vec![]
        }
    }

    pub fn from_vec() -> Self {
        Self {
            config: vec![]
        }
    }
}