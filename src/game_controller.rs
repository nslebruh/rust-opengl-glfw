pub struct GameController {
    current_loop: i32,

}

impl GameController {
    pub fn init() -> Self {
        Self {
            current_loop: 0,
        }
    }

    pub fn run_loop(&mut self) {
        self.current_loop += 1;
        match self.current_loop % 60 {
            0 => println!("Seconds passed: {}", self.current_loop / 60),
            _ => ()
        }
    }
}