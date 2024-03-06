// defines the organisms in the world

use rand::Rng;

use crate::{cell::{Cell, CellType}, Brain};

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
            cells: vec![Cell::new(CellType::Body, 1, 1, 0)],
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
    pub fn teleport_random (&mut self) { // moves the organism randomly
        let mut rng = rand::thread_rng();
        self.x += rng.gen_range(-1..2);
        self.y += rng.gen_range(-1..2);
        self.z += rng.gen_range(-1..2);
    }
}
