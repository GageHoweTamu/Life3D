// defines the organisms in the world

use core::num;

use rand::{Rng, prelude::IteratorRandom};
// use octree_rs::Octree;
use crate::cell::{Cell, CellType, Brain, Eye, Producer};
use crate::world::{World, Entity};
use crate::block::{Block, BlockType};

#[derive(Clone)]
pub struct Organism { // an organism is a collection of cells, including a brain.
    pub cells: Vec<Cell>, 
    pub health: u8,
    pub energy: u8,
    pub lifespan: u8,
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
            // cells: vec![brain_cell, Cell::new(CellType::Mover, 0, 1, 1, 0)],
            cells: vec![brain_cell],
            health: 100,
            energy: 100,
            lifespan: 100,
            x: 0,
            y: 0,
            z: 0,
        }
    }
    pub fn mutate(&mut self) { // mutates a random cell
        let mut rng = rand::thread_rng();
    
        match rng.gen_range(0..3) {
            0 => self.add_random_cell(),
            1 => {
                let cell_index = rng.gen_range(0..self.cells.len());
                self.cells[cell_index].mutate();
            },
            2 => self.remove_random_cell(),
            _ => (),
        }
    }
    pub fn teleport_random(&mut self) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..2);
        let dy = rng.gen_range(-1..2);
        let dz = rng.gen_range(-1..2);
        let num_mover_cells = self.cells.iter().filter(|cell| matches!(cell.cell_type, CellType::Mover)).count();
        self.x += dx*num_mover_cells as i8;
        self.y += dy*num_mover_cells as i8;
        self.z += dz*num_mover_cells as i8;
        // 50% chance to rotate
        if rng.gen_range(0..2) == 0 {
            self.rotate();
            // println!("organism rotated :)")
        }

    }
    pub fn reproduce(&mut self) -> Organism {
        if self.energy >= 10 { self.energy -= 10; }
        else { self.energy = 0; }

        let mut new_organism = Organism::new();
        let size = self.cells.len() as i8;
        new_organism.cells = self.cells.clone();
        new_organism.x = self.x + rand::thread_rng().gen_range((size*-1)*2 as i8 .. (size*2)+1 as i8); // random offset from parent is proportional to the size of the parent
        new_organism.y = self.y + rand::thread_rng().gen_range((size*-1)*2 as i8 .. (size*2)+1 as i8);
        new_organism.z = self.z + rand::thread_rng().gen_range((size*-1)*2 as i8 .. (size*2)+1 as i8);
        // println!("reproducing");
        new_organism
    }
    pub fn produce_food(&mut self) -> Option<Block> {
        let mut rng = rand::thread_rng();
        for cell in &self.cells {
            if let CellType::Producer(_) = cell.cell_type {
                let dx = rng.gen_range(-1..2);
                let dy = rng.gen_range(-1..2);
                let dz = rng.gen_range(-1..2);
                // println!("Producing food");
                return Some(Block::new(BlockType::Food, self.x + dx, self.y + dy, self.z + dz));
            }
        }
        None
    }
    pub fn add_random_cell(&mut self) {
        // println!("Adding a cell");
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
        let random_rotation = rng.gen_range(0..6);
        self.cells.push(Cell::new(cell_type, random_rotation, dx, dy, dz));
        // println!("An organism added a cell");
    }
    pub fn remove_random_cell(&mut self) { // removes a random cell, except the brain
        let mut rng = rand::thread_rng();
        if self.cells.len() > 1 {
            let cell_index = rng.gen_range(0..self.cells.len());
            if matches!(self.cells[cell_index].cell_type, CellType::Brain(_)) {
                return;
            }
            self.cells.remove(cell_index);
        }
    }
    pub fn shift(&mut self, dx: i8, dy: i8, dz: i8) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
    pub fn eat(&mut self, blocks: &mut Vec<Block>) {
        let nearby_blocks = self.get_nearby_blocks(blocks);
        let mut to_remove = Vec::new();
    
        for (i, block) in nearby_blocks.iter().enumerate() {
            if matches!(block.block_type, BlockType::Food) {
                if self.energy < 100 {
                    self.energy += 10;
                }
                to_remove.push(i);
                // println!("Eating a block, gained energy: {}", self.energy);
            }
        }
    
        for i in to_remove.iter().rev() {
            blocks.remove(*i);
        }
    }
    pub fn is_dead(&self) -> bool {
        if self.health <= 0 || self.energy <= 0 || self.lifespan <= 0 {
            true
        } else {
            false }
    }
    pub fn kill(&self) -> Vec<Block> {
        let mut blocks = Vec::new();
        for cell in &self.cells {
            blocks.push(Block::new(BlockType::Food, self.x + cell.local_x, self.y + cell.local_y, self.z + cell.local_z));
        }
        blocks
    }
    pub fn get_nearby_organisms<'a>(&self, organisms: &'a Vec<Organism>) -> Vec<&'a Organism> {
        let mut nearby_organisms = Vec::new();
        for organism in organisms {
            if (self.x - organism.x).abs() <= 1 && 
                (self.y - organism.y).abs() <= 1 && 
                (self.z - organism.z).abs() <= 1 {
                nearby_organisms.push(organism);
            }
        }
        nearby_organisms
    }
    pub fn get_nearby_blocks<'a>(&self, blocks: &'a Vec<Block>) -> Vec<&'a Block> {
        let mut nearby_blocks = Vec::new();
        for block in blocks {
            if (self.x - block.x).abs() <= 1 && 
                (self.y - block.y).abs() <= 1 && 
                (self.z - block.z).abs() <= 1 {
                    println!("Block found at: {}, {}, {}; Organism at: {}, {}, {}", block.x, block.y, block.z, self.x, self.y, self.z);
                nearby_blocks.push(block);
            }
        }
        nearby_blocks
    }
    pub fn get_hunger(&self) -> Option<f32> {
        for cell in &self.cells {
            if let CellType::Brain(brain) = &cell.cell_type {
                return Some(brain.hunger);
            }
        }
        None
    }
    pub fn get_aggression(&self) -> Option<f32> {
        for cell in &self.cells {
            if let CellType::Brain(brain) = &cell.cell_type {
                return Some(brain.aggression);
            }
        }
        None
    }
    pub fn move_better(&mut self, organisms: &Vec<Organism>, blocks: &Vec<Block>) {
        let eye = self.cells.iter().filter(|cell| matches!(cell.cell_type, CellType::Eye(_))).choose(&mut rand::thread_rng()).unwrap();
        let (dx, dy, dz) = match eye.rotation {
            0 => (1, 0, 0), 1 => (-1, 0, 0), 2 => (0, 1, 0), 3 => (0, -1, 0), 4 => (0, 0, 1), _ => (0, 0, -1),
        };
    
        let (danger_in_sight, food_in_sight) = self.get_nearby_organisms(organisms).iter().fold((0, 0), |(danger, food), organism| {
            let is_in_sight =   (dx != 0 && dx == (organism.x - self.x).signum()) ||
                                (dy != 0 && dy == (organism.y - self.y).signum()) ||
                                (dz != 0 && dz == (organism.z - self.z).signum());
            if is_in_sight {
                (danger + organism.cells.iter().filter(|cell| matches!(cell.cell_type, CellType::Killer)).count(), food)
            } else {
                (danger, food)
            }
        });
    
        let food_in_sight = blocks.iter().filter(|block| {
            let is_in_sight = (dx != 0 && dx == (block.x - self.x).signum()) ||
                                (dy != 0 && dy == (block.y - self.y).signum()) ||
                                (dz != 0 && dz == (block.z - self.z).signum());
            is_in_sight && matches!(block.block_type, BlockType::Food)
        }).count();
    
        let killers = self.cells.iter().filter(|cell| matches!(cell.cell_type, CellType::Killer)).count();
        // brain
        let hunger = self.get_hunger().unwrap();
        let aggression = self.get_aggression().unwrap();
        let decision : f32 = (food_in_sight as f32 * 0.1 * hunger) + (killers as f32 * 0.2 * aggression) - (danger_in_sight as f32);

        if decision < -0.5 {
            self.shift(-dx, -dy, -dz);
            // println!("Running away from danger");
        } else if decision > 0.5 {
            self.shift(dx, dy, dz);
            // println!("Moving towards food, or to kill a nearby organism");
        } else {
            self.teleport_random();
        }
    }
    pub fn damage_nearby_organisms(&self, organisms: &mut Vec<Organism>) {
        for organism in organisms {
            println!("subtracting self.x: {}, organism.x: {}, self.y: {}, organism.y: {}, self.z: {}, organism.z: {}", self.x, organism.x, self.y, organism.y, self.z, organism.z);
            if (self.x - organism.x).abs() <= 1 && 
                (self.y - organism.y).abs() <= 1 && 
                (self.z - organism.z).abs() <= 1 {
                if organism.health > 10 { organism.health -= 10; } else { organism.health = 0; }
                // println!("Damaging nearby organism");
            }
        }
    }

    pub fn rotate(&mut self) {
        let direction = rand::random::<u8>() % 6; // Random direction between 0 and 5
    
        for cell in &mut self.cells {
            let (new_x, new_y, new_z) = match direction {
                0 => (cell.local_x, -cell.local_z, cell.local_y),  // x
                1 => (cell.local_x, cell.local_z, -cell.local_y),  // -x
                2 => (-cell.local_z, cell.local_y, cell.local_x),  // y
                3 => (cell.local_z, cell.local_y, -cell.local_x),  // -y
                4 => (cell.local_x, cell.local_y, cell.local_z),  // z
                _ => (-cell.local_x, cell.local_y, cell.local_z),  // -z
            };
    
            cell.local_x = new_x;
            cell.local_y = new_y;
            cell.local_z = new_z;
    
            // Update cell rotation
            cell.rotation = (cell.rotation + direction as i8) % 6;
        }
    }
        
}