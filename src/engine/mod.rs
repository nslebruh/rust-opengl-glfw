#![macro_use]
pub mod window;
pub mod input_functions;
pub mod keybinds;
pub mod camera;
pub mod generation;
pub mod shader;
pub mod util;
pub mod buffer_object;
pub mod mesh;
#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    }
}