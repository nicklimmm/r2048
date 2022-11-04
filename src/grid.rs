use rand::Rng;
use std::fmt;

pub const GRID_SIZE: usize = 4;

#[derive(Clone)]
pub struct Grid {
    pub cells: [[u32; GRID_SIZE]; GRID_SIZE],
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();

        let mut max_width = 1;
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let width = self.cells[i][j].to_string().len();
                if max_width < width {
                    max_width = width;
                }
            }
        }

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                str.push_str(&format!("{:max_width$} ", self.cells[i][j].to_string()));
            }
            str.push('\n');
        }

        write!(f, "{}", str)
    }
}

impl Grid {
    const INIT_FILLED_CELLS: u32 = 3;

    pub fn new() -> Self {
        let mut g = Grid {
            cells: [[0; GRID_SIZE]; GRID_SIZE],
        };

        for _ in 0..Self::INIT_FILLED_CELLS {
            g.insert_random_cell();
        }

        return g;
    }

    pub fn update(&mut self, action: Action) {
        // Use symmetry property
        if action.is_vertical() {
            self.transpose();
            self.update(action.transpose());
            self.transpose();
            return;
        }

        // Update each row
        let mut i = 0;
        for _ in 0..GRID_SIZE {
            self.update_row(i, &action);
            i += 1;
        }
    }

    fn update_row(&mut self, i: usize, action: &Action) {
        let mut anchor = Grid::starting_anchor(&action);
        let mut j = Grid::starting_j(&action);

        for _ in 0..(GRID_SIZE - 1) {
            let (anchor_val, j_val) = (self.cells[i][anchor], self.cells[i][j]);
            if anchor_val == 0 || j_val == 0 || anchor_val == j_val {
                // Merge values
                self.cells[i][anchor] += self.cells[i][j];
                self.cells[i][j] = 0;

                if anchor_val != 0 && anchor_val == j_val {
                    anchor = Grid::update_ptr(anchor, &action);
                }
            } else {
                anchor = Grid::update_ptr(anchor, &action);

                // Shift to empty values (new anchor)
                (self.cells[i][anchor], self.cells[i][j]) =
                    (self.cells[i][j], self.cells[i][anchor]);
            }

            j = Grid::update_ptr(j, &action);
        }
    }

    fn transpose(&mut self) -> &mut Self {
        for i in 0..GRID_SIZE {
            for j in (i + 1)..GRID_SIZE {
                (self.cells[i][j], self.cells[j][i]) = (self.cells[j][i], self.cells[i][j]);
            }
        }

        return self;
    }

    fn starting_anchor(action: &Action) -> usize {
        match action {
            Action::Left => 0,
            _ => GRID_SIZE - 1,
        }
    }

    fn starting_j(action: &Action) -> usize {
        let anchor = Grid::starting_anchor(action);
        match action {
            Action::Left => anchor + 1,
            _ => anchor - 1,
        }
    }

    fn update_ptr(ptr: usize, action: &Action) -> usize {
        match action {
            Action::Left => ptr.saturating_add(1),
            _ => ptr.saturating_sub(1),
        }
    }

    fn empty_cells(&self) -> Vec<(usize, usize)> {
        let mut available_cells = Vec::new();

        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.cells[i][j] == 0 {
                    available_cells.push((i, j));
                }
            }
        }

        return available_cells;
    }

    fn random_empty_cell(&self) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();
        let empty_cells = self.empty_cells();

        if empty_cells.len() == 0 {
            return None;
        }

        return Some(empty_cells[rng.gen_range(0..empty_cells.len())]);
    }

    pub fn insert_random_cell(&mut self) {
        let cell = self.random_empty_cell();
        match cell {
            Some((i, j)) => {
                self.cells[i][j] = 2;
            }
            None => (),
        };
    }

    pub fn game_over(&self) -> bool {
        /* Not over when:
           1. There exists an empty cell, or
           2. There are 2 adjacent non-zero cells that are equal to each other
        */

        // Find empty cell
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                if self.cells[i][j] == 0 {
                    return false;
                }
            }
        }

        // Horizontal check
        for i in 0..GRID_SIZE {
            for j in 1..GRID_SIZE {
                if self.cells[i][j] != 0 && self.cells[i][j] == self.cells[i][j - 1] {
                    return false;
                }
            }
        }

        // Vertical check
        for j in 0..GRID_SIZE {
            for i in 1..GRID_SIZE {
                if self.cells[i][j] != 0 && self.cells[i][j] == self.cells[i - 1][j] {
                    return false;
                }
            }
        }

        return true;
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn test_update() {
        /*
           Cases:
           - Merge
           - Skip through
           - No merge
           - No double merges
           - Merge ordering based on action
        */
        let test_cases = [
            (
                [
                    [0, 2, 0, 2], // Merge and skip through
                    [0, 2, 4, 0], // Not merge
                    [2, 2, 2, 2], // No double merges
                    [2, 2, 0, 2], // Merge ordering
                ],
                Action::Left,
                [[4, 0, 0, 0], [2, 4, 0, 0], [4, 4, 0, 0], [4, 2, 0, 0]],
            ),
            (
                [
                    [2, 0, 2, 0], // Merge and skip through
                    [0, 2, 4, 0], // Not merge
                    [2, 2, 2, 2], // No double merges
                    [2, 2, 0, 2], // Merge ordering
                ],
                Action::Right,
                [[0, 0, 0, 4], [0, 0, 2, 4], [0, 0, 4, 4], [0, 0, 2, 4]],
            ),
            (
                [
                    [0, 2, 2, 2], // Merge and skip through
                    [2, 0, 2, 0], // Not merge
                    [0, 4, 2, 2], // No double merges
                    [2, 0, 2, 2], // Merge ordering
                ],
                Action::Up,
                [[4, 2, 4, 4], [0, 4, 4, 2], [0, 0, 0, 0], [0, 0, 0, 0]],
            ),
            (
                [
                    [2, 0, 2, 2], // Merge and skip through
                    [0, 2, 2, 0], // Not merge
                    [2, 0, 2, 2], // No double merges
                    [0, 4, 2, 2], // Merge ordering
                ],
                Action::Down,
                [[0, 0, 0, 0], [0, 0, 0, 0], [0, 2, 4, 2], [4, 4, 4, 4]],
            ),
        ];

        for (cells, action, expected_val) in test_cases {
            let mut g = Grid::new();
            g.cells = cells;
            g.update(action);
            assert_eq!(g.cells, expected_val);
        }
    }

    #[test]
    fn test_game_over() {
        let test_cases = [
            (
                // Has empty cell
                [
                    [64, 2, 4, 8],
                    [2, 4, 0, 16],
                    [4, 8, 16, 32],
                    [8, 16, 32, 64],
                ],
                false,
            ),
            (
                // Merge possible (row-wise)
                [
                    [2, 2, 8, 16],
                    [4, 8, 16, 32],
                    [8, 16, 32, 64],
                    [16, 32, 64, 128],
                ],
                false,
            ),
            (
                // Merge possible (col-wise)
                [
                    [2, 4, 8, 16],
                    [2, 8, 16, 32],
                    [8, 16, 32, 64],
                    [16, 32, 64, 128],
                ],
                false,
            ),
            (
                // No moves left
                [
                    [64, 2, 4, 8],
                    [2, 4, 8, 16],
                    [4, 8, 16, 32],
                    [8, 16, 32, 64],
                ],
                true,
            ),
        ];

        for (cells, expected_val) in test_cases {
            let mut g = Grid::new();
            g.cells = cells;
            assert_eq!(g.game_over(), expected_val);
        }
    }

    #[test]
    fn test_transpose() {
        let mut g = Grid::new();
        g.cells = [
            [2, 4, 8, 16],
            [8, 16, 32, 64],
            [16, 64, 128, 256],
            [32, 128, 512, 1024],
        ];

        assert_eq!(
            g.transpose().cells,
            [
                [2, 8, 16, 32],
                [4, 16, 64, 128],
                [8, 32, 128, 512],
                [16, 64, 256, 1024],
            ]
        );

        // tranpose == inverse
        assert_eq!(g.clone().transpose().transpose().cells, g.cells);
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Up,
    Right,
    Down,
    Left,
}

impl Action {
    fn is_vertical(&self) -> bool {
        match self {
            Action::Up | Action::Down => true,
            _ => false,
        }
    }

    fn transpose(&self) -> Action {
        match self {
            Action::Up => Action::Left,
            Action::Right => Action::Down,
            Action::Down => Action::Right,
            Action::Left => Action::Up,
        }
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;

    #[test]
    fn test_is_vertical() {
        let test_cases = [
            (Action::Up, true),
            (Action::Down, true),
            (Action::Left, false),
            (Action::Right, false),
        ];

        for (action, expected_val) in test_cases {
            assert_eq!(action.is_vertical(), expected_val);
        }
    }

    #[test]
    fn test_transpose() {
        let test_cases = [
            (Action::Up, Action::Left),
            (Action::Down, Action::Right),
            (Action::Left, Action::Up),
            (Action::Right, Action::Down),
        ];

        for (action, expected_val) in test_cases {
            assert_eq!(action.transpose(), expected_val);

            // transpose == inverse
            assert_eq!(action.transpose().transpose(), action);
        }
    }
}
