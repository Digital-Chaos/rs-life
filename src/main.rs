/* main.rs - Terminal based implementation of John Conway's Game of Life
*  (c)2020 James Wright, see LICENSE file.
*/

extern crate termion;
extern crate rand;

use rand::{thread_rng};
use std::{thread, time};
use std::io::{Write, stdout};
use termion::{clear,color,cursor,style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use rs_life::{Cell, Grid};

/*****************************************************************************/

const SLEEP_MILLIS: u64 = 100;

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

    // Create a PRNG
    let rng = thread_rng();

    // Initialise grid with randomised cell states
    let mut grid = Grid::random(rng, terminal.size.0 as usize, terminal.size.1 as usize);

    // Main loop
    loop {

        // Check input for key events
        let input = terminal.input.next();
        if let Some(Ok(key)) = input {
            match key {
                Key::Esc => break,
                Key::Char('q') => break,
                _ => {}
            }
        }

        // Check if terminal size has changed and regenerate grid
        let term_size = termion::terminal_size().unwrap();
        if term_size != terminal.size {
            terminal.size = term_size;
            grid = Grid::random(rng, terminal.size.0 as usize, terminal.size.1 as usize);
        }

        // Get the next Grid state using the Cell::next function
        grid = Grid::next(&grid, Cell::next);

        // Render to terminal
        terminal.write(&format!("{}{}{}{}{}", 
            cursor::Goto(1, 1), style::Bold, color::Fg(color::Green), grid, style::Reset));

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
