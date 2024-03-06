// main

use rand::Rng;

use kiss3d::nalgebra::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

mod organism;
mod cell;
mod world;
use organism::*;
use cell::*;
use world::*;
use world::*;

/*
fn main() {

    let new_world = World::new(10, 10, 10);
    
    let mut window = Window::new("Kiss3d: cube");
    let mut c      = window.add_cube(1.0, 1.0, 1.0);

    c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        c.prepend_to_local_rotation(&rot);
    }
}
*/

fn main() {
    let world_timer : u32 = 0;
    let mut world = World::new(20, 20, 20);
    let mut organisms = vec![Organism::new()];

    organisms.push(Organism::new());

    let mut window = Window::new("Kiss3d Window");
    window.set_light(Light::StickToCamera);

    while window.render() { // main loop
        println!("Rendering window");
        
        // delete existing cubes
        // window.remove_node(all);

        for organism in &mut organisms {
            organism.teleport_random();
            let mut x = window.add_cube(organism.x as f32, organism.y as f32, organism.z as f32);
            x.set_color(1.0, 0.0, 0.0);
            println!("Organism at: {}, {}, {}", organism.x, organism.y, organism.z);
        }

        println!("finished rendering");
    }
}