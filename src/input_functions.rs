use crate::{input_controller::{InputFunctionArguments, InputFunction}, camera::CameraMovement};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT_FUNCTIONS_VECTOR: Vec<InputFunction> = vec![
        (InputFunction::new("set_should_close", set_window_should_close)),
        (InputFunction::new("test", test)),
        (InputFunction::new("camera_forward", camera_forward)),
        (InputFunction::new("camera_backward", camera_backward)),
        (InputFunction::new("camera_left", camera_left)),
        (InputFunction::new("camera_right", camera_right)),
    ];
}
 

pub fn set_window_should_close(args: InputFunctionArguments) {
    args.window.unwrap().set_should_close(true)
}

pub fn test(_args: InputFunctionArguments) {
    println!("test")
}

pub fn camera_forward(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::FORWARD, args.delta_time.unwrap())
}

pub fn camera_backward(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::BACKWARD, args.delta_time.unwrap())
}

pub fn camera_left(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::LEFT, args.delta_time.unwrap())
}

pub fn camera_right(args: InputFunctionArguments) {
    args.camera.unwrap().process_action_input(CameraMovement::RIGHT, args.delta_time.unwrap())
}