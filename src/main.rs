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
    let mut world_vector = World::new(20, 20, 20);
    let mut organisms = vec![];
    let mut blocks = vec![];
    let mut window = Window::new("Main Window");
    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));
    let mut camera = FirstPerson::new(Point3::new(0.0, 0.0, 0.0).into(), Point3::new(0.0, 0.0, 0.0).into());
    camera.set_move_step(5.0);
    camera.set_pitch_step(0.01);
    camera.set_yaw_step(0.01);
    let mut parent_objects = Vec::new();
    let mut last_instant = Instant::now(); // for fps calculation

    // PARAMETERS
    let max_organisms = 200;

    organisms.push(Organism::new());
    organisms.push(Organism::new());
    
    let mut frame_counter = 0;
    while window.render() {                                                     // For each frame
        println!("________Rendering window________");
        frame_counter += 1;
        for mut parent in parent_objects.drain(..) { // delete old render objects
            window.remove_node(&mut parent);
        }

        if frame_counter % 6 = 0 { } // every 6 frames, update the world

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
                if let Some(block) = organism.produce_food() {
                    blocks.push(block);
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
                    CellType::Brain(_) => cube.set_color(0.8, 0.1, 0.1),
                    CellType::Eye => cube.set_color(1.0, 1.0, 1.0),
                    CellType::Armor => cube.set_color(1.0, 1.0, 0.0),
                    CellType::Damager => cube.set_color(1.0, 0.0, 0.0),
                    CellType::Eater => cube.set_color(0.8, 0.8, 0.0),
                    CellType::Producer(_) => cube.set_color(0.0, 1.0, 0.0),
                };
                cube.append_translation(&Translation3::new(cell.local_x as f32, cell.local_y as f32, cell.local_z as f32));
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