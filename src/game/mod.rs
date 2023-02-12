pub mod world;
pub mod chunk;
pub mod block;
pub mod player;

use world::World;

pub struct Game {
    pub world: World,
    pub renderer: Renderer
}

impl Game {
    pub fn render(&self) {

    }
}

pub struct Camera {
    
}

pub struct Renderer {

}

pub fn get_bounds(x: u32) -> ((i32, i32, i32), (i32, i32, i32)) {
    let upper_bounds: (i32, i32, i32);
    let result = x as i32 / 2;
    let lower_bounds = (-result, -result, -result);
    let mod_result = x as i32 % 2;
    if mod_result == 1 {
        upper_bounds = (result, result, result);
    } else {
        upper_bounds = (result - 1, result - 1, result - 1)
    }
    (lower_bounds, upper_bounds)
}