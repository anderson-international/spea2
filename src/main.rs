mod item;
mod sack;
mod spea2;

use std::time::Instant;

use crate::sack::SackPool;

fn main() {
    let start = Instant::now();
    let sack_count = 10;
    let sack_max_weight = 50;
    let mut sack_pool = SackPool::new();
    sack_pool.initialise(sack_count, sack_max_weight);
    spea2::evolve(sack_pool.sacks);

    let duration = start.elapsed();

    println!("duration: {:?}", duration);
}
