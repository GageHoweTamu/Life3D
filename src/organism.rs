// defines the organisms in the world

use rand::Rng;

use crate::cell::{Cell, CellType};

pub struct Organism {
    pub cells: Vec<Cell>,
    health: f32,
    energy: f32,
    age: u32,
    pub x: i8,
    pub y: i8,
    pub z: i8,
    aggressiveness: f32, // 0.0 to 1.0
}

impl Organism {
    pub fn new() -> Organism {
        Organism {
            cells: vec![Cell::new(CellType::Body, 1, 1, 0)],
            health: 100.0,
            energy: 100.0,
            age: 0,
            x: 0,
            y: 0,
            z: 0,
            aggressiveness: 0.0,
        }
    }
    pub fn mutate(&mut self) {
        // do something here
    }
    pub fn teleport_random (&mut self) {
        let mut rng = rand::thread_rng();
        self.x += rng.gen_range(-1..2);
        self.y += rng.gen_range(-1..2);
        self.z += rng.gen_range(-1..2);
    }
}
