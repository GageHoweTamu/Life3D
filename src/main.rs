// main

use std::time::{Instant};

use kiss3d::scene::SceneNode;
use rand::Rng;

use kiss3d::nalgebra::{Point, Point3, Translation3, UnitQuaternion, Vector3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::FirstPerson;
use kiss3d::text::TextRenderer;

mod organism;
mod cell;
mod world;
mod block;
use organism::*;
use cell::*;
use world::*;
use world::*;
use block::*;

use std::sync::{Arc, Mutex};

use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;

fn update_world(organisms: &mut Vec<Organism>, new_organisms: &mut Vec<Organism>, blocks: &mut Vec<Block>, max_organisms: usize, max_blocks: usize, sim_world: &mut World) {
    let organisms_len = organisms.len();
    // sim_world.clear(); // this causes a slowdown

    for organism in organisms.iter_mut() {
        // println!("Organism energy: {}", organism.energy);

        for cell in &organism.cells { // add the cells to the world vector
            // sim_world.set_entity((organism.x + cell.local_x) as usize, (organism.y + cell.local_y) as usize, (organism.z + cell.local_z) as usize, Some(Entity::Cell(cell.clone())));
        }

        if rand::thread_rng().gen_range(0..10) == 0 { // 1% chance of reproduction
            if organisms_len < max_organisms {
                let mut new_organism = organism.reproduce();
                if rand::thread_rng().gen_range(0..2) == 0 {
                    new_organism.mutate(); // reproduced organisms have a 50% chance of mutation
                }
                new_organisms.push(new_organism);
                println!("A new organism was born!");
            }
        }
        if rand::thread_rng().gen_range(0..10) == 0 { // 10% chance of food production
            if max_blocks > blocks.len() {
                if let Some(block) = organism.produce_food() {
                    blocks.push(block);
                }
            }
        }
        if rand::thread_rng().gen_range(0..1000) == 0 { // 0.1% chance of random mutation
            organism.mutate();
            // println!("A random mutation occurred!")
        }

        // Check for mover
        if organism.cells.iter().any(|cell| matches!(cell.cell_type, CellType::Mover)) {
            // Check for Eye
            if organism.cells.iter().any(|cell| matches!(cell.cell_type, CellType::Eye(_))) {
                // organism.move_based_on_vision(&World::new(128, 128, 128));
            }
            // If it doesn't have an Eye cell, move randomly
            else {
                organism.teleport_random();
            }
        }

        // Eats one food block if adjacent to one
        organism.eat(&mut *blocks);

        if organism.lifespan > 0 {
            organism.lifespan -= 1;
        }
        if organism.energy > 0 {
            organism.energy -= 2;
        }
        if organism.is_dead() {
            println!("Organism died");
            for val in organism.kill() {
                if blocks.len() < max_blocks {
                    blocks.push(val);       // Add the dead organism's cells as food blocks
                }
            }
        }
    }
    organisms.retain(|organism| { // Remove dead organisms
        let result = !organism.is_dead();
        if !result { // println!("Removing dead organism");
        } result
    });

    // println!("Number of organisms: {}", organisms.len());
    // println!("Number of blocks: {}", blocks.len());
}
// main

fn main() {
    let (tx, rx) = channel();

    let mut sim_world = World::new(128, 128, 128);

    let organisms = Arc::new(Mutex::new(vec![Organism::new()])); // Create a vec with one new organism
    let blocks = Arc::new(Mutex::new(vec![]));

    let organisms_clone = Arc::clone(&organisms);
    let blocks_clone = Arc::clone(&blocks);

    let max_organisms = 50;
    let max_blocks = 50;

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(200)); // Sleep

            let mut new_organisms = Vec::new();
            let mut new_blocks = Vec::new();

            update_world(&mut *organisms_clone.lock().unwrap(), &mut new_organisms, &mut *blocks_clone.lock().unwrap(), max_organisms, max_blocks, &mut sim_world);

            tx.send((new_organisms, new_blocks)).unwrap();
        }
    });

    let mut window = Window::new("Main Window");
    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));
    let mut camera = FirstPerson::new_with_frustrum(90.0, 0.1, 200.0, Point3::new(0.0, 0.0, 0.0).into(), Point3::new(0.0, 0.0, 0.0).into());
    camera.set_move_step(5.0);
    camera.set_pitch_step(0.01);
    camera.set_yaw_step(0.01);
    //move_dir takes up bool, down, left, right
    camera.rebind_up_key(Some(kiss3d::event::Key::W));
    let x = camera.up_key();
    println!("Camera up key: {:?}", x);
    let mut parent_objects = Vec::new();
    let mut last_instant = Instant::now(); // for fps calculation
    let fps_renderer = TextRenderer::new();

    let mut frame_counter = 0;
    while window.render() {                                                     // For each frame
        frame_counter += 1;
        for mut parent in parent_objects.drain(..) { // delete old render objects
            window.remove_node(&mut parent);
        }
        

        if let Ok((new_organisms, new_blocks)) = rx.try_recv() {
            organisms.lock().unwrap().append(&mut new_organisms.clone());
            blocks.lock().unwrap().append(&mut new_blocks.clone());
        }

        let mut new_organisms: Vec<kiss3d::event::Key> = Vec::new();
        let num_organisms = organisms.lock().unwrap().len();

        for organism in &mut *organisms.lock().unwrap() {
            let mut parent = window.add_group();
            for cell in &organism.cells { // render cells
                let mut cube = parent.add_cube(1.0, 1.0, 1.0);
                match cell.cell_type {
                    CellType::Brain(_) => cube.set_color(0.9, 0.4, 0.4),
                    CellType::Eye(_) => {
                        cube.set_color(1.0, 1.0, 1.0);
                        // a is the point of the eye: organism.x + cell.local_x, organism.y + cell.local_y, organism.z + cell.local_z
                        let a = Point3::new((organism.x + cell.local_x) as f32, (organism.y + cell.local_y) as f32, (organism.z + cell.local_z) as f32);
                        let offset = match cell.rotation {
                            0 => Vector3::new(1.0, 0.0, 0.0), // x
                            1 => Vector3::new(-1.0, 0.0, 0.0), // -x
                            2 => Vector3::new(0.0, 1.0, 0.0), // y
                            3 => Vector3::new(0.0, -1.0, 0.0), // -y
                            4 => Vector3::new(0.0, 0.0, 1.0), // z
                            _ => Vector3::new(0.0, 0.0, -1.0), // -z
                        };
                        let b = a + offset;
                        window.draw_line(&a, &b, &Point3::new(1.0, 1.0, 1.0));
                    },
                    CellType::Armor => cube.set_color(1.0, 1.0, 0.0),
                    CellType::Killer => cube.set_color(1.0, 0.0, 0.0),
                    CellType::Eater => cube.set_color(0.8, 0.8, 0.0),
                    CellType::Mover => cube.set_color(0.0, 0.0, 1.0),
                    CellType::Producer(_) => cube.set_color(0.0, 1.0, 0.0),
                };
                cube.append_translation(&Translation3::new((organism.x + cell.local_x) as f32, (organism.y + cell.local_y) as f32, (organism.z + cell.local_z) as f32));
            }
            for block in &*blocks.lock().unwrap() {
                let mut cube = parent.add_cube(1.0, 1.0, 1.0);
                match block.block_type {
                    BlockType::Food => cube.set_color(0.0, 0.5, 0.5),
                    BlockType::Wall => cube.set_color(0.2, 0.2, 0.2),
                };
                cube.append_translation(&Translation3::new(block.x as f32, block.y as f32, block.z as f32));
            }
            parent_objects.push(parent);
        }

        let now = Instant::now();
        let framerate = 1.0 / (now.duration_since(last_instant)).as_secs_f32();
        //                                                                              println!("Current framerate: {}", framerate);
        last_instant = now;
    }
}