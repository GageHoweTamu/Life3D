// defines the cells the organisms are made of
use rand::Rng;

pub enum CellType {
    Eye,
    Armor,
    Damager,
    Eater,
    Producer,
}

pub struct Cell {
    cell_type: CellType,
    pub local_x: i8,
    pub local_y: i8,
    pub local_z: i8,
}

impl Cell {
    pub fn new(cell_type: CellType, local_x: i8, local_y: i8, local_z: i8) -> Cell {
        Cell {
            cell_type,
            local_x,
            local_y,
            local_z,
        }
    }
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let cell_type = match rng.gen_range(0..6) {
            1 => CellType::Eye,
            2 => CellType::Armor,
            3 => CellType::Damager,
            4 => CellType::Producer,
            _ => CellType::Eater,
        };
        self.cell_type = cell_type;
    }
    pub fn shift(&mut self, x: i8, y: i8, z: i8) {
        self.local_x += x;
        self.local_y += y;
        self.local_z += z;
    }
}