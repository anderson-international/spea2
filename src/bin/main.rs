use std::time::Instant;

use spea2::sack::SackPool;

fn main() {
    let sack_count = 10;
    let sack_max_weight = 50.0;
    let model = SackPool::new(sack_count, sack_max_weight);
    let start = Instant::now();
    spea2::evolve(model);
    let duration = start.elapsed();

    println!("duration: {:?}", duration);
}
