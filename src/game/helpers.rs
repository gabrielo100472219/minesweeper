use super::Position;

pub fn get_adjacent_positions(position: Position, max_width: usize, max_height: usize) -> Vec<Position> {
    let mut adjacents: Vec<Position> = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = position.x + dx;
            let ny = position.y + dy;
            if nx >= 0 && nx < max_width as i8 && ny >= 0 && ny < max_height as i8 {
                adjacents.push(Position::new(nx, ny));
            }
        }
    }

    adjacents
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_cell_has_8_neighbors() {
        let pos = Position::new(1, 1);
        let neighbors = get_adjacent_positions(pos, 3, 3);
        assert_eq!(neighbors.len(), 8);
    }

    #[test]
    fn corner_has_3_neighbors() {
        let pos = Position::new(0, 0);
        let neighbors = get_adjacent_positions(pos, 3, 3);
        assert_eq!(neighbors.len(), 3);
    }

    #[test]
    fn edge_has_5_neighbors() {
        let pos = Position::new(0, 1); // left edge, center
        let neighbors = get_adjacent_positions(pos, 3, 3);
        assert_eq!(neighbors.len(), 5);
    }

    #[test]
    fn out_of_bounds_filtering_works() {
        let pos = Position::new(0, 0);
        let neighbors = get_adjacent_positions(pos, 1, 1);
        assert_eq!(neighbors.len(), 0);
    }
}

