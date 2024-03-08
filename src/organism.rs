// defines the organisms in the world

use rand::Rng;
// use octree_rs::Octree;
use crate::cell::{Cell, CellType, Brain, Eye, Producer};
use crate::world::World;
use crate::block::{Block, BlockType};

#[derive(Clone)]
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
        let brain_cell = Cell::new(CellType::Brain(brain), 0, 0, 0, 0);
        Organism {
            cells: vec![brain_cell, Cell::new(CellType::Eater, 0, 1, 1, 0)],
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
        let cell_type = match rng.gen_range(0..7) { // add random rotation
            0 => CellType::Eye(Eye {}),
            1 => CellType::Armor,
            2 => CellType::Killer,
            3 => CellType::Eater,
            4 => CellType::Producer(Producer {}),
            5 => CellType::Mover,
            _ => CellType::Eater,
        };
        let dx = rng.gen_range(-1..2);
        let dy = rng.gen_range(-1..2);
        let dz = rng.gen_range(-1..2);
        self.cells.push(Cell::new(cell_type, 0, dx, dy, dz));
        println!("An organism added a cell");
    }
    pub fn move_based_on_vision(&mut self, world: &World) { // still in testing
        // Iterate over all cells in the organism
        for cell in &self.cells {
            // If the cell is an Eye
            if let CellType::Eye(eye) = &cell.cell_type {
                // Use the eye to look at the world
                let (food_blocks, killer_cells) = eye.look(cell.rotation, world, self.x as usize, self.y as usize, self.z as usize);

                // Decide the direction of movement based on what the eye sees
                // For example, if there are more food blocks than killer cells, move forward
                // Otherwise, move backward
                // This is a very simple decision-making process and can be made more complex
                if food_blocks > killer_cells {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }

                // Ensure the organism stays within the bounds of the world
                self.x = self.x.max(0).min(world.width as i8 - 1);
                self.y = self.y.max(0).min(world.height as i8 - 1);
                self.z = self.z.max(0).min(world.depth as i8 - 1);
            }
        }
    }
}