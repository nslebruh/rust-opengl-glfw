//?https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html


extern crate gl;
use std::str::FromStr;
use strum_macros::{EnumIter, EnumString};
use strum::IntoEnumIterator;
use std::ffi::{CString, CStr};
use cgmath::{Vector3, Vector2};
use glfw::Key;

use gl::{types::*, VERTEX_SHADER, FRAGMENT_SHADER};

pub type Triangle = Vector3<Vector2<f32>>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct DigitalInputState {
    pub key: BastardKey,
    pub pressed: bool,
    pub released: bool
}

impl DigitalInputState {
    pub fn new(key: BastardKey) -> Self {
        Self {
            key,
            pressed: false,
            released: false,
        }
    }
    pub fn toggle(&mut self) {
        self.pressed = !self.pressed;
        self.released = !self.released;
        println!("{:?} {}", self.key, match self.pressed {
            true => "pressed",
            false => "released"
        })
    }
}


#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Keybind {
    key: BastardKey,
    function: InputFunction
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct InputFunction {
    name: String,
    function: Option<fn()> 
}

pub struct InputController {
    //keybinds: Vec<Keybind>,
    key_states: Vec<DigitalInputState>
}

impl InputController {
    pub fn init() -> Self {
        let key_states: Vec<DigitalInputState> = BastardKey::iter().map(|bk: BastardKey| DigitalInputState::new(bk)).collect();
        Self {
            key_states
        }
    }

    pub fn get_key_state(&self, key: BastardKey) -> DigitalInputState {
        self.key_states
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

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, EnumIter, EnumString)]
pub enum BastardKey {
    Space = glfw::ffi::KEY_SPACE,
    Apostrophe = glfw::ffi::KEY_APOSTROPHE,
    Comma = glfw::ffi::KEY_COMMA,
    Minus = glfw::ffi::KEY_MINUS,
    Period = glfw::ffi::KEY_PERIOD,
    Slash = glfw::ffi::KEY_SLASH,
    Num0 = glfw::ffi::KEY_0,
    Num1 = glfw::ffi::KEY_1,
    Num2 = glfw::ffi::KEY_2,
    Num3 = glfw::ffi::KEY_3,
    Num4 = glfw::ffi::KEY_4,
    Num5 = glfw::ffi::KEY_5,
    Num6 = glfw::ffi::KEY_6,
    Num7 = glfw::ffi::KEY_7,
    Num8 = glfw::ffi::KEY_8,
    Num9 = glfw::ffi::KEY_9,
    Semicolon = glfw::ffi::KEY_SEMICOLON,
    Equal = glfw::ffi::KEY_EQUAL,
    A = glfw::ffi::KEY_A,
    B = glfw::ffi::KEY_B,
    C = glfw::ffi::KEY_C,
    D = glfw::ffi::KEY_D,
    E = glfw::ffi::KEY_E,
    F = glfw::ffi::KEY_F,
    G = glfw::ffi::KEY_G,
    H = glfw::ffi::KEY_H,
    I = glfw::ffi::KEY_I,
    J = glfw::ffi::KEY_J,
    K = glfw::ffi::KEY_K,
    L = glfw::ffi::KEY_L,
    M = glfw::ffi::KEY_M,
    N = glfw::ffi::KEY_N,
    O = glfw::ffi::KEY_O,
    P = glfw::ffi::KEY_P,
    Q = glfw::ffi::KEY_Q,
    R = glfw::ffi::KEY_R,
    S = glfw::ffi::KEY_S,
    T = glfw::ffi::KEY_T,
    U = glfw::ffi::KEY_U,
    V = glfw::ffi::KEY_V,
    W = glfw::ffi::KEY_W,
    X = glfw::ffi::KEY_X,
    Y = glfw::ffi::KEY_Y,
    Z = glfw::ffi::KEY_Z,
    LeftBracket = glfw::ffi::KEY_LEFT_BRACKET,
    Backslash = glfw::ffi::KEY_BACKSLASH,
    RightBracket = glfw::ffi::KEY_RIGHT_BRACKET,
    GraveAccent = glfw::ffi::KEY_GRAVE_ACCENT,
    World1 = glfw::ffi::KEY_WORLD_1,
    World2 = glfw::ffi::KEY_WORLD_2,

    Escape = glfw::ffi::KEY_ESCAPE,
    Enter = glfw::ffi::KEY_ENTER,
    Tab = glfw::ffi::KEY_TAB,
    Backspace = glfw::ffi::KEY_BACKSPACE,
    Insert = glfw::ffi::KEY_INSERT,
    Delete = glfw::ffi::KEY_DELETE,
    Right = glfw::ffi::KEY_RIGHT,
    Left = glfw::ffi::KEY_LEFT,
    Down = glfw::ffi::KEY_DOWN,
    Up = glfw::ffi::KEY_UP,
    PageUp = glfw::ffi::KEY_PAGE_UP,
    PageDown = glfw::ffi::KEY_PAGE_DOWN,
    Home = glfw::ffi::KEY_HOME,
    End = glfw::ffi::KEY_END,
    CapsLock = glfw::ffi::KEY_CAPS_LOCK,
    ScrollLock = glfw::ffi::KEY_SCROLL_LOCK,
    NumLock = glfw::ffi::KEY_NUM_LOCK,
    PrintScreen = glfw::ffi::KEY_PRINT_SCREEN,
    Pause = glfw::ffi::KEY_PAUSE,
    F1 = glfw::ffi::KEY_F1,
    F2 = glfw::ffi::KEY_F2,
    F3 = glfw::ffi::KEY_F3,
    F4 = glfw::ffi::KEY_F4,
    F5 = glfw::ffi::KEY_F5,
    F6 = glfw::ffi::KEY_F6,
    F7 = glfw::ffi::KEY_F7,
    F8 = glfw::ffi::KEY_F8,
    F9 = glfw::ffi::KEY_F9,
    F10 = glfw::ffi::KEY_F10,
    F11 = glfw::ffi::KEY_F11,
    F12 = glfw::ffi::KEY_F12,
    F13 = glfw::ffi::KEY_F13,
    F14 = glfw::ffi::KEY_F14,
    F15 = glfw::ffi::KEY_F15,
    F16 = glfw::ffi::KEY_F16,
    F17 = glfw::ffi::KEY_F17,
    F18 = glfw::ffi::KEY_F18,
    F19 = glfw::ffi::KEY_F19,
    F20 = glfw::ffi::KEY_F20,
    F21 = glfw::ffi::KEY_F21,
    F22 = glfw::ffi::KEY_F22,
    F23 = glfw::ffi::KEY_F23,
    F24 = glfw::ffi::KEY_F24,
    F25 = glfw::ffi::KEY_F25,
    Kp0 = glfw::ffi::KEY_KP_0,
    Kp1 = glfw::ffi::KEY_KP_1,
    Kp2 = glfw::ffi::KEY_KP_2,
    Kp3 = glfw::ffi::KEY_KP_3,
    Kp4 = glfw::ffi::KEY_KP_4,
    Kp5 = glfw::ffi::KEY_KP_5,
    Kp6 = glfw::ffi::KEY_KP_6,
    Kp7 = glfw::ffi::KEY_KP_7,
    Kp8 = glfw::ffi::KEY_KP_8,
    Kp9 = glfw::ffi::KEY_KP_9,
    KpDecimal = glfw::ffi::KEY_KP_DECIMAL,
    KpDivide = glfw::ffi::KEY_KP_DIVIDE,
    KpMultiply = glfw::ffi::KEY_KP_MULTIPLY,
    KpSubtract = glfw::ffi::KEY_KP_SUBTRACT,
    KpAdd = glfw::ffi::KEY_KP_ADD,
    KpEnter = glfw::ffi::KEY_KP_ENTER,
    KpEqual = glfw::ffi::KEY_KP_EQUAL,
    LeftShift = glfw::ffi::KEY_LEFT_SHIFT,
    LeftControl = glfw::ffi::KEY_LEFT_CONTROL,
    LeftAlt = glfw::ffi::KEY_LEFT_ALT,
    LeftSuper = glfw::ffi::KEY_LEFT_SUPER,
    RightShift = glfw::ffi::KEY_RIGHT_SHIFT,
    RightControl = glfw::ffi::KEY_RIGHT_CONTROL,
    RightAlt = glfw::ffi::KEY_RIGHT_ALT,
    RightSuper = glfw::ffi::KEY_RIGHT_SUPER,
    Menu = glfw::ffi::KEY_MENU,
    Unknown = glfw::ffi::KEY_UNKNOWN,
}

/// Wrapper around `glfwGetKeyName`
fn bastard_get_key_name(key: Option<BastardKey>, scancode: Option<glfw::Scancode>) -> Option<String> {
    unsafe {
        glfw::string_from_nullable_c_str(glfw::ffi::glfwGetKeyName(
            match key {
                Some(k) => k as std::os::raw::c_int,
                None => glfw::ffi::KEY_UNKNOWN,
            },
            scancode.unwrap_or(glfw::ffi::KEY_UNKNOWN),
        ))
    }
}

/// Wrapper around `glfwGetKeyName`
#[deprecated(
    since = "0.16.0",
    note = "'key_name' can cause a segfault, use 'get_key_name' instead"
)]
fn bastard_key_name(key: Option<BastardKey>, scancode: Option<glfw::Scancode>) -> String {
    unsafe {
        glfw::string_from_c_str(glfw::ffi::glfwGetKeyName(
            match key {
                Some(k) => k as std::os::raw::c_int,
                None => glfw::ffi::KEY_UNKNOWN,
            },
            scancode.unwrap_or(glfw::ffi::KEY_UNKNOWN),
        ))
    }
}

/// Wrapper around `glfwGetKeyScancode`.
fn bastard_get_key_scancode(key: Option<BastardKey>) -> Option<glfw::Scancode> {
    unsafe {
        match glfw::ffi::glfwGetKeyScancode(match key {
            Some(key) => key as std::os::raw::c_int,
            None => glfw::ffi::KEY_UNKNOWN,
        }) {
            glfw::ffi::KEY_UNKNOWN => None,
            scancode => Some(scancode as glfw::Scancode),
        }
    }
}

impl BastardKey {
    /// Wrapper around `glfwGetKeyName` without scancode
    #[deprecated(
        since = "0.16.0",
        note = "Key method 'name' can cause a segfault, use 'get_name' instead"
    )]
    pub fn name(&self) -> String {
        #[allow(deprecated)]
        bastard_key_name(Some(*self), None)
    }

    /// Wrapper around `glfwGetKeyName` without scancode
    pub fn get_name(&self) -> Option<String> {
        bastard_get_key_name(Some(*self), None)
    }

    /// Wrapper around `glfwGetKeyScancode`.
    pub fn get_scancode(&self) -> Option<glfw::Scancode> {
        bastard_get_key_scancode(Some(*self))
    }
}
