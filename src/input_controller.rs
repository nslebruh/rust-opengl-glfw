use std::collections::HashMap;

use glfw::{Action, Key, MouseButton};


#[derive(Debug, Clone)]
pub struct InputController {
    //keybinds: HashMap<Key, Keybind>,
    key_states: HashMap<Key, Action>,
    mouse_states: HashMap<MouseButton, Action>
}

impl InputController {
    pub fn init() -> Self {
        let key_states: HashMap<Key, Action> = HashMap::from([
            (Key::Space, Action::Release),
            (Key::Apostrophe, Action::Release),
            (Key::Comma, Action::Release),
            (Key::Minus, Action::Release),
            (Key::Period, Action::Release),
            (Key::Slash, Action::Release),
            (Key::Num0, Action::Release),
            (Key::Num1, Action::Release),
            (Key::Num2, Action::Release),
            (Key::Num3, Action::Release),
            (Key::Num4, Action::Release),
            (Key::Num5, Action::Release),
            (Key::Num6, Action::Release),
            (Key::Num7, Action::Release),
            (Key::Num8, Action::Release),
            (Key::Num9, Action::Release),
            (Key::Semicolon, Action::Release),
            (Key::Equal, Action::Release),
            (Key::A, Action::Release),
            (Key::B, Action::Release),
            (Key::C, Action::Release),
            (Key::D, Action::Release),
            (Key::E, Action::Release),
            (Key::F, Action::Release),
            (Key::G, Action::Release),
            (Key::H, Action::Release),
            (Key::I, Action::Release),
            (Key::J, Action::Release),
            (Key::K, Action::Release),
            (Key::L, Action::Release),
            (Key::M, Action::Release),
            (Key::N, Action::Release),
            (Key::O, Action::Release),
            (Key::P, Action::Release),
            (Key::Q, Action::Release),
            (Key::R, Action::Release),
            (Key::S, Action::Release),
            (Key::T, Action::Release),
            (Key::U, Action::Release),
            (Key::V, Action::Release),
            (Key::W, Action::Release),
            (Key::X, Action::Release),
            (Key::Y, Action::Release),
            (Key::Z, Action::Release),
            (Key::LeftBracket, Action::Release),
            (Key::Backslash, Action::Release),
            (Key::RightBracket, Action::Release),
            (Key::GraveAccent, Action::Release),
            (Key::World1, Action::Release),
            (Key::World2, Action::Release),
            (Key::Escape, Action::Release),
            (Key::Enter, Action::Release),
            (Key::Tab, Action::Release),
            (Key::Backspace, Action::Release),
            (Key::Insert, Action::Release),
            (Key::Delete, Action::Release),
            (Key::Right, Action::Release),
            (Key::Left, Action::Release),
            (Key::Down, Action::Release),
            (Key::Up, Action::Release),
            (Key::PageUp, Action::Release),
            (Key::PageDown, Action::Release),
            (Key::Home, Action::Release),
            (Key::End, Action::Release),
            (Key::CapsLock, Action::Release),
            (Key::ScrollLock, Action::Release),
            (Key::NumLock, Action::Release),
            (Key::PrintScreen, Action::Release),
            (Key::Pause, Action::Release),
            (Key::F1, Action::Release),
            (Key::F2, Action::Release),
            (Key::F3, Action::Release),
            (Key::F4, Action::Release),
            (Key::F5, Action::Release),
            (Key::F6, Action::Release),
            (Key::F7, Action::Release),
            (Key::F8, Action::Release),
            (Key::F9, Action::Release),
            (Key::F10, Action::Release),
            (Key::F11, Action::Release),
            (Key::F12, Action::Release),
            (Key::F13, Action::Release),
            (Key::F14, Action::Release),
            (Key::F15, Action::Release),
            (Key::F16, Action::Release),
            (Key::F17, Action::Release),
            (Key::F18, Action::Release),
            (Key::F19, Action::Release),
            (Key::F20, Action::Release),
            (Key::F21, Action::Release),
            (Key::F22, Action::Release),
            (Key::F23, Action::Release),
            (Key::F24, Action::Release),
            (Key::F25, Action::Release),
            (Key::Kp0, Action::Release),
            (Key::Kp1, Action::Release),
            (Key::Kp2, Action::Release),
            (Key::Kp3, Action::Release),
            (Key::Kp4, Action::Release),
            (Key::Kp5, Action::Release),
            (Key::Kp6, Action::Release),
            (Key::Kp7, Action::Release),
            (Key::Kp8, Action::Release),
            (Key::Kp9, Action::Release),
            (Key::KpDecimal, Action::Release),
            (Key::KpDivide, Action::Release),
            (Key::KpMultiply, Action::Release),
            (Key::KpSubtract, Action::Release),
            (Key::KpAdd, Action::Release),
            (Key::KpEnter, Action::Release),
            (Key::KpEqual, Action::Release),
            (Key::LeftShift, Action::Release),
            (Key::LeftControl, Action::Release),
            (Key::LeftAlt, Action::Release),
            (Key::LeftSuper, Action::Release),
            (Key::RightShift, Action::Release),
            (Key::RightControl, Action::Release),
            (Key::RightAlt, Action::Release),
            (Key::RightSuper, Action::Release),
            (Key::Menu, Action::Release),
            (Key::Unknown, Action::Release),            
        ]);
        let mouse_states = HashMap::from([
            (MouseButton::Button1, Action::Release),
            (MouseButton::Button2, Action::Release),
            (MouseButton::Button3, Action::Release),
            (MouseButton::Button4, Action::Release),
            (MouseButton::Button5, Action::Release),
            (MouseButton::Button6, Action::Release),
            (MouseButton::Button7, Action::Release),
            (MouseButton::Button8, Action::Release)
        ]);

        Self {
            key_states,
            mouse_states
        }
    }

    pub fn set_key_state(&mut self, key: &Key, action: &Action) {
        let hashed_action = self.key_states.get_mut(key).unwrap();
        *hashed_action = *action;
        println!("key: {:?}, action: {:?}", key, action);
        
    }

    pub fn set_mouse_state(&mut self, mouse: &MouseButton, action: &Action) {
        let hashed_action = self.mouse_states.get_mut(mouse).unwrap();
        *hashed_action = *action;
        println!("key: {:?}, action: {:?}", mouse, action);
    }
}

