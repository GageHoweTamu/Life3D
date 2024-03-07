// defines the world and its components
use std::collections::HashMap;
use crate::organism::Organism;
use crate::block::Block;
use crate::cell::Cell;

pub enum VoxelType {
    Empty,
    Food,
    Poison,
    Wall,
    Occupied,
}

pub struct Voxel {
    voxel_type: VoxelType,
    warmth: f32,
}

pub enum Entity {
    Block(Block),
    Cell(Cell),
}

pub struct World {
    grid: Vec<Vec<Vec<Option<Entity>>>>,
    width: usize,
    height: usize,
    depth: usize,
}
impl World {
    pub fn new(width: usize, height: usize, depth: usize) -> World {
        let grid = vec![vec![vec![None; depth]; height]; width];
        World {
            grid,
            width,
            height,
            depth,
        }
    }
}