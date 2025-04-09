#[derive(Clone)]
pub struct Cell {
    pub is_bomb: bool,
    pub is_flagged: bool,
    pub is_open: bool,
    pub adjacent_bombs: i8,
}

impl Cell {
    pub fn new() -> Self{
        Self {
            is_bomb: false,
            is_flagged: false,
            is_open: false,
            adjacent_bombs: 0,
        }
    }
}
