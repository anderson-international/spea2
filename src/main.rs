mod item;
mod sack;
mod spea2;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    spea2::evolve();

    let duration = start.elapsed();

    println!("duration: {:?}", duration);
}
