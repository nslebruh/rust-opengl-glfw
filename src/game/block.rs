use block_mesh::{Voxel, MergeVoxel, VoxelVisibility};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Stone
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Block(pub BlockType);

impl Voxel for Block {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.0 != BlockType::Air {
            VoxelVisibility::Opaque
        } else {
            VoxelVisibility::Empty
        }
    }
}

impl MergeVoxel for Block {
    type MergeValue = bool;

    fn merge_value(&self) -> Self::MergeValue {
        self.0 != BlockType::Air
    }
}