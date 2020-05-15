
#[derive(Debug, Copy, Clone)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Cell {
        Cell { alive }
    }

    pub fn next_state(&self, neighbours: i8) -> bool {
        if self.alive && (neighbours < 2 || neighbours > 3) {
            return false;
        }

        if self.alive && (neighbours == 2 || neighbours == 3) {
            return true;
        }

        if !self.alive && neighbours == 3 {
            return true;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_cell() {
        assert_eq!(Cell::new(true).alive, Cell{alive: true}.alive);
    }

    #[test]
    fn alive_cell_next_state_with_zero_neighbours() {
        let cell = Cell::new(true);
        let alive = cell.next_state(0);
        assert_eq!(alive, false);
    }

    #[test]
    fn alive_cell_next_state_with_two_neighbours() {
        let cell = Cell::new(true);
        let alive = cell.next_state(2);
        assert_eq!(alive, true);
    }

    #[test]
    fn alive_cell_next_state_with_three_neighbours() {
        let cell = Cell::new(true);
        let alive = cell.next_state(3);
        assert_eq!(alive, true);
    }

    #[test]
    fn alive_cell_next_state_with_four_neighbours() {
        let cell = Cell::new(true);
        let alive = cell.next_state(4);
        assert_eq!(alive, false);
    }

    #[test]
    fn dead_cell_next_state_with_three_neighbours() {
        let cell = Cell::new(false);
        let alive = cell.next_state(3);
        assert_eq!(alive, true);
    }
}
