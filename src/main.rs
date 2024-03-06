// main

use kiss3d::scene::SceneNode;
use rand::Rng;

use kiss3d::nalgebra::{Translation, UnitQuaternion, Vector3, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::camera::FirstPerson;

mod organism;
mod cell;
mod world;
use organism::*;
use cell::*;
use world::*;
use world::*;

fn main() {
    let mut world_timer : u32 = 0;
    let mut world_vector = World::new(20, 20, 20);
    let mut organisms = vec![Organism::new()];
    organisms.push(Organism::new());

    let mut window = Window::new("Main Window");
    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));
    let mut camera = FirstPerson::new(Point3::new(0.0, 0.0, 0.0).into(), Point3::new(0.0, 0.0, 0.0).into());
    camera.set_move_step(5.0);
    camera.set_pitch_step(0.01);
    camera.set_yaw_step(0.01);

    let mut parent_objects = Vec::new();

    while window.render() { // main loop
        println!("Rendering window");

        for mut parents in parent_objects.drain(..) { // delete each organism's scene node
            window.remove_node(&mut parents);
        }

        // pub fn new(eye: Point3<f32>, at: Point3<f32>) -> FirstPerson
        
        // 

        for organism in &mut organisms {
            organism.teleport_random();
            // make a node for each organism
            
            // for each cell in the organism, make a cube (child node)




            println!("Organism at: {}, {}, {} with {} cells", organism.x, organism.y, organism.z, organism.cells.len());
        }

        world_timer += 1;
        println!("World age: {}", world_timer);
        println!("finished rendering");
    }
}

/*
fn main() {
    let world_timer : u32 = 0;
    let mut world = World::new(20, 20, 20);
    let mut organisms = vec![Organism::new()];
    organisms.push(Organism::new());

    let mut window = Window::new("Main Window");
    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));
    let mut camera = FirstPerson::new(Point3::new(0.0, 0.0, 0.0).into(), Point3::new(0.0, 0.0, 0.0).into());
    camera.set_move_step(5.0);
    camera.set_pitch_step(0.01);
    camera.set_yaw_step(0.01);

    // let mut cubes = Vec::new();

    while window.render() { // main loop
        println!("Rendering window");
        // pub fn new(eye: Point3<f32>, at: Point3<f32>) -> FirstPerson
        
        // delete existing cubes
        // window.remove_node(all);

        for organism in &mut organisms {
            organism.teleport_random();

            for cell in &organism.cells {
                let mut x = window.add_cube(1.0, 1.0, 1.0);
                x.set_color(1.0, 0.4, 0.4);

                println!("Cell at: {}, {}, {}", cell.local_x, cell.local_y, cell.local_z);
            }


            println!("Organism at: {}, {}, {}", organism.x, organism.y, organism.z);
        }

        println!("finished rendering");
    }
}
*/