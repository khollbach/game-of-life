const SIM_WIDTH: usize = 100;
const DISPLAY_WIDTH: usize = 5;

/// A cell in a game of life, which is either dead or alive.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Dead,
    Alive,
}

/// A simulated game of life.
/// 
/// The simulation is finite, and the "viewable window" is even smaller.
pub struct Game {
    grid: [[Cell; SIM_WIDTH]; SIM_WIDTH],
}

impl Game {
    /// Create a new game, where all cells are initially dead.
    pub fn new() -> Game {
        Game {
            grid: [[Cell::Dead; SIM_WIDTH]; SIM_WIDTH],
        }
    }

    /// Create a new game that starts in the given state.
    pub fn from_grid(grid: [[u8; DISPLAY_WIDTH]; DISPLAY_WIDTH]) -> Game {
        let mut game = Game::new();

        for i in 0..DISPLAY_WIDTH {
            for j in 0..DISPLAY_WIDTH {
                let cell = if grid[i][j] != 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                };

                *game.get_display_coords_mut(i, j) = cell;
            }
        }

        game
    }

    /// Return the "viewable window" of the game's current state.
    pub fn display(&self) -> [[u8; DISPLAY_WIDTH]; DISPLAY_WIDTH] {
        let mut output = [[0; DISPLAY_WIDTH]; DISPLAY_WIDTH];

        for i in 0..DISPLAY_WIDTH {
            for j in 0..DISPLAY_WIDTH {
                let cell = self.get_display_coords(i, j);
                let byte = match cell {
                    Cell::Dead => 0,
                    Cell::Alive => 1,
                };

                output[i][j] = byte;
            }
        }

        output
    }

    /// NOTE: the displayed region is in the middle of the simulated region.
    fn get_display_coords(&self, display_row: usize, display_col: usize) -> Cell {
        let i = display_row + SIM_WIDTH / 2;
        let j = display_col + SIM_WIDTH / 2;
        self.grid[i][j]
    }

    /// NOTE: the displayed region is in the middle of the simulated region.
    fn get_display_coords_mut(&mut self, display_row: usize, display_col: usize) -> &mut Cell {
        let i = display_row + SIM_WIDTH / 2;
        let j = display_col + SIM_WIDTH / 2;
        &mut self.grid[i][j]
    }

    /// Transition the game to the next state.
    /// 
    /// See https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life for details.
    pub fn evolve(&mut self) {
        *self = self.transform();
    }

    fn transform(&self) -> Game {
        let mut output = Game::new();

        for i in 0..SIM_WIDTH {
            for j in 0..SIM_WIDTH {
                let n = neighbors(i as isize, j as isize)
                    .filter(|&(x, y)| self.grid[x as usize][y as usize] == Cell::Alive)
                    .count();

                let new_cell = match (self.grid[i][j], n) {
                    (Cell::Alive, 2 | 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                output.grid[i][j] = new_cell;
            }
        }

        output
    }
}

/// An iterator over the 8 neighbors of (i, j).
/// 
/// Checks bounds against the range `0..=SIM_WIDTH`, and will yield fewer than 8
/// neighbors if any of them would be out-of-bounds.
fn neighbors(i: isize, j: isize) -> impl Iterator<Item = (isize, isize)> {
    [-1, 0, 1].into_iter().flat_map(move |di| {
        [-1, 0, 1].into_iter().filter_map(move |dj| {
            let i2 = i + di;
            let j2 = j + dj;

            let n = SIM_WIDTH as isize;
            let in_bounds = 0 <= i2 && i2 < n && 0 <= j2 && j2 < n;

            if (di, dj) != (0, 0) && in_bounds {
                Some((i2, j2))
            } else {
                None
            }
        })
    })
}
