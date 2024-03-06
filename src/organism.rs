// defines the organisms in the world

use rand::Rng;

use crate::cell::{Cell, CellType};

pub struct Organism {
    pub cells: Vec<Cell>,
    pub brain: Brain,
    health: f32,
    energy: f32,
    age: u32,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Organism {
    pub fn new() -> Organism {
        Organism {
            cells: vec![Cell::new(CellType::Eater, 1, 1, 0)],
            brain: Brain {
                aggression: 0.5,
                hunger: 0.5,
            },
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
        let cell_index = rng.gen_range(0..self.cells.len());
        self.cells[cell_index].mutate();
    }
    pub fn teleport_random(&mut self) {
        let mut rng = rand::thread_rng();
        let dx = rng.gen_range(-1..2);
        let dy = rng.gen_range(-1..2);
        let dz = rng.gen_range(-1..2);

        self.x += dx;
        self.y += dy;
        self.z += dz;

        for cell in &mut self.cells {
            cell.shift(dx, dy, dz);
        }
    }
}

pub struct Brain {
    pub aggression: f32,    // How likely the organism is to attack
                            // 0.0: never attacks
                            // 0.5: attacks smaller organisms (default)
                            // 1.0: attacks everything
    pub hunger: f32, // How likely the organism is to pursue food in spite of danger
                        // 0.0: never pursues food
                        // 0.5: pursues food if not in danger (default)
                        // 1.0: always pursues food
}