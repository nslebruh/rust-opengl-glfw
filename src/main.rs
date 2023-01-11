extern crate glfw;
extern crate gl;

use gl::types::*;
use cgmath::Vector2;
use glfw::{Action, Context, Key};

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


pub struct DigitalInputState {
    pub key: Key,
    pub pressed: bool,
    pub released: bool
}

impl DigitalInputState {
    pub fn new(key: Key) -> Self {
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

fn main() {
    let vertices: [Vector2<f32>; 3] = [
        Vector2::new(-0.5, 0.0),
        Vector2::new(0.5, 0.0),
        Vector2::new(0.0, 0.5)
    ];

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    let (screen_width, screen_height) = window.get_framebuffer_size();

    gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_scroll_polling(true);

    unsafe{
        gl::Viewport(0, 0, screen_width, screen_height)
    }

    let target_fps: f64 = 60.0;
    let mut w = DigitalInputState::new(Key::W);
    let mut last_time = glfw.get_time();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut w);
        }

        let (w, h) = window.get_framebuffer_size();
        let _ratio: f32 = w as f32 / h as f32;

        unsafe {
            gl::Viewport(0, 0, w, h);
            gl_clear_color(255, 119, 110, 255);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        while glfw.get_time() < last_time + 1.0 / target_fps {
        }
        last_time += 1.0 / target_fps;

        window.swap_buffers();

    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, w: &mut DigitalInputState) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        glfw::WindowEvent::Key(Key::W, _, action, _) if action != Action::Repeat => {
            w.toggle();
        },
        glfw::WindowEvent::CursorPos(x, y) => {
            println!("x: {}, y: {}", x, y)
        },
        glfw::WindowEvent::MouseButton(mouse_button, action, _) => {
            println!("mouse_button: {:?}, action: {:?}", mouse_button, action)
        },
        glfw::WindowEvent::Scroll(x, y) => {
            println!("x: {}, y: {}", x, y)
        }

        _ => {}
    }
}