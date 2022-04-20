mod item;
mod sack;

use sack::SackPool;
use std::time::Instant;

fn main() {
    let sack_pool = SackPool::new();

    let start = Instant::now();

    let model = spea2::evolve(sack_pool);

    let duration = start.elapsed();

    println!("duration: {:?}", duration);
    println!("{:?}", model);
}
