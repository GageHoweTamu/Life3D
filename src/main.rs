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

fn update_world(organisms: &mut Vec<Organism>, new_organisms: &mut Vec<Organism>, blocks: &mut Vec<Block>, max_organisms: usize, max_blocks: usize) {
    let organisms_len = organisms.len();
    for organism in organisms.iter_mut() {
        if rand::thread_rng().gen_range(0..100) == 0 { // 1% chance of reproduction
            if organisms_len < max_organisms {
                let mut new_organism = organism.reproduce();
                new_organism.mutate();                  // reproduced organisms have a 50% chance of mutation
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
        if rand::thread_rng().gen_range(0..100) == 0 { // 1% chance of random mutation
            organism.mutate();
            println!("A random mutation occurred!")
        }
        // if the organism has a mover cell, move it
        if organism.cells.iter().any(|cell| match cell.cell_type {
            CellType::Mover => true,
            _ => false,
        }) {
            organism.teleport_random();
        }
    }
}
// main

fn main() {
    let (tx, rx) = channel();

    let organisms = Arc::new(Mutex::new(vec![Organism::new(), Organism::new()]));
    let blocks = Arc::new(Mutex::new(vec![]));

    let organisms_clone = Arc::clone(&organisms);
    let blocks_clone = Arc::clone(&blocks);

    let max_organisms = 100;
    let max_blocks = 100;

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(200)); // Sleep

            let mut new_organisms = Vec::new();
            let mut new_blocks = Vec::new();

            update_world(&mut *organisms_clone.lock().unwrap(), &mut new_organisms, &mut *blocks_clone.lock().unwrap(), max_organisms, max_blocks);

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
    let mut world = World::new(128, 128, 128);
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
                    CellType::Brain(_) => cube.set_color(0.8, 0.5, 0.5),
                    CellType::Eye(_) => {
                        cube.set_color(1.0, 1.0, 1.0);
                        // render a line in the direction of the eye
                        // a is the point of the eye: organism.x + cell.local_x, organism.y + cell.local_y, organism.z + cell.local_z
                        let a = Point3::new((organism.x + cell.local_x) as f32, (organism.y + cell.local_y) as f32, (organism.z + cell.local_z) as f32);
                        let offset = match cell.rotation {
                            0 => Vector3::new(1.0, 0.0, 0.0),
                            1 => Vector3::new(-1.0, 0.0, 0.0),
                            2 => Vector3::new(0.0, 1.0, 0.0),
                            3 => Vector3::new(0.0, -1.0, 0.0),
                            4 => Vector3::new(0.0, 0.0, 1.0),
                            5 => Vector3::new(0.0, 0.0, -1.0),
                            _ => Vector3::new(0.0, 0.0, 0.0),
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
                    BlockType::Food => cube.set_color(0.5, 1.0, 0.0),
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