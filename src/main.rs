// main

use std::time::{Instant};
use kiss3d::scene::SceneNode;
use rand::Rng;
use kiss3d::nalgebra::{Point, Point2, Point3, Translation3, UnitQuaternion, Vector3};
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

/*
TODO:
- [x] Fix issue where organisms eat food without being adjacent to it
- [ ] Add text rendering for fps, total organisms, etc.
- [x] implement organism.rotate()
- [ ] make camera rotate around the simulation
*/

// 1 in x chances
static CHANCE_OF_REPRODUCTION: i8 = 30;              // how likely an organism is to reproduce
static CHANCE_OF_MUTATION: i8 = 100;                 // random mutation apart from reproduction
static CHANCE_OF_FOOD_PRODUCTION: i8 = 30;           // chance of a producer cell producing food

static MAX_ORGANISMS: usize = 100;                  // soft limit; multiple organisms can be created in a single tick
static MAX_BLOCKS: usize = 100;                     // this can by bypassed when an organism dies

fn update_world(organisms: &mut Vec<Organism>, new_organisms: &mut Vec<Organism>, blocks: &mut Vec<Block>, max_organisms: usize, max_blocks: usize, sim_world: &mut World) {
    let organisms_len = organisms.len();    

    for organism in organisms.iter_mut() {

        // reproduce
        if rand::thread_rng().gen_range(0..CHANCE_OF_REPRODUCTION) == 0 {
            if organisms_len < max_organisms {
                let mut new_organism = organism.reproduce();
                if rand::thread_rng().gen_range(0..2) == 0 {
                    new_organism.mutate(); // reproduced organisms have a 50% chance of mutation
                }
                new_organisms.push(new_organism);
            }
        }
        // produce food
        if rand::thread_rng().gen_range(0..CHANCE_OF_FOOD_PRODUCTION) == 0 {
            if organism.cells.iter().any(|cell| matches!(cell.cell_type, CellType::Producer(_))) {
                if max_blocks > blocks.len() {
                    if let Some(block) = organism.produce_food() {
                        blocks.push(block);
                    }
                }
            }
        }
        // random mutation
        if rand::thread_rng().gen_range(0..CHANCE_OF_MUTATION) == 0 {
            organism.mutate();
        }

        // Eats one food block if adjacent to one and has an eater cell
        if organism.cells.iter().any(|cell| matches!(cell.cell_type, CellType::Eater)) {
            organism.eat(&mut *blocks);
        }

        // Housekeeping
        if organism.lifespan > 0 {
            organism.lifespan -= 1;
        }
        if organism.energy > 0 {
            organism.energy -= 2;
        }
        if organism.is_dead() {
            // println!("Organism died");
            for val in organism.kill() {
                if blocks.len() < max_blocks {
                    blocks.push(val);       // Add the dead organism's cells as food blocks
                }
            }
        }
    }

    let organisms_clone = &(organisms.clone()); // avoids borrowing issues; maybe there's a better way though
            // damage nearby organisms if there are killer cells
    let mut to_damage = Vec::new();
    for (i, organism) in organisms.iter_mut().enumerate() {
        if organism.cells.iter().any(|cell| matches!(cell.cell_type, CellType::Killer)) {
            to_damage.push(i);
        }
    }
    for i in to_damage {
        organisms_clone[i].damage_nearby_organisms(organisms);
    }
    let mut to_move_better = Vec::new();
    for (i, organism) in organisms.iter_mut().enumerate() {
        if organism.cells.iter().any(|cell| matches!(cell.cell_type, CellType::Mover)) {
            if organism.cells.iter().any(|cell| matches!(cell.cell_type, CellType::Eye(_))) {
                to_move_better.push(i);
            }
            else { organism.teleport_random(); }
        }
    }
    for i in to_move_better {
        organisms[i].move_better(organisms_clone, blocks);
    }
    organisms.retain(|organism| { // Remove dead organisms
        let result = !organism.is_dead();
        if !result { // println!("Removing dead organism");
        } result
    });

    // println!("Number of organisms: {}", organisms.len());
    // println!("Number of blocks: {}", blocks.len());
}

fn main() {
    let (tx, rx) = channel();

    let mut sim_world = World::new(128, 128, 128);

    let organisms = Arc::new(Mutex::new(vec![Organism::new()])); // Create a vec with one new organism
    let blocks = Arc::new(Mutex::new(vec![]));

    let organisms_clone = Arc::clone(&organisms);
    let blocks_clone = Arc::clone(&blocks);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(200)); // Sleep

            let mut new_organisms = Vec::new();
            let mut new_blocks = Vec::new();

            update_world(&mut *organisms_clone.lock().unwrap(), &mut new_organisms, &mut *blocks_clone.lock().unwrap(), MAX_ORGANISMS, MAX_BLOCKS, &mut sim_world);

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


        let num_organisms = organisms.lock().unwrap().len();

        for organism in &mut *organisms.lock().unwrap() {
            let mut parent = window.add_group();
            for cell in &organism.cells { // render cells
                let mut cube = parent.add_cube(1.0, 1.0, 1.0);
                match cell.cell_type {
                    CellType::Brain(_) => cube.set_color(0.9, 0.2, 0.4),
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
                    CellType::Killer => cube.set_color(1.0, 0.0, 0.0),      // killers are red
                    CellType::Eater => cube.set_color(1.0, 0.0, 1.0),       // eaters are purple
                    CellType::Mover => cube.set_color(0.0, 0.0, 1.0),       // movers are blue
                    CellType::Producer(_) => cube.set_color(0.0, 1.0, 0.0), // producers are green
                                                                            // eyes are white
                };
                cube.append_translation(&Translation3::new((organism.x + cell.local_x) as f32, (organism.y + cell.local_y) as f32, (organism.z + cell.local_z) as f32));
            }
            for block in &*blocks.lock().unwrap() {
                let mut cube = parent.add_cube(1.0, 1.0, 1.0);
                match block.block_type {
                    BlockType::Food => cube.set_color(0.2, 0.3, 0.3),
                    BlockType::Wall => cube.set_color(0.6, 0.6, 0.6),
                };
                cube.append_translation(&Translation3::new(block.x as f32, block.y as f32, block.z as f32));
            }
            parent_objects.push(parent);
        }

        let mut fps_renderer = TextRenderer::new();

        // Draw text
        let font2 = kiss3d::text::Font::default();
        let color2 = Point3::new(1.0, 1.0, 1.0);
        let point2 = Point2::new(0.0, 0.0);
        fps_renderer.draw_text("hi!", &point2, 10.0, &font2, &color2);
        
        // Render the text
        fps_renderer.render(window.width() as f32, window.height() as f32);

        let now = Instant::now();
        let framerate = 1.0 / (now.duration_since(last_instant)).as_secs_f32();
        //                                                                              println!("Current framerate: {}", framerate);
        last_instant = now;
    }
}