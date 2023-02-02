use kdtree::{KdTree, distance::squared_euclidean};
use cgmath::{vec3, Vector3};
use std::collections::{HashSet, HashMap};

pub fn create_chunk(num: i32) -> Vec<Vector3<f32>> {
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

