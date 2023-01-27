extern crate gl;
extern crate glfw;

use std::{collections::HashMap, ops::DerefMut};

use glfw::{Key, MouseButton, Window, Action};
use std::ops::Deref;

use crate::{camera::Camera, input_functions::*};




type InputFunctionType = fn(InputFunctionArguments) -> ();

pub struct InputState(pub HashMap<Key, Action>);

impl Deref for InputState {
    type Target = HashMap<Key, Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InputState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct InputFunction(pub InputFunctionType);

pub struct InputFunctionArguments<'a> {
    pub window: Option<&'a mut Window>,
    pub camera: Option<&'a mut Camera>,
    pub delta_time: Option<&'a f32>,
    pub input_state: Option<&'a HashMap<Key, Action>>,
    pub key: Option<&'a Key>,
    pub action: Option<&'a Action>,
    pub _glfw: Option<&'a glfw::Glfw>,
    pub input_controller: Option<&'a mut InputController>
}

impl<'a> InputFunctionArguments<'a> {
    pub fn new() -> Self {
        Self {
            window: None,
            camera: None,
            delta_time: None,
            input_state: None,
            key: None,
            action: None,
            _glfw: None,
            input_controller: None
        }
    }

    pub fn window(self, window: &'a mut Window) -> Self {
        Self {
            window: Some(window),
            ..self
        }
    }

    pub fn camera(self, camera: &'a mut Camera) -> Self {
        Self {
            camera: Some(camera),
            ..self
        }
    }
    
    pub fn delta_time(self, delta_time: &'a f32) -> Self {
        Self {
            delta_time: Some(delta_time),
            ..self
        }
    }

    pub fn input_state(self, input_state: &'a HashMap<Key, Action>) -> Self {
        Self {
            input_state: Some(input_state),
            ..self
        }
    }

    pub fn key(self, key: &'a Key) -> Self {
        Self {
            key: Some(key),
            ..self
        }
    }

    pub fn action(self, action: &'a Action) -> Self {
        Self {
            action: Some(action),
            ..self
        }
    }

    pub fn _glfw(self, _glfw: &'a glfw::Glfw) -> Self {
        Self {
            _glfw: Some(_glfw),
            ..self
        }
    }

    pub fn input_controller(self, input_controller: &'a mut InputController) -> Self {
        Self {
            input_controller: Some(input_controller),
            ..self
        }
    }
}


pub struct InputController {
    pub keybinds: HashMap<Key, InputFunction>,
    pub mouse_keybinds: HashMap<MouseButton, InputFunction>,
    pub input_state: HashMap<Key, Action>,
    pub window_size: (i32, i32)
}

impl InputController {
    pub fn init(keybinds: Option<HashMap<Key, InputFunction>>, mouse_keybinds: Option<HashMap<MouseButton, InputFunction>>) -> Self {

        let keybinds = match keybinds {
            Some(x) => x,
            None => InputController::create_keybinds_hashmap()
        };

        let mouse_keybinds = match mouse_keybinds {
            Some(x) => x,
            None => InputController::create_mouse_keybinds_hashmap()
        };
        let is: HashMap<Key, Action> = keybinds.iter().map(|(k, _v)| ((*k, Action::Release))).collect();
        let input_state: HashMap<Key, Action> = HashMap::from(is);

        Self {
            keybinds,
            mouse_keybinds,
            input_state,
            window_size: (1820, 720)
        }
    }

    pub fn create_keybinds_hashmap() -> HashMap<Key, InputFunction> {
        HashMap::from([
            (
                Key::Escape,
                InputFunction(set_window_should_close)            
            ),
            (
                Key::W,
                InputFunction(camera_forward)
            ),
            (
                Key::A,
                InputFunction(camera_left)
            ),
            (
                Key::S,
                InputFunction(camera_backward)
            ),
            (
                Key::D,
                InputFunction(camera_right)
            ),
        ])
    }

    pub fn create_mouse_keybinds_hashmap() -> HashMap<MouseButton, InputFunction> {
        HashMap::from([
            (
                MouseButton::Button1,
                InputFunction(|_args| println!("test"))
            )
        ])
    }
}
