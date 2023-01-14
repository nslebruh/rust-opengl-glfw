//?https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html


extern crate gl;
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use cgmath::{Vector3, Vector2};
use glfw::{Key, MouseButton, Action};
use gl::{types::*, VERTEX_SHADER, FRAGMENT_SHADER};




pub type Triangle = Vector3<Vector2<f32>>;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Keybind {
    key: Key,
    function: InputFunction
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct InputFunction {
    name: String,
    function: Option<fn()> 
}

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


trait Apply<Args> {
    type Output;
    fn apply(&self, args: Args) -> Self::Output;
}

macro_rules! impl_apply {
    // Empty case
    () => {};
    ($first_generic:ident $($other_generics:ident)*) => {
        impl_apply!($($other_generics)*);

        impl<$first_generic, $($other_generics,)* Ret, Func>
            Apply<($first_generic, $($other_generics,)*)>
            for Func
        where
            Func: Fn($first_generic, $($other_generics,)*) -> Ret,
        {
            type Output = Ret;
            #[allow(non_snake_case)]
            fn apply(
                &self,
                ($first_generic, $($other_generics,)*): ($first_generic, $($other_generics,)*),
            ) -> Self::Output {
                self($first_generic, $($other_generics,)*)
            }
        }
    };
}
impl<Ret, Func> Apply<()> for Func
where
    Func: Fn() -> Ret,
{
    type Output = Ret;
    fn apply(&self, (): ()) -> Self::Output {
        self()
    }
}

impl_apply!(A B C D E F G H I J K L M);


pub fn rgba_from_u8(r: u8, g: u8, b: u8, a: u8) -> Result<(f32, f32, f32, f32), ()> {
    Ok((f32::from(r) / 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0, f32::from(a) / 255.0))
}

pub unsafe fn gl_clear_color(r: u8, g: u8, b: u8, a: u8) {
    (
        |
            r,
            g,
            b,
            a
        | {gl::ClearColor(r, g, b, a)}
    ).apply(rgba_from_u8(r, g, b, a).unwrap());
}


pub fn vert_shader_from_source(source: &CStr) -> Result<GLuint, String> {
    shader_from_source(source, VERTEX_SHADER)
}

pub fn frag_shader_from_source(source: &CStr) -> Result<GLuint, String> {
    shader_from_source(source, FRAGMENT_SHADER)
}


fn shader_from_source(source: &CStr, type_: GLenum) -> Result<GLuint, String> {
    let id = unsafe { gl::CreateShader(type_) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id)
    };

    let mut success: gl::types::GLint = 1;

    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        
        buffer.extend([b' '].iter().cycle().take(len as usize));
        
        let error = create_whitespace_cstring_with_len(len as usize);

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(error.to_string_lossy().into_owned());
    }
    Ok(id)
}


fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

