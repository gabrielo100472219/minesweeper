#[derive(Clone)]
pub struct Cell {
    is_bomb: bool,
    is_flagged: bool,
    is_open: bool,
    adjacent_bombs: i8,
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
