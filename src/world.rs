// defines the world and its components
use std::collections::HashMap;
use crate::organism::Organism;

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

pub struct World {
    cells: Vec<Vec<Vec<Voxel>>>,
    organisms: HashMap<(usize, usize, usize), Organism>,
    width: usize,
    height: usize,
    depth: usize,
}

impl World {
    pub fn new(width: usize, height: usize, depth: usize) -> World {
        let mut cells = Vec::with_capacity(width);
        for _ in 0..width {
            let mut row = Vec::with_capacity(height);
            for _ in 0..height {
                let mut column = Vec::with_capacity(depth);
                for _ in 0..depth {
                    column.push(Voxel {
                        voxel_type: VoxelType::Empty,
                        warmth: 0.0,
                    });
                }
                row.push(column);
            }
            cells.push(row);
        }
        World {
            cells,
            organisms: HashMap::new(),
            width,
            height,
            depth,
        }
    }
    pub fn move_organism(&mut self, old_x: usize, old_y: usize, old_z: usize, new_x: usize, new_y: usize, new_z: usize) {
        println!("Moving organism from ({}, {}, {}) to ({}, {}, {})", old_x, old_y, old_z, new_x, new_y, new_z);
        if let Some(organism) = self.organisms.remove(&(old_x, old_y, old_z)) {
            self.organisms.insert((new_x, new_y, new_z), organism);
        }
    }
}
