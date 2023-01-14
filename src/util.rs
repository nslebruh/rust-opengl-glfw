//?https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html


extern crate gl;
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use cgmath::{Vector3, Vector2};
use glfw::Key;
use gl::{types::*, VERTEX_SHADER, FRAGMENT_SHADER};




pub type Triangle = Vector3<Vector2<f32>>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct DigitalInputState {
    pub pressed: bool,
    pub released: bool
}

impl DigitalInputState {
    pub fn new() -> DigitalInputState {
        Self {
            pressed: false,
            released: false
        }
    }

    pub fn toggle(&mut self) {
        self.pressed = !self.pressed;
        self.released = !self.released;
        println!("{}", match self.pressed {
            true => "pressed",
            false => "released"
        })
    }
}

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
    key_states: HashMap<Key, DigitalInputState>
}

impl InputController {
    pub fn init() -> Self {
        let key_states: HashMap<Key, DigitalInputState> = HashMap::from([
            (Key::Space, DigitalInputState::new()),
            (Key::Apostrophe, DigitalInputState::new()),
            (Key::Comma, DigitalInputState::new()),
            (Key::Minus, DigitalInputState::new()),
            (Key::Period, DigitalInputState::new()),
            (Key::Slash, DigitalInputState::new()),
            (Key::Num0, DigitalInputState::new()),
            (Key::Num1, DigitalInputState::new()),
            (Key::Num2, DigitalInputState::new()),
            (Key::Num3, DigitalInputState::new()),
            (Key::Num4, DigitalInputState::new()),
            (Key::Num5, DigitalInputState::new()),
            (Key::Num6, DigitalInputState::new()),
            (Key::Num7, DigitalInputState::new()),
            (Key::Num8, DigitalInputState::new()),
            (Key::Num9, DigitalInputState::new()),
            (Key::Semicolon, DigitalInputState::new()),
            (Key::Equal, DigitalInputState::new()),
            (Key::A, DigitalInputState::new()),
            (Key::B, DigitalInputState::new()),
            (Key::C, DigitalInputState::new()),
            (Key::D, DigitalInputState::new()),
            (Key::E, DigitalInputState::new()),
            (Key::F, DigitalInputState::new()),
            (Key::G, DigitalInputState::new()),
            (Key::H, DigitalInputState::new()),
            (Key::I, DigitalInputState::new()),
            (Key::J, DigitalInputState::new()),
            (Key::K, DigitalInputState::new()),
            (Key::L, DigitalInputState::new()),
            (Key::M, DigitalInputState::new()),
            (Key::N, DigitalInputState::new()),
            (Key::O, DigitalInputState::new()),
            (Key::P, DigitalInputState::new()),
            (Key::Q, DigitalInputState::new()),
            (Key::R, DigitalInputState::new()),
            (Key::S, DigitalInputState::new()),
            (Key::T, DigitalInputState::new()),
            (Key::U, DigitalInputState::new()),
            (Key::V, DigitalInputState::new()),
            (Key::W, DigitalInputState::new()),
            (Key::X, DigitalInputState::new()),
            (Key::Y, DigitalInputState::new()),
            (Key::Z, DigitalInputState::new()),
            (Key::LeftBracket, DigitalInputState::new()),
            (Key::Backslash, DigitalInputState::new()),
            (Key::RightBracket, DigitalInputState::new()),
            (Key::GraveAccent, DigitalInputState::new()),
            (Key::World1, DigitalInputState::new()),
            (Key::World2, DigitalInputState::new()),
            (Key::Escape, DigitalInputState::new()),
            (Key::Enter, DigitalInputState::new()),
            (Key::Tab, DigitalInputState::new()),
            (Key::Backspace, DigitalInputState::new()),
            (Key::Insert, DigitalInputState::new()),
            (Key::Delete, DigitalInputState::new()),
            (Key::Right, DigitalInputState::new()),
            (Key::Left, DigitalInputState::new()),
            (Key::Down, DigitalInputState::new()),
            (Key::Up, DigitalInputState::new()),
            (Key::PageUp, DigitalInputState::new()),
            (Key::PageDown, DigitalInputState::new()),
            (Key::Home, DigitalInputState::new()),
            (Key::End, DigitalInputState::new()),
            (Key::CapsLock, DigitalInputState::new()),
            (Key::ScrollLock, DigitalInputState::new()),
            (Key::NumLock, DigitalInputState::new()),
            (Key::PrintScreen, DigitalInputState::new()),
            (Key::Pause, DigitalInputState::new()),
            (Key::F1, DigitalInputState::new()),
            (Key::F2, DigitalInputState::new()),
            (Key::F3, DigitalInputState::new()),
            (Key::F4, DigitalInputState::new()),
            (Key::F5, DigitalInputState::new()),
            (Key::F6, DigitalInputState::new()),
            (Key::F7, DigitalInputState::new()),
            (Key::F8, DigitalInputState::new()),
            (Key::F9, DigitalInputState::new()),
            (Key::F10, DigitalInputState::new()),
            (Key::F11, DigitalInputState::new()),
            (Key::F12, DigitalInputState::new()),
            (Key::F13, DigitalInputState::new()),
            (Key::F14, DigitalInputState::new()),
            (Key::F15, DigitalInputState::new()),
            (Key::F16, DigitalInputState::new()),
            (Key::F17, DigitalInputState::new()),
            (Key::F18, DigitalInputState::new()),
            (Key::F19, DigitalInputState::new()),
            (Key::F20, DigitalInputState::new()),
            (Key::F21, DigitalInputState::new()),
            (Key::F22, DigitalInputState::new()),
            (Key::F23, DigitalInputState::new()),
            (Key::F24, DigitalInputState::new()),
            (Key::F25, DigitalInputState::new()),
            (Key::Kp0, DigitalInputState::new()),
            (Key::Kp1, DigitalInputState::new()),
            (Key::Kp2, DigitalInputState::new()),
            (Key::Kp3, DigitalInputState::new()),
            (Key::Kp4, DigitalInputState::new()),
            (Key::Kp5, DigitalInputState::new()),
            (Key::Kp6, DigitalInputState::new()),
            (Key::Kp7, DigitalInputState::new()),
            (Key::Kp8, DigitalInputState::new()),
            (Key::Kp9, DigitalInputState::new()),
            (Key::KpDecimal, DigitalInputState::new()),
            (Key::KpDivide, DigitalInputState::new()),
            (Key::KpMultiply, DigitalInputState::new()),
            (Key::KpSubtract, DigitalInputState::new()),
            (Key::KpAdd, DigitalInputState::new()),
            (Key::KpEnter, DigitalInputState::new()),
            (Key::KpEqual, DigitalInputState::new()),
            (Key::LeftShift, DigitalInputState::new()),
            (Key::LeftControl, DigitalInputState::new()),
            (Key::LeftAlt, DigitalInputState::new()),
            (Key::LeftSuper, DigitalInputState::new()),
            (Key::RightShift, DigitalInputState::new()),
            (Key::RightControl, DigitalInputState::new()),
            (Key::RightAlt, DigitalInputState::new()),
            (Key::RightSuper, DigitalInputState::new()),
            (Key::Menu, DigitalInputState::new()),
            (Key::Unknown, DigitalInputState::new()),            
        ]);

        Self {
            key_states
        }
    }

    pub fn toggle_key_state(&self, key: &Key) {
        let mut dis = self.key_states[key];
        dis.toggle()
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

