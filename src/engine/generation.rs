use cgmath::{vec3, Vector3};
use noise::{Fbm, Perlin};
use std::collections::{HashSet, HashMap};
use noise::utils::*;

#[allow(dead_code)]
type IPosition = Vector3<i32>;

type FPosition = Vector3<f32>;

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
pub struct Block(pub BlockType);

#[allow(dead_code)]
pub struct Chunk {
    pub position: Vector3<i32>,
    pub should_render: bool,
    pub blocks: HashMap<IPosition, (Block, bool)>,
    //vec_blocks: Vec<Vec<Vec<(Block, bool)>>>
}
#[allow(dead_code)]
impl Chunk {
    pub fn gen(position: IPosition) -> Self {
        let mut blocks: HashMap<Vector3<i32>, (Block, bool)> = HashMap::new();
        let fbm = Fbm::<Perlin>::new(0);
        let plane_map = PlaneMapBuilder::<_, 2>::new(fbm)
        .set_size(16, 16)
        .set_x_bounds(0.0, 1.0)
        .set_y_bounds(0.0, 1.0)
        .build();

        println!("1 / 256 = {}", 1.0f64 / 256.0f64);

        let _ = &plane_map.write_to_file("test2.png");
        for x in 0..=15 {
            for y in 0..=255 {
                for z in 0..=15 {
                    if y == 0 {
                        println!("y value: {}", plane_map.get_value(x, z) * 256.0f64);
                    }
                    let b_type: BlockType = if y > 12 {BlockType::Air} else if y == 1 {BlockType::Grass} else {BlockType::Dirt};
                    blocks.insert(vec3(x as i32 - 15 , y as i32 - 255, z as i32 - 15), (Block(b_type), false));
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

    pub fn is_edge_position(&self, pos: IPosition) -> bool {
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
        let set: HashSet<IPosition> = self.blocks.keys().cloned().collect();
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

pub fn block_pos_to_f32(pos: IPosition) -> FPosition {
    FPosition {
        x: pos.x as f32,
        y: pos.y as f32,
        z: pos.z as f32
    }
}

pub fn write_example_to_file(map: &NoiseMap, filename: &str) {
    use std::{fs, path::Path};

    let target_dir = Path::new("example_images/");

    if !target_dir.exists() {
        fs::create_dir(target_dir).expect("failed to create example_images directory");
    }

    let target = target_dir.join(Path::new(filename));

    map.write_to_file(target.to_str().unwrap())
}

pub fn noise() {}