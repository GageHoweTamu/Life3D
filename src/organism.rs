// defines the organisms in the world

use rand::Rng;
// use octree_rs::Octree;
use crate::cell::{Cell, CellType, Brain, Eye, Producer};
use crate::block::{Block, BlockType};

pub struct Organism { // an organism is a collection of cells, including a brain.
    pub cells: Vec<Cell>, 
    health: f32,
    energy: f32,
    age: u32,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}
impl Organism {
    pub fn new() -> Organism {
        let brain = Brain {
            aggression: 0.5,
            hunger: 0.5,
        };
        let brain_cell = Cell::new(CellType::Brain(brain), 0, 0, 0);
        Organism {
            cells: vec![brain_cell, Cell::new(CellType::Eater, 1, 1, 0)],
            health: 100.0,
            energy: 100.0,
            age: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
    pub fn mutate(&mut self) { // mutates a random cell
        let mut rng = rand::thread_rng();
        // determine whether to add a cell or mutate an existing one
    
        match rng.gen_range(0..2) {
            0 => self.add_random_cell(),
            1 => {
                let cell_index = rng.gen_range(0..self.cells.len());
                self.cells[cell_index].mutate();
            },
            _ => (),
        }
    }
    pub fn teleport_random(&mut self) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..2);
        let dy = rng.gen_range(-1..2);
        let dz = rng.gen_range(-1..2);

        self.x += dx;
        self.y += dy;
        self.z += dz;

    }
    pub fn reproduce(&self) -> Organism { // this currently does not mutate
        let mut new_organism = Organism::new();
        new_organism.cells = self.cells.clone();
        new_organism.x = self.x + rand::thread_rng().gen_range(-10..11);
        new_organism.y = self.y + rand::thread_rng().gen_range(-10..11);
        new_organism.z = self.z + rand::thread_rng().gen_range(-10..11);
        new_organism
    }
    pub fn produce_food(&mut self) -> Option<Block> {
        let mut rng = rand::thread_rng();
        for cell in &self.cells {
            if let CellType::Producer(_) = cell.cell_type {
                let dx = rng.gen_range(-1..2);
                let dy = rng.gen_range(-1..2);
                let dz = rng.gen_range(-1..2);
                println!("Producing food at ({}, {}, {})", self.x + dx, self.y + dy, self.z + dz);
                return Some(Block::new(BlockType::Food, self.x + dx, self.y + dy, self.z + dz));
            }
        }
        None
    }
    pub fn add_random_cell(&mut self) {
        let mut rng = rand::thread_rng();
        let cell_type = match rng.gen_range(0..5) {
            0 => CellType::Eye(Eye { rotation: rng.gen_range(0..6) }),
            1 => CellType::Armor,
            2 => CellType::Damager,
            3 => CellType::Eater,
            4 => CellType::Producer(Producer {}),
            _ => CellType::Eater,
        };
        let dx = rng.gen_range(-1..2);
        let dy = rng.gen_range(-1..2);
        let dz = rng.gen_range(-1..2);
        self.cells.push(Cell::new(cell_type, dx, dy, dz));
    }
}