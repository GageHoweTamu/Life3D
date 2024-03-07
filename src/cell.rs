// defines the cells the organisms are made of
use rand::Rng;
use crate::block::{Block, BlockType};

#[derive(Debug)]
#[derive(Clone)]
pub struct Brain {
    pub aggression: f32,
    pub hunger: f32,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Producer {}
impl Producer {
    pub fn produce(&self) -> Block {
        Block::new(BlockType::Food, 0, 0, 0)
    }
}
pub struct Eye {}
impl Eye {
    // returns number of food blocks and killer cells in front of the eye
    pub fn look(&self) {
        println!("Detected: ...");
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum CellType {
    Brain(Brain), // The brain cell is the first cell in the organism, and cannot be removed
    Eye,
    Armor,
    Damager,
    Eater,
    Producer(Producer),
}

#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub local_x: i8,
    pub local_y: i8,
    pub local_z: i8,
}

impl Cell {
    pub fn new(cell_type: CellType, local_x: i8, local_y: i8, local_z: i8) -> Cell {
        Cell {
            cell_type,
            local_x,
            local_y,
            local_z,
        }
    }
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        match &mut self.cell_type {
            CellType::Brain(x) => {
                x.aggression = (x.aggression + rng.gen_range(-0.2..0.2)).clamp(0.0, 1.0);
                x.hunger = (x.hunger + rng.gen_range(-0.2..0.2)).clamp(0.0, 1.0);
            }
            _ => {
                let mutated_type = match rng.gen_range(0..6) {
                    1 => CellType::Eye,
                    2 => CellType::Armor,
                    3 => CellType::Damager,
                    4 => CellType::Eater,
                    5 => CellType::Producer(Producer {}),
                    _ => self.cell_type.clone(),
                };
                self.cell_type = mutated_type;
            }
        }
        println!("Mutated cell to {:?}", self.cell_type);
    }
    pub fn shift(&mut self, x: i8, y: i8, z: i8) {
        self.local_x += x;
        self.local_y += y;
        self.local_z += z;
    }
    pub fn clone(&self) -> Cell { // this may not be needed
        let mut temp = Cell {
            cell_type: self.cell_type.clone(),
            local_x: self.local_x,
            local_y: self.local_y,
            local_z: self.local_z,
        };
        match &self.cell_type {
            CellType::Brain(x) => {
                if let CellType::Brain(y) = &mut temp.cell_type {
                    y.aggression = x.aggression;
                    y.hunger = x.hunger;
                }
            }
            _ => {}
        }
        temp
    }
}