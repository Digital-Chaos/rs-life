/* lib.rs - John Conway's Game of Life Cellular Automata
*  (c)2020 James Wright, see LICENSE file.
*/

extern crate rand;

use std::fmt;
use std::marker::Sync;
use rand::Rng;
use rayon::prelude::*;

/*****************************************************************************/

#[derive(Debug, Eq, PartialEq)]
pub struct Cell {
    alive:  bool
}

impl Cell {
    const EMPTY_CELL:   char = ' ';
    const LIVE_CELL:    char = 'O';

    // Return next cell state
    pub fn next(&self, neighbours: u8) -> Cell {
        Cell { alive: (neighbours == 3) || ((neighbours == 2) && self.alive) }
    }

    // Map cell state to a char
    pub fn to_char(&self) -> char {
        if self.alive { Cell::LIVE_CELL } else { Cell::EMPTY_CELL }
    }
}

/*****************************************************************************/

pub struct Grid {
    cells:  Vec<Vec<Cell>>
}

impl Grid {
    // Return a Grid of randomised cell states
    pub fn random<R: Rng>(mut rng: R, width: usize, height: usize) -> Grid {
        Grid {
            cells:  (0..height)
                    .map(|_| (0..width).map(|_| Cell { alive: (rng.gen::<u32>() & 1) != 0 }).collect())
                    .collect()
        }
    }

    // Return next Grid state
    pub fn next<F>(&self, cell_func: F) -> Grid
    where F: Fn(&Cell, u8)->Cell + Sync {
        Grid {
            cells:  self.cells.par_iter().enumerate()
                    .map(|(y, row)| row.iter().enumerate()
                        .map(|(x, cell)| cell_func(&cell, self.neighbours(x, y)))
                        .collect())
                    .collect()
        }
    }

    // Return number of neighbours for given cell position
    fn neighbours(&self, x: usize, y: usize) -> u8 {
        let cells  = &self.cells;
        let height = cells.len();
        let width  = cells[0].len();

        let left   = if x > 0 { x - 1 } else { width - 1 };
        let right  = if x < width - 1 { x + 1 } else { 0 };
        let top    = if y > 0 { y - 1 } else { height - 1 };
        let bottom = if y < height - 1 { y + 1 } else { 0 };

        (cells[top][left].alive    as u8) + (cells[top][x].alive    as u8) + (cells[top][right].alive    as u8) +
        (cells[y][left].alive      as u8) +                                  (cells[y][right].alive      as u8) +
        (cells[bottom][left].alive as u8) + (cells[bottom][x].alive as u8) + (cells[bottom][right].alive as u8)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", 
            self.cells.iter()
            .map(|row| row.iter().map(Cell::to_char).collect::<String>() )
            .collect::<Vec<String>>()
            .join("\r\n"))
    }
}

/*****************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    const CELL_DEAD:    Cell = Cell { alive: false };
    const CELL_ALIVE:   Cell = Cell { alive: true  };

    #[test]
    fn random_should_generate_grid_with_radomnly_populated_cells() {
        // given
        let width:  usize = 5;
        let height: usize = 4;
        let rng = StepRng::new(0, 1);

        // when
        let grid = Grid::random(rng, width, height);
        
        // then
        assert_eq!(grid.cells.len(), height, "Grid should have correct # of rows");
        assert_eq!(grid.cells[0].len(), width, "Grid should have correct # of cells in a row");
        assert_eq!(grid.cells.iter().any(|row| row.len() != width), false);
        assert_eq!(grid.cells.iter()
            .map(|row| row.iter().filter(|&cell| cell.alive == true).count())
            .sum::<usize>(), 
            (width * height) / 2, "Half of grid cells should be alive");
    }


    #[test]
    fn next_should_return_next_grid_given_cell_inversion_function() {
        // given
        fn cell_function(cell: &Cell, _neighbours: u8) -> Cell  { Cell { alive: !cell.alive } };
        let grid = Grid {
            cells:  vec!(vec!(CELL_DEAD, CELL_DEAD,  CELL_DEAD),
                         vec!(CELL_DEAD, CELL_ALIVE, CELL_DEAD),
                         vec!(CELL_DEAD, CELL_DEAD,  CELL_DEAD))
        };

        // when
        let next_grid = grid.next(cell_function); 

        // then
        assert_eq!(next_grid.cells.len(),       grid.cells.len());
        assert_eq!(next_grid.cells[0].len(),    grid.cells[0].len());
        assert_eq!(next_grid.cells,             vec!(vec!(CELL_ALIVE, CELL_ALIVE,  CELL_ALIVE),
                                                     vec!(CELL_ALIVE, CELL_DEAD,   CELL_ALIVE),
                                                     vec!(CELL_ALIVE, CELL_ALIVE,  CELL_ALIVE)));
    }

    #[test]
    fn next_should_return_next_grid_given_cell_neighbour_function() {
        // given
        fn cell_function(_cell: &Cell, neighbours: u8) -> Cell  { Cell { alive: neighbours == 8 } };
        let grid = Grid {
            cells:  vec!(vec!(CELL_ALIVE, CELL_ALIVE,  CELL_ALIVE),
                         vec!(CELL_ALIVE, CELL_DEAD,   CELL_ALIVE),
                         vec!(CELL_ALIVE, CELL_ALIVE,  CELL_ALIVE))
        };

        // when
        let next_grid = grid.next(cell_function); 

        // then
        assert_eq!(next_grid.cells.len(),       grid.cells.len());
        assert_eq!(next_grid.cells[0].len(),    grid.cells[0].len());
        assert_eq!(next_grid.cells,             vec!(vec!(CELL_DEAD, CELL_DEAD,  CELL_DEAD),
                                                     vec!(CELL_DEAD, CELL_ALIVE, CELL_DEAD),
                                                     vec!(CELL_DEAD, CELL_DEAD,  CELL_DEAD)));
    }

    #[test]
    fn fmt_should_format_grid_as_string() {
        // given
        let grid = Grid { 
            cells:  vec!(vec!(CELL_ALIVE, CELL_ALIVE,  CELL_ALIVE),
                         vec!(CELL_ALIVE, CELL_DEAD,   CELL_ALIVE),
                         vec!(CELL_ALIVE, CELL_ALIVE,  CELL_ALIVE))
        };

        // when
        let formatted = format!("{}", grid); 

        // then
        assert_eq!(formatted, "OOO\r\nO O\r\nOOO");
    }

/*****************************************************************************/

    #[test]
    fn to_char_should_return_correct_chars() {
        assert_eq!(Cell { alive: false }.to_char(), ' ');
        assert_eq!(Cell { alive: true }.to_char(),  'O');
    }

    #[test]
    fn next_should_return_correct_next_state() {
        // cell stays alive if 2 or 3 neighbours
        assert_eq!(Cell::next(&CELL_ALIVE, 0).alive, false);
        assert_eq!(Cell::next(&CELL_ALIVE, 1).alive, false);
        assert_eq!(Cell::next(&CELL_ALIVE, 2).alive, true);
        assert_eq!(Cell::next(&CELL_ALIVE, 3).alive, true);
        assert_eq!(Cell::next(&CELL_ALIVE, 4).alive, false);
        assert_eq!(Cell::next(&CELL_ALIVE, 5).alive, false);
        assert_eq!(Cell::next(&CELL_ALIVE, 6).alive, false);
        assert_eq!(Cell::next(&CELL_ALIVE, 7).alive, false);
        assert_eq!(Cell::next(&CELL_ALIVE, 8).alive, false);

        // cell stays dead unless 3 neighbours
        assert_eq!(Cell::next(&CELL_DEAD, 0).alive, false);
        assert_eq!(Cell::next(&CELL_DEAD, 1).alive, false);
        assert_eq!(Cell::next(&CELL_DEAD, 2).alive, false);
        assert_eq!(Cell::next(&CELL_DEAD, 3).alive, true);
        assert_eq!(Cell::next(&CELL_DEAD, 4).alive, false);
        assert_eq!(Cell::next(&CELL_DEAD, 5).alive, false);
        assert_eq!(Cell::next(&CELL_DEAD, 6).alive, false);
        assert_eq!(Cell::next(&CELL_DEAD, 7).alive, false);
        assert_eq!(Cell::next(&CELL_DEAD, 8).alive, false);
    }
}

/*****************************************************************************/
