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
//pub fn toggle_cursor_mode(args: InputFunctionArguments) {
//    let window = args.window.unwrap();
//    let mode = match window.get_cursor_mode() {
//        glfw::CursorMode::Normal => glfw::CursorMode::Disabled,
//        _ => glfw::CursorMode::Normal,
//    };
//    window.set_cursor_mode(mode)
//}

//pub fn toggle_fullscreen(args: InputFunctionArguments) {
//    let key = args.key.unwrap();
//    let input_state = args.input_state.unwrap();
//    let action = args.action.unwrap();
//    let prev_state = input_state.get(key).unwrap();
//    if Action::Press == *(action) && *prev_state == Action::Release {
//        &input_state.insert(*key, *action);
//    };
//        
//}