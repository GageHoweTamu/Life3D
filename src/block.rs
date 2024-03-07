// blocks not attached to any organism

pub enum BlockType {
    Food,
    Wall,
}

pub struct Block {
    pub block_type: BlockType,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}