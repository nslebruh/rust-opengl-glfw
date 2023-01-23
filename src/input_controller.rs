extern crate gl;
extern crate glfw;

use std::collections::HashMap;

use glfw::{Key, MouseButton, Window};

use crate::camera::Camera;
use crate::input_functions::INPUT_FUNCTIONS_VECTOR;

#[derive(Debug)]
pub struct InputFunctionArguments<'a> {
    pub window: Option<&'a mut Window>,
    pub camera: Option<&'a mut Camera>,
    pub delta_time: Option<&'a f32>
}

impl<'a> InputFunctionArguments<'a> {
    pub fn new() -> Self {
        Self {
            window: None,
            camera: None,
            delta_time: None
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
}

#[derive(Clone, Copy)]
pub struct InputFunction {
    pub name: &'static str,
    pub function: fn(args: InputFunctionArguments),

}

impl InputFunction {
    pub fn new(name: &'static str, function: fn(args: InputFunctionArguments)) -> Self {
        Self {
            name,
            function
        }
    }
    pub fn run(&self, args: InputFunctionArguments) {
        (self.function)(args)
    }
}

pub struct InputController {
    pub keybinds: HashMap<Key, InputFunction>,
    pub mouse_keybinds: HashMap<MouseButton, InputFunction>,
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
        
        Self {
            keybinds,
            mouse_keybinds,
        }
    }

    pub fn create_keybinds_hashmap() -> HashMap<Key, InputFunction> {
        HashMap::from([
            (
                Key::Escape,
                *INPUT_FUNCTIONS_VECTOR.get(0).unwrap()
            ),
            (
                Key::W,
                *INPUT_FUNCTIONS_VECTOR.get(2).unwrap()
            ),
            (
                Key::A,
                *INPUT_FUNCTIONS_VECTOR.get(4).unwrap()
            ),
            (
                Key::S,
                *INPUT_FUNCTIONS_VECTOR.get(3).unwrap()
            ),
            (
                Key::D,
                *INPUT_FUNCTIONS_VECTOR.get(5).unwrap()
            ),
        ])
    }

    pub fn create_mouse_keybinds_hashmap() -> HashMap<MouseButton, InputFunction> {
        HashMap::from([
            (
                MouseButton::Button1,
                InputFunction::new("Test2", |_| println!("Test2")))])
    }
}