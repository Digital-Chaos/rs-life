/* bench.rs - Benchmark suite
*  (c)2020 James Wright, see LICENSE file.
*/

#[macro_use]
extern crate bencher;

use bencher::Bencher;
use rand::thread_rng;
use rs_life::{Cell,Grid};

/*****************************************************************************/

fn grid_random(bench: &mut Bencher) {
    let rng = thread_rng();
    bench.iter(|| { Grid::random(rng, 320, 240) })
}

fn grid_next(bench: &mut Bencher) {
    let rng = thread_rng();
    let grid = Grid::random(rng, 320, 240);
    bench.iter(|| { grid.next(Cell::next) })
}

/*****************************************************************************/

benchmark_group!(benches, 
    grid_random, 
    grid_next, 
);
benchmark_main!(benches);

/*****************************************************************************/
