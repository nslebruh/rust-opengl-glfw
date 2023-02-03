use kdtree::{KdTree, distance::squared_euclidean};
use cgmath::{vec3, Vector3};
use std::collections::{HashSet, HashMap};

#[allow(dead_code)]
type Position = Vector3<i32>;


#[allow(dead_code)]
type GenerationSeed = u32;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum BlockType {
    Air = 0,
    Dirt = 1,
    Grass = 2,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BlockData {
    transparent: bool
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Block(BlockType, BlockData);

#[allow(dead_code)]
pub struct Chunk {
    position: Vector3<i32>,
    should_render: bool,
    blocks: HashMap<Position, (Block, bool)>,
    //vec_blocks: Vec<Vec<Vec<(Block, bool)>>>
}
#[allow(dead_code)]
impl Chunk {
    pub fn gen(position: Position, _seed: GenerationSeed) -> Self {
        let mut blocks = HashMap::new();
        for x in 0..=15 {
            for y in -0..=15 {
                for z in 0..=15 {
                    let b_type: BlockType = if y > 12 {BlockType::Air} else if y == 1 {BlockType::Grass} else {BlockType::Dirt};
                    blocks.insert(vec3(x, y, z), (Block(b_type, BlockData {transparent: false}), false));
                }
            }
        }

        let mut output = Self {
            position,
            should_render: false,
            blocks
        };
        output.update_block_visibility();
        output
    }

    pub fn is_edge_position(&self, pos: Position) -> bool {
        let min_bounds = Vector3::new(0, 0, 0);
        let max_bounds = Vector3::new(15, 15, 15);

        pos.x == min_bounds.x 
        || pos.x == max_bounds.x
        || pos.y == min_bounds.y
        || pos.y == max_bounds.y
        || pos.z == min_bounds.z
        || pos.z == max_bounds.z 
    } 

    pub fn update_block_visibility(&mut self) {
        let set: HashSet<Position> = self.blocks.keys().cloned().collect();
        let mut result = HashMap::new();
        let blocks = self.blocks.clone();
        let adjacents: Vec<Vector3<i32>> = vec![
                vec3(1, 0, 0),
                vec3(-1, 0, 0),
                vec3(0, 1, 0),
                vec3(0, -1, 0),
                vec3(0, 0, 1),
                vec3(0, 0, -1),
            ];

        for (pos, block) in blocks {
            let mut count = 0;
            for adjacent in &adjacents {
                if set.contains(&(pos + adjacent)) {
                    count += 1
                }
            }
            result.insert(pos, (block.0, count != 6));
        }
        self.blocks = result
    }
}


pub fn gen_cube_chunk_f32(num: i32) -> Vec<Vector3<f32>> {
    let mut output: Vec<Vector3<f32>> = vec![];
    for x in 0..=num-1 {
        for y in 0..=num-1 {
            for z in 0..=num-1 {
                output.push(vec3(x as f32, y as f32, z as f32))
            }
        }
    }
    output
}

pub fn has_six_adjacent_vector3s(vectors: &[Vector3<f32>]) -> Vec<bool> {
    let mut tree = KdTree::new(3);
    let points: Vec<([f32; 3], usize)> = vectors
        .iter()
        .enumerate()
        .map(
            |v|
            {
                tree.add([v.1.x, v.1.y, v.1.z], v.0).unwrap();
                ([v.1.x, v.1.y, v.1.z], v.0)
            }
        ).collect();

    let mut result = vec![false; vectors.len()];

    for (v, i) in points {
        if (tree.within(&v, 1.0, &squared_euclidean).unwrap().len() - 1) < 6 {
            result[i] = true;
        }
    }

    result
}

//use i32 for position data
#[allow(dead_code)]
pub fn check_adjacent(vectors: &[Vector3<i32>]) -> HashMap<Vector3<i32>, bool> {
    let set: HashSet<Vector3<i32>> = vectors.iter().cloned().collect();
    let mut result = HashMap::new();

    for &vector in vectors {
        let mut count = 0;
        let adjacents = [
            vec3(1, 0, 0),
            vec3(-1, 0, 0),
            vec3(0, 1, 0),
            vec3(0, -1, 0),
            vec3(0, 0, 1),
            vec3(0, 0, -1),
        ];
        for adjacent in adjacents.iter() {
            if set.contains(&(vector + adjacent)) {
                count += 1;
            }
        }
        result.insert(vector, count == 6);
    }

    result
}

#[allow(dead_code)]
pub fn vec_i32_to_f32(vec: Vec<Vector3<i32>>) -> Vec<Vector3<f32>> {
    vec.iter().map(|f| Vector3 {x: f.x as f32, y: f.y as f32, z: f.z as f32}).collect()
}

#[allow(dead_code)]
pub fn vec_f32_to_i32(vec: Vec<Vector3<f32>>) -> Vec<Vector3<i32>> {
    vec.iter().map(|f| {Vector3 {x: f.x as i32, y: f.y as i32, z: f.z as i32}}).collect()
}

//pub fn cube_pos_i32_to_f32_vec(pos: HashMap<Vector3<i32>>) -> Vec<Vector3<>> {}