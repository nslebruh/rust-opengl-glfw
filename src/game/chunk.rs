use glm::{I32Vec3, Vec3, Vec2};
use noise::{Fbm, Perlin, NoiseFn};
use block_mesh::{greedy_quads, GreedyQuadsBuffer, ndshape::{ConstShape, ConstShape3u32}, RIGHT_HANDED_Y_UP_CONFIG, QuadCoordinateConfig};
use crate::engine::mesh::{Mesh, Vertex, Texture};

use super::block::{Block, BlockType};

pub type ChunkShape = ConstShape3u32<16, 16, 16>;

pub struct Chunk {
    pub position: I32Vec3,
    pub blocks: [Block; ChunkShape::SIZE as usize],
    pub mesh: Mesh
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            position: Default::default(),
            blocks: [Block::default(); ChunkShape::SIZE as usize],
            mesh: Mesh::default()

        }
    }
}

impl Chunk {
    pub fn gen(position: I32Vec3, noise: &Fbm<Perlin>) -> Self {
        let x_offset = position.x * 16;
        let y_offset = position.y * 16;
        let z_offset = position.z * 16;
        let mut blocks = [Block::default(); ChunkShape::SIZE as usize];
        for i in 0..ChunkShape::SIZE {
            let [x, y, z] = ChunkShape::delinearize(i);
            blocks[i as usize] = if noise.get([(x as i32 + x_offset) as f64, (y as i32 + y_offset) as f64, (z as i32 + z_offset) as f64]).trunc() > -1.0 {
                blocks[i as usize]
            } else {
                Block(BlockType::Air)
            }
        };

        let mut buffer = GreedyQuadsBuffer::new(blocks.len());
        greedy_quads(&blocks, &ChunkShape {}, [0; 3], [15; 3], &RIGHT_HANDED_Y_UP_CONFIG.faces, &mut buffer);
        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;
        let mut indices = Vec::with_capacity(num_indices);
        let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
        let mut textures: Vec<Texture> = Vec::new();
        textures.push(Texture { id: 0, type_: String::from("texture_diffuse"), path: String::from("dirt.png") });
        println!("quads created: {}", buffer.quads.num_quads());

        for (group, face) in buffer.quads.groups.into_iter().zip(RIGHT_HANDED_Y_UP_CONFIG.faces.into_iter()) {
            for (num, quad) in group.into_iter().enumerate() {
                indices.extend_from_slice(&face.quad_mesh_indices(num as u32));
                let position = &face.quad_mesh_positions(&quad, 1.0);
                let normal = &face.quad_mesh_normals();
                let tex_coord = &face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad);
                for j in 0..3 {
                    vertices.push(Vertex {
                        position: Vec3::from(position[j]),
                        normal: Vec3::from(normal[j]),
                        texCoords: Vec2::from(tex_coord[j]),
                        ..Default::default()
                    });
                }
                //positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                //normals.extend_from_slice(&face.quad_mesh_normals());
                //textures.extend_from_slice(&face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad))

            }
        }

        let mut mesh: Mesh = Mesh::new(vertices, indices, textures);

        Self {
            position,
            blocks,
            mesh
        }
    }

    pub unsafe fn create_mesh(&mut self, id: u32) -> u32 {
        id
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