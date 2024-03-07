// blocks not attached to any organism

#[derive(Clone)]
pub enum BlockType {
    Food,
    Wall,
}

#[derive(Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Block {
    pub fn new(block_type: BlockType, x: i8, y: i8, z: i8) -> Block {
        Block {
            block_type,
            x,
            y,
            z,
        }
    }
}

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
// main

use std::time::{Instant};

use kiss3d::scene::SceneNode;
use rand::Rng;

use kiss3d::nalgebra::{Translation3, UnitQuaternion, Vector3, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::FirstPerson;

mod organism;
mod cell;
mod world;
mod block;
use organism::*;
use cell::*;
use world::*;
use world::*;
use block::*;

fn main() {
    let mut world_timer : u32 = 0;

    let mut organisms = vec![];
    let mut blocks = vec![];
    let mut window = Window::new("Main Window");
    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));
    // let mut camera = FirstPerson::new(Point3::new(0.0, 0.0, 0.0).into(), Point3::new(0.0, 0.0, 0.0).into());
    let mut camera = FirstPerson::new_with_frustrum(90.0, 0.1, 200.0, Point3::new(0.0, 0.0, 0.0).into(), Point3::new(0.0, 0.0, 0.0).into());
    camera.set_move_step(5.0);
    camera.set_pitch_step(0.01);
    camera.set_yaw_step(0.01);
    let mut parent_objects = Vec::new();
    let mut last_instant = Instant::now(); // for fps calculation
    let mut world = World::new(128, 128, 128);

    // PARAMETERS
    let max_organisms = 100;
    let max_blocks = 100;

    // ---------------------

    organisms.push(Organism::new());
    organisms.push(Organism::new());
    
    let mut frame_counter = 0;
    while window.render() {                                                     // For each frame
        println!("________Rendering window________");
        frame_counter += 1;
        for mut parent in parent_objects.drain(..) { // delete old render objects
            window.remove_node(&mut parent);
        }

        if frame_counter % 6 == 0 { } // every 6 frames, update the world

        let mut new_organisms = Vec::new();
        let num_organisms = organisms.len();

        for organism in &mut organisms {                                        // For each organism
            
            organism.teleport_random();
            if rand::thread_rng().gen_range(0..1000) == 0 { // .1% chance of reproduction
                if num_organisms < max_organisms {
                    let new_organism = organism.reproduce();
                    new_organisms.push(new_organism);
                }
            }
            if rand::thread_rng().gen_range(0..100) == 0 { // 1% chance of food production
                if max_blocks > blocks.len() {
                    if let Some(block) = organism.produce_food() {
                        blocks.push(block);
                    }
                }
            }
            // 1% chance of mutation
            if rand::thread_rng().gen_range(0..100) == 0 {
                organism.mutate();
            }
            
            // WORLD UPDATE LOGIC ENDS

            let mut parent = window.add_group();
            for cell in &organism.cells {                                       // manage rendering
                let mut cube = parent.add_cube(1.0, 1.0, 1.0);
                match cell.cell_type {
                    CellType::Brain(_) => cube.set_color(0.8, 0.5, 0.5),
                    CellType::Eye(_) => cube.set_color(1.0, 1.0, 1.0),
                    CellType::Armor => cube.set_color(1.0, 1.0, 0.0),
                    CellType::Damager => cube.set_color(1.0, 0.0, 0.0),
                    CellType::Eater => cube.set_color(0.8, 0.8, 0.0),
                    CellType::Producer(_) => cube.set_color(0.0, 1.0, 0.0),
                };
                cube.append_translation(&Translation3::new((organism.x + cell.local_x) as f32, (organism.y + cell.local_y) as f32, (organism.z + cell.local_z) as f32));
            }
            for block in &blocks {
                let mut cube = parent.add_cube(1.0, 1.0, 1.0);
                match block.block_type {
                    BlockType::Food => cube.set_color(0.5, 1.0, 0.0),
                    BlockType::Wall => cube.set_color(0.2, 0.2, 0.2),
                };
                cube.append_translation(&Translation3::new(block.x as f32, block.y as f32, block.z as f32));
            }
            parent_objects.push(parent);
        }
        organisms.append(&mut new_organisms); // add new organisms to the list

        let now = Instant::now();
        let framerate = 1.0 / (now.duration_since(last_instant)).as_secs_f32();
        println!("Current framerate: {}", framerate);
        // println!("Number of organisms: {}", organisms.len());
    
        last_instant = now;
        world_timer += 1;
        // println!("World age: {}", world_timer);
    }
}