// defines the world and its components
use crate::organism::Organism;
use crate::block::Block;
use crate::cell::Cell;
use crate::octrees::Octree;

#[derive(Clone)]
pub enum Entity {
    Block(Block),
    Cell(Cell),
}

struct World {
    width: usize,
    height: usize,
    depth: usize,
    octree: Octree,
}

impl World {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        World {
            width,
            height,
            depth,
            octree: Octree::new([width as f32 / 2.0, height as f32 / 2.0, depth as f32 / 2.0], 
                                 width.max(height).max(depth) as f32, 
                                 1.0),
        }
    }
}
