// defines the cells the organisms are made of
use rand::Rng;
use crate::block::{Block, BlockType};
use crate::world::{World, Entity};

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

#[derive(Debug)]
#[derive(Clone)]
pub struct Eye {
    pub rotation: i8,   // 0: forward, 1: backward, 2: left, 3: right, 4: up, 5: down
                        // This determines where the eye looks in the look() function
}
impl Eye {
    /*
    pub fn look(&self) -> (i8, i8) { // returns number of food blocks and killer cells in front of the eye
        // look logic
        // scans the grid in the direction of rotation
        // looks for food blocks and killer cells
        println!("Detected: ...");
        //(0, 0)
    }
    */
    pub fn look(&self, world: &World, x: usize, y: usize, z: usize) -> (i32, i32) {
        let mut food_blocks = 0;
        let mut killer_cells = 0;

        // Define the direction of look based on rotation
        let (dx, dy, dz) = match self.rotation {
            0 => (1, 0, 0),  // forward
            1 => (-1, 0, 0), // backward
            2 => (0, 1, 0),  // left
            3 => (0, -1, 0), // right
            4 => (0, 0, 1),  // up
            5 => (0, 0, -1), // down
            _ => (0, 0, 0),  // invalid rotation
        };

        // Start from the current position and move in the direction of look
        let mut current_x = x as i32;
        let mut current_y = y as i32;
        let mut current_z = z as i32;

        // Continue looking as long as we're within the bounds of the world
        while current_x >= 0 && current_x < world.width as i32
            && current_y >= 0 && current_y < world.height as i32
            && current_z >= 0 && current_z < world.depth as i32
        {
            if let Some(entity) = world.get_entity(current_x as usize, current_y as usize, current_z as usize) {
                match entity {
                    Entity::Block(block) => {
                        if block.block_type == BlockType::Food {
                            food_blocks += 1; 
                        }
                    }
                    Entity::Cell(cell) => {
                        if let CellType::Brain(brain) = &cell.cell_type {
                            if brain.aggression > 0.5 {
                                killer_cells += 1;
                            }
                        }
                    }
                }
            }

            // Move to the next position in the direction of look
            current_x += dx;
            current_y += dy;
            current_z += dz;
        }

        (food_blocks, killer_cells)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum CellType {
    Brain(Brain), // The brain cell is the first cell in the organism, and cannot be removed
    Eye(Eye),
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
                    1 => CellType::Eye(Eye { rotation: rng.gen_range(0..6) }),
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