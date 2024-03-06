// defines the world and its components

pub enum VoxelType {
    Empty,
    Food,
    Poison,
    Wall,
    Occupied,
}

pub struct Voxel {
    voxel_type: VoxelType,
    warmth: f32,
}

pub struct World {
    cells: Vec<Vec<Vec<Voxel>>>,
    width: usize,
    height: usize,
    depth: usize,
}

impl World {
    pub fn new(width: usize, height: usize, depth: usize) -> World {
        let mut cells = Vec::with_capacity(width);
        for _ in 0..width {
            let mut row = Vec::with_capacity(height);
            for _ in 0..height {
                let mut column = Vec::with_capacity(depth);
                for _ in 0..depth {
                    column.push(Voxel {
                        voxel_type: VoxelType::Empty,
                        warmth: 0.0,
                    });
                }
                row.push(column);
            }
            cells.push(row);
        }
        World {
            cells,
            width,
            height,
            depth,
        }
    }
}
