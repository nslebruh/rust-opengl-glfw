use super::get_bounds;
use nalgebra_glm::{I32Vec3, vec3};
use noise::{Fbm, Perlin, Seedable};
use super::chunk::Chunk;
use super::player::Player;

#[derive(Default)]
pub struct World {
    pub chunks: Vec<Vec<Vec<Chunk>>>,
    pub lower_size: (i32, i32, i32),
    pub upper_size: (i32, i32, i32),
    pub player: Player,
    pub current_chunk: I32Vec3,
    noise: Fbm<Perlin>,
    render_distance: usize,
    seed: u32,
    initial_size: u32,
}

impl World {
    pub fn set_seed(self, seed: u32) -> Self {
        Self {
            noise: self.noise.set_seed(seed),
            seed,
            ..self
        }
    }

    pub fn initial_gen(self, initial_size: u32) -> Self { 
        let (upper_size, lower_size) = get_bounds(initial_size);
        let mut chunks: Vec<Vec<Vec<Chunk>>> = Vec::new();

        for x_pos in 1..=initial_size {
            chunks.push(Vec::new());
            for y_pos in 1..=initial_size {
                chunks.push(Vec::new());
                for z_pos in 1..=initial_size {
                    chunks[x_pos as usize][y_pos as usize].push(Chunk::gen(vec3(x_pos as i32, y_pos as i32, z_pos as i32), &self.noise));
                }
            }
        }

        Self {
            chunks,
            upper_size,
            lower_size,
            initial_size,
            ..self
        }
    }

    //pub fn generate_new_chunk(&mut self, position: I32Vec3) {
    //    self.chunks[position.x as usize][position.y as usize].insert(position.z as usize, Chunk::gen(position, &self.noise))
    //}

    pub fn calculate_visible_chunks(&mut self) {

    }

    pub fn set_render_distance(&mut self, distance: usize) {
        self.render_distance = distance
    }

}