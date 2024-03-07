// defines the world and its components
use std::collections::HashMap;
use crate::organism::Organism;
use crate::block::Block;
use crate::cell::Cell;

#[derive(Clone)]
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
        let grid = vec![vec![vec![None; depth]; height]; width]; // declares a 3D grid of empty entities
        World {
            grid,
            width,
            height,
            depth,
        }
    }
    pub fn set_entity(&mut self, x: usize, y: usize, z: usize, entity: Entity) {
        if x < self.width && y < self.height && z < self.depth {
            self.grid[x][y][z] = Some(entity);
        } else {
            println!("Coordinates out of bounds");
        }
    }

    pub fn get_entity(&self, x: usize, y: usize, z: usize) -> Option<&Entity> {
        if x < self.width && y < self.height && z < self.depth {
            self.grid[x][y][z].as_ref()
        } else {
            println!("Coordinates out of bounds");
            None
        }
    }
}