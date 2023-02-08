use cgmath::{vec3, Vector3};
use std::collections::{HashSet, HashMap};
use simdnoise::NoiseBuilder;

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

pub struct World {
    pub chunks: Vec<Vec<Chunk>>,
    seed: i32,
    noise: Vec<Vec<Vec<[i32; 16]>>>
}

impl World {
    pub fn new(square_size: usize, seed: i32) -> Self {
        let size = square_size - 1;
        let noise = simdnoise::NoiseBuilder::fbm_2d( (16 * square_size) as usize, (16 * square_size) as usize).with_seed(seed).generate_scaled(0.0, 2.56);
        let mut chunks_noise: Vec<Vec<Vec<[i32; 16]>>> = Vec::new();
        let mut chunks: Vec<Vec<Chunk>> = Vec::new();
        for i in 0..=size {
            chunks_noise.push(Vec::new());
            chunks.push(Vec::new());
            for _ in 0..=size {
                chunks_noise[i].push(Vec::new())
            }
        }

        let mut z_pos: usize = 0;
        let mut x_pos: usize = 0;
        let mut line_num: usize = 0;

        for (_, chunk) in noise.chunks(16).enumerate() {
            println!("{x_pos}");
            chunks_noise[x_pos][z_pos].push(f32_slice_to_i32(chunk));
            x_pos += 1;
            if x_pos == square_size {
                x_pos = 0;
                line_num += 1;
            }
            if line_num == 16 {
                z_pos += 1;
                line_num = 0;
            }
        }
        for (l, _) in chunks_noise.iter().enumerate() {
            for (n, _) in chunks_noise.iter().enumerate() {
                println!("{:?}", chunks_noise[l][n].len())
            }
        }

        for x in 0..=size {
            for z in 0..=size {
                chunks[x].push(Chunk::gen(vec3(x as i32, 0, z as i32), &chunks_noise[x][z]))
            }
        }

        for (m, _) in chunks.iter().enumerate() {
            println!("{:?}", chunks[m].len());
        }  

        Self {
            seed,
            chunks,
            noise: chunks_noise
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Chunk {
    pub position: Vector3<i32>,
    pub should_render: bool,
    pub blocks: HashMap<IPosition, (Block, bool)>,
    //vec_blocks: Vec<Vec<Vec<(Block, bool)>>>
}
#[allow(dead_code)]

impl Chunk {
    pub fn gen(position: IPosition, noise: &Vec<[i32; 16]>) -> Self {
        println!("{:?}", &position);
        println!("{}", noise.len());
        let mut blocks: HashMap<Vector3<i32>, (Block, bool)> = HashMap::new();
        
        for x in 0..=15 {
            for z in 0..=15 {
                for y in 0..=noise[x][z] {
                    blocks.insert(vec3(x as i32, y as i32, z as i32), (Block(BlockType::Dirt), false));
                }
            }
        }
        


        //let mut z_pos: usize = 0;
        //for (i, n) in noise.iter().enumerate() {
        //    let y = (*n * 100.0).trunc() as i32;
        //    println!("{}", y);
        //    if (i+1) % 16 == 0 {
        //        z_pos += 1
        //    }
        //    blocks.insert(vec3(i as i32 % 16, y as i32, z_pos as i32), (Block(BlockType::Dirt), false));
//
//
//
        //}
        //let fbm = Fbm::<Perlin>::new(0);
        //let plane_map = PlaneMapBuilder::<_, 2>::new(fbm)
        //.set_size(16, 16)
        //.set_x_bounds(0.0, 1.0)
        //.set_y_bounds(0.0, 1.0)
        //.build();
        //println!("1 / 256 = {}", 1.0f64 / 256.0f64);
        //let _ = &plane_map.write_to_file("test2.png");
        //for x in 0..=15 {
        //    for y in 0..=255 {
        //        for z in 0..=15 {
        //            //if y == 0 {
        //            //    println!("y value: {}", plane_map.get_value(x, z) * 256.0f64);
        //            //}
        //            let b_type: BlockType = if y > 12 {BlockType::Air} else if y == 1 {BlockType::Grass} else {BlockType::Dirt};
        //            blocks.insert(vec3(x as i32 - 15 , y as i32 - 255, z as i32 - 15), (Block(b_type), false));
        //        }
        //    }
        //};


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


fn f32_slice_to_i32(slice: &[f32]) -> [i32; 16] {
    let mut output: [i32; 16] = [0; 16];
    for (i, f) in slice.iter().enumerate() {
        output[i] = (f * 100.0).trunc() as i32 / 16;
    }
    println!("{:?}", output);
    output
}