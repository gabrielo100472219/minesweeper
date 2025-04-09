use super::Position;

pub fn get_adjacent_positions(position: Position, max_width: usize, max_height: usize) -> Vec<Position> {
    let mut adjacents: Vec<Position> = Vec::new();
    for x in range(-1, 2){
        for y in range(-1, 2){
            if (x == 0 && y == 0) {
                continue;
            }
            let nx = position::x + x;
            let ny = position::y + y;
            if (0 <= nx < max_width && 0 <= ny < max_height) {
                adjacents.push(Position::new(nx, ny));
            }
        }
    }
}
