use block_mesh::{Voxel, MergeVoxel, VoxelVisibility};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum BlockType {
    Air,
    Grass
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Block(pub BlockType, pub bool);

impl Default for Block {
    fn default() -> Self {
        Self(BlockType::Air, false)
    }
}

impl Voxel for Block {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.1 {
            VoxelVisibility::Opaque
        } else {
            VoxelVisibility::Empty
        }
    }
}

impl MergeVoxel for Block {
    type MergeValue = bool;

    fn merge_value(&self) -> Self::MergeValue {
        self.1
    }
}