/* Terminal based implementation of John Conway's Game of Life 
*  (c)2020 James A Wright, see LICENSE file.
*/

extern crate rand;
extern crate termion;

use std::{thread, time};
use std::io::{Write, stdout};
use rand::{thread_rng, Rng};
use termion::{clear,color,cursor,style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/*****************************************************************************/

const SLEEP_MILLIS: u64     = 100;
const EMPTY_CELL: 	char 	= ' ';
const LIVE_CELL: 	char 	= 'O';

/*****************************************************************************/

// Return next cell state
fn cell_update(cell: bool, neighbours: u8) -> bool {
    (neighbours == 3) || ((neighbours == 2) && cell)
}

// Return wrapped index making sure it's within range [0..len)
fn grid_index_wrap(i: isize, len: usize) -> usize {
    (i + len as isize) as usize % len
}

// Return number of neighbours for given cell position
fn grid_neighbours(grid: &[Vec<bool>], x: usize, y: usize) -> u8 {
    let left   = grid_index_wrap(x as isize - 1, grid[0].len());  // wrap to row width
    let right  = grid_index_wrap(x as isize + 1, grid[0].len());  // wrap to row width
    let top    = grid_index_wrap(y as isize - 1, grid.len());     // wrap to grid height
    let bottom = grid_index_wrap(y as isize + 1, grid.len());     // wrap to grid height

    (grid[top][left]     as u8) + 
    (grid[top][x]        as u8) + 
    (grid[top][right]    as u8) +
    (grid[y][left]       as u8) + 
    (grid[y][right]      as u8) +
    (grid[bottom][left]  as u8) + 
    (grid[bottom][x]     as u8) + 
    (grid[bottom][right] as u8)
}

// Return next row of state states
fn grid_update_row(grid: &[Vec<bool>], y: usize) -> Vec<bool> {
    grid[y].iter().enumerate()
        .map(|(x, cell)| cell_update(*cell, grid_neighbours(grid, x, y)))
        .collect()
}

// Return next grid state
fn grid_update(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    (0..grid.len())
        .map(|y| grid_update_row(grid, y))
        .collect()
}

// Render grid state to a String
fn grid_to_string(grid: &[Vec<bool>]) -> String {
    grid.iter()
        .map(|row| row.iter().map(|x| if *x { LIVE_CELL } else { EMPTY_CELL } ).collect::<String>() )
        .collect::<Vec<String>>()
        .join("\r\n")
}

// Generate a grid of randomsied cell states (bools)
fn grid_randomise(width: u16, height: u16) -> Vec<Vec<bool>> {
    let mut rng = thread_rng();			// store rng locally for performance
    (0..height)
        .map(|_| (0..width).map(|_| rng.gen::<bool>()).collect())
        .collect()
}

/*****************************************************************************/

struct Terminal <R, W> {
    input:  R,
    output: W,
    size: (u16, u16)
}

impl<R, W: Write> Terminal<R, W> {
    fn init(&mut self){
        write!(self.output, "{}{}{}", style::Reset, clear::All, cursor::Hide).unwrap();
    }

    fn restore(&mut self) {
        write!(self.output, "{}{}{}{}", style::Reset, clear::All, cursor::Show, cursor::Goto(1, 1)).unwrap();
    }

    fn write(&mut self, text: &str) {
        write!(self.output, "{}", text).unwrap();
    }

    fn flush(&mut self) {
        self.output.flush().unwrap();
    }
}

/*****************************************************************************/

fn main() {    
    let sleep_duration = time::Duration::from_millis(SLEEP_MILLIS);

    // Initialise terminal
    let mut terminal = Terminal {
        input:  termion::async_stdin().keys(),
        output: stdout().into_raw_mode().unwrap(),
        size:   termion::terminal_size().unwrap()
    };

    terminal.init();
    terminal.flush();

    // Initialise grid with randomised cell states
    let mut grid: Vec<Vec<bool>> = grid_randomise(terminal.size.0, terminal.size.1);

    // Main loop
    loop {

        // Check input for key events
        let input = terminal.input.next();
        if let Some(Ok(key)) = input {
            match key {
                Key::Esc => break,
                Key::Char('q') => break,
                Key::Char('r') => { grid = grid_randomise(terminal.size.0, terminal.size.1) },
                _ => {}
            }
        }

        // Check if terminal size has changed and regenerate grid
        let term_size = termion::terminal_size().unwrap();
        if term_size != terminal.size {
            terminal.size = term_size;
            grid = grid_randomise(terminal.size.0, terminal.size.1);
        }

        // Update next grid state
        grid = grid_update(&grid);            

        // Render to terminal
        terminal.write(&format!("{}{}{}{}{}", 
            cursor::Goto(1, 1), style::Bold, color::Fg(color::Green), 
            grid_to_string(&grid), 
            style::Reset));

        // Flush terminal output
        terminal.flush();

        // Sleep
        thread::sleep(sleep_duration);
    }

    // Restore terminal and flush output
    terminal.restore();
    terminal.flush();
}

/*****************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_update_should_return_correct_next_state() {
        // cell stays alive if 2 or 3 neighbours
        assert_eq!(cell_update(true, 0), false);
        assert_eq!(cell_update(true, 1), false);
        assert_eq!(cell_update(true, 2), true);
        assert_eq!(cell_update(true, 3), true);
        assert_eq!(cell_update(true, 4), false);
        assert_eq!(cell_update(true, 5), false);
        assert_eq!(cell_update(true, 6), false);
        assert_eq!(cell_update(true, 7), false);
        assert_eq!(cell_update(true, 8), false);

        // cell stays dead unless 3 neighbours
        assert_eq!(cell_update(false, 0), false);       
        assert_eq!(cell_update(false, 1), false);
        assert_eq!(cell_update(false, 2), false);
        assert_eq!(cell_update(false, 3), true);
        assert_eq!(cell_update(false, 4), false);
        assert_eq!(cell_update(false, 5), false);
        assert_eq!(cell_update(false, 6), false);
        assert_eq!(cell_update(false, 7), false);
        assert_eq!(cell_update(false, 8), false);
    }

    #[test]
    fn grid_index_wrap_should_return_correct_indices() {
        assert_eq!(grid_index_wrap(-3, 3), 0);
        assert_eq!(grid_index_wrap(-2, 3), 1);
        assert_eq!(grid_index_wrap(-1, 3), 2);
        assert_eq!(grid_index_wrap( 0, 3), 0);
        assert_eq!(grid_index_wrap( 1, 3), 1);
        assert_eq!(grid_index_wrap( 2, 3), 2);
        assert_eq!(grid_index_wrap( 3, 3), 0);
    }

    #[test]
    fn grid_randomise_should_generate_grid_with_correct_dimensions() {
        // given
        let width:  u16 = 7;
        let height: u16 = 5;

        // when
        let grid: Vec<Vec<bool>> = grid_randomise(width, height);
        
        // then
        assert_eq!(grid.len(), height as usize);
        assert_eq!(grid.iter().any(|row| row.len() != width as usize), false);
    }
}

/*****************************************************************************/
