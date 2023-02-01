use crate::input_controller::InputFunctionArguments;

pub struct GameController {
    current_loop: i32,
    pub frames_per_second: f64

}

impl GameController {
    pub fn init() -> Self {
        Self {
            current_loop: 0,
            frames_per_second: 60.0
        }
    }

    pub fn run_loop(&mut self, args: InputFunctionArguments) {
        let camera = args.camera.unwrap();
        self.current_loop += 1;
    }
}