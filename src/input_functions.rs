use glfw::Action;

use crate::{input_controller::InputFunctionArguments, camera::CameraMovement};

pub fn set_window_should_close(args: InputFunctionArguments) {
    args.window.unwrap().set_should_close(true)
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

pub fn toggle_cursor_mode(args: InputFunctionArguments) {
    let window = args.window.unwrap();
    let mode = match window.get_cursor_mode() {
        glfw::CursorMode::Normal => glfw::CursorMode::Disabled,
        _ => glfw::CursorMode::Normal,
    };
    window.set_cursor_mode(mode)
}

pub fn toggle_fullscreen(args: InputFunctionArguments) {
    let prev_state = ((&args).input_state.unwrap()).get(&args.key.unwrap()).unwrap().clone();
    if (&args).action.unwrap().clone() == Action::Press {}
}