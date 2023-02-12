use glm::I32Vec3;
use noise::{Fbm, Perlin, NoiseFn};
use block_mesh::{visible_block_faces, UnitQuadBuffer, ndshape::{ConstShape, ConstShape3u32}};
use super::block::{Block, BlockType};

type ChunkShape = ConstShape3u32<16, 16, 16>;

pub struct Chunk {
    pub position: I32Vec3,
    pub blocks: [Block; ChunkShape::SIZE as usize],
    pub mesh: UnitQuadBuffer
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            position: Default::default(),
            blocks: [Block::default(); ChunkShape::SIZE as usize],
            mesh: UnitQuadBuffer::default()

        }
    }
}

impl Chunk {
    pub fn gen(position: I32Vec3, noise: &Fbm<Perlin>) -> Self {
        Self::default()
    }

    pub fn create_mesh(&mut self) {

    } 
}

//pub struct Chunk {
//    pub position: I32Vec3,
//    pub blocks: Vec<Vec<Vec<Block>>>,
//    pub mesh: Vec<f32>
//}
//
//impl Chunk {
//    pub fn gen(position: I32Vec3, noise: &Fbm<Perlin>) -> Self {
//        let x_offset = position.x * 16;
//        let y_offset = position.y * 16;
//        let z_offset = position.z * 16;
//        let mut blocks: Vec<Vec<Vec<Block>>> = Vec::new();
//        for x_pos in 0usize..=15 {
//            blocks.push(Vec::new());
//            for y_pos in 0usize..=15 {
//                blocks.push(Vec::new());
//                for z_pos in 0usize..=15 {
//                    blocks[x_pos][y_pos].push(Block(
//                        if noise.get([
//                            (x_pos as i32 + x_offset) as f64,
//                            (y_pos as i32 + y_offset) as f64,
//                            (z_pos as i32 + z_offset) as f64
//                            ]).trunc() == -1.0_f64 {BlockType::Air} else {BlockType::Grass},
//                            false 
//                    )
//                )}
//            }
//        }
//        Self {
//            position,
//            blocks,
//            mesh: vec![]
//        }
//    }
//
//    pub fn update(&mut self, position: I32Vec3, result: Block) {
//        self.blocks[position.x as usize][position.y as usize][position.z as usize] = result
//    }
//}