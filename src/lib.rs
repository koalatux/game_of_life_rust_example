type Cells = Vec<bool>;
type Grid = Vec<Cells>;

#[derive(PartialEq)]
struct Point { x: i32, y: i32 }

impl Point {
    fn is_in(&self, grid: &Grid) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < grid[0].len() as _ && self.y < grid.len() as _
    }

    fn get_state_in(&self, grid: &Grid) -> bool {
        grid[self.y as usize][self.x as usize]
    }

    fn get_neighbour_states_in(&self, grid: &Grid) -> Cells {
        let offsets = vec![-1, 0, 1];
        let mut neighbour_states = vec![];
        for y in &offsets {
            for x in &offsets {
                let neighbour = Point { x: self.x + x, y: self.y + y };
                if neighbour != *self && neighbour.is_in(grid) {
                    neighbour_states.push(neighbour.get_state_in(grid));
                }
            }
        }
        neighbour_states
    }

    fn get_next_state_in(&self, grid: &Grid) -> bool {
        get_next_state_from(
            self.get_state_in(grid),
            self.get_neighbour_states_in(grid)
                .iter()
                .filter(|&x| *x)
                .count() as _,
        )
    }
}

fn get_next_state_from(current_state: bool, alive_neighbours_count: u8) -> bool {
    alive_neighbours_count == 3 || (current_state && alive_neighbours_count == 2)
}

pub fn get_next(grid: &Grid) -> Grid {
    grid.iter().enumerate().map(
        |(y, row)| row.iter().enumerate().map(
            |(x, _)| Point { x: x as _, y: y as _ }.get_next_state_in(grid)
        ).collect()
    ).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_grid() -> Grid {
        vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
            vec![false, false, false],
        ]
    }

    #[test]
    fn test_point_is_in() {
        let grid = get_test_grid();
        assert!(Point { x: 0, y: 0 }.is_in(&grid));
        assert!(Point { x: 2, y: 3 }.is_in(&grid));
        assert!(!Point { x: -1, y: 0 }.is_in(&grid));
        assert!(!Point { x: 0, y: -1 }.is_in(&grid));
        assert!(!Point { x: 3, y: 0 }.is_in(&grid));
        assert!(!Point { x: 0, y: 4 }.is_in(&grid));
    }

    #[test]
    fn test_point_get_state_in() {
        let grid = get_test_grid();
        assert!(!Point { x: 0, y: 0 }.get_state_in(&grid));
        assert!(Point { x: 1, y: 0 }.get_state_in(&grid));
        assert!(!Point { x: 2, y: 0 }.get_state_in(&grid));
    }

    #[test]
    fn test_point_get_neighbour_states_in() {
        let grid = get_test_grid();
        let expected = vec![true, false, false];
        assert_eq!(expected, Point { x: 0, y: 0 }.get_neighbour_states_in(&grid));
        let expected = vec![true, true, false];
        assert_eq!(expected, Point { x: 2, y: 3 }.get_neighbour_states_in(&grid));
        let expected = vec![false, true, false, false, true, true, true, true];
        assert_eq!(expected, Point { x: 1, y: 1 }.get_neighbour_states_in(&grid));
    }

    #[test]
    fn test_point_get_next_state_in() {
        let grid = get_test_grid();
        assert!(!Point { x: 0, y: 0 }.get_next_state_in(&grid));
        assert!(!Point { x: 1, y: 0 }.get_next_state_in(&grid));
        assert!(!Point { x: 2, y: 0 }.get_next_state_in(&grid));
        assert!(Point { x: 0, y: 1 }.get_next_state_in(&grid));
        assert!(!Point { x: 1, y: 1 }.get_next_state_in(&grid));
        assert!(Point { x: 2, y: 1 }.get_next_state_in(&grid));
    }

    #[test]
    fn test_get_next_state_from() {
        assert!(!get_next_state_from(true, 0));
        assert!(!get_next_state_from(true, 1));
        assert!(get_next_state_from(true, 2));
        assert!(get_next_state_from(true, 3));
        assert!(!get_next_state_from(true, 4));
        assert!(!get_next_state_from(true, 5));
        assert!(!get_next_state_from(true, 6));
        assert!(!get_next_state_from(true, 7));
        assert!(!get_next_state_from(true, 8));
        assert!(!get_next_state_from(false, 0));
        assert!(!get_next_state_from(false, 1));
        assert!(!get_next_state_from(false, 2));
        assert!(get_next_state_from(false, 3));
        assert!(!get_next_state_from(false, 4));
        assert!(!get_next_state_from(false, 5));
        assert!(!get_next_state_from(false, 6));
        assert!(!get_next_state_from(false, 7));
        assert!(!get_next_state_from(false, 8));
    }

    #[test]
    fn test_get_next() {
        let grid = get_test_grid();
        let expected = vec![
            vec![false, false, false],
            vec![true, false, true],
            vec![false, true, true],
            vec![false, true, false],
        ];
        assert_eq!(expected, get_next(&grid));
    }
}
