//?https://nercury.github.io/rust/opengl/tutorial/2018/02/10/opengl-in-rust-from-scratch-03-compiling-shaders.html

extern crate gl;

pub mod shader;

use std::ffi::CString;



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

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn _create_square_indices(top_right: u32, top_left: u32, bottom_right: u32, bottom_left: u32) -> Vec<u32> {
    vec![top_right, bottom_right, top_left, bottom_right, top_left, bottom_left]
}