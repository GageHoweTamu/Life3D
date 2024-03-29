// blocks not attached to any organism

#[derive(PartialEq)]
#[derive(Clone)]
pub enum BlockType {
    Food,
    Wall,
}

#[derive(PartialEq)]
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