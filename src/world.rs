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
    pub grid: Vec<Vec<Vec<Option<Entity>>>>,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
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
    pub fn set_entity(&mut self, x: usize, y: usize, z: usize, entity: Option<Entity>) {
        if x < self.width && y < self.height && z < self.depth {
            self.grid[x][y][z] = entity;
        } else {
            println!("OOB in set_entity ");
        }
    }

    pub fn get_entity(&self, x: usize, y: usize, z: usize) -> Option<&Entity> {
        if x < self.width && y < self.height && z < self.depth {
            self.grid[x][y][z].as_ref()
        } else {
            // print!("OOB in get_entity ");
            None
        }
    }
    pub fn get_adjacent_entities(&self, x: usize, y: usize, z: usize) -> Vec<((usize, usize, usize), &Entity)> {
        let mut adjacent_entities = Vec::new();
        let offsets = [
            (-1, -1, -1), (0, -1, -1), (1, -1, -1),
            (-1, 0, -1), (0, 0, -1), (1, 0, -1),
            (-1, 1, -1), (0, 1, -1), (1, 1, -1),
            (-1, -1, 0), (0, -1, 0), (1, -1, 0),
            (-1, 0, 0), (1, 0, 0),
            (-1, 1, 0), (0, 1, 0), (1, 1, 0),
            (-1, -1, 1), (0, -1, 1), (1, -1, 1),
            (-1, 0, 1), (0, 0, 1), (1, 0, 1),
            (-1, 1, 1), (0, 1, 1), (1, 1, 1),
        ];

        for (dx, dy, dz) in offsets.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            let nz = (z as isize + dz) as usize;

            if let Some(entity) = self.get_entity(nx, ny, nz) {
                adjacent_entities.push(((nx, ny, nz), entity));
            }
        }

        adjacent_entities
    }
}