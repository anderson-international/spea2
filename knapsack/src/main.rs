mod item;
mod sack;

use sack::SackPool;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    SackPool::new();
    let duration = start.elapsed();

    println!("duration: {:?}", duration);
}
