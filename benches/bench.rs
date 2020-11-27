/* bench.rs - Benchmark suite
*  (c)2020 James Wright, see LICENSE file.
*/

#[macro_use]
extern crate bencher;

use bencher::Bencher;
use rand::thread_rng;
use rs_life::{Cell,Grid};

/*****************************************************************************/

fn grid_next_160_x_120(bench: &mut Bencher) {
    let rng = thread_rng();
    let grid = Grid::random(rng, 160, 120);
    bench.iter(|| { grid.next(Cell::next) })
}

fn grid_next_320_x_240(bench: &mut Bencher) {
    let rng = thread_rng();
    let grid = Grid::random(rng, 320, 240);
    bench.iter(|| { grid.next(Cell::next) })
}

fn grid_next_640_x_480(bench: &mut Bencher) {
    let rng = thread_rng();
    let grid = Grid::random(rng, 640, 480);
    bench.iter(|| { grid.next(Cell::next) })
}

fn grid_random(bench: &mut Bencher) {
    let rng = thread_rng();
    bench.iter(|| { Grid::random(rng, 320, 240) })
}

/*****************************************************************************/

benchmark_group!(benches, 
    grid_next_160_x_120,
    grid_next_320_x_240,
    grid_next_640_x_480,
    grid_random,
);
benchmark_main!(benches);

/*****************************************************************************/
