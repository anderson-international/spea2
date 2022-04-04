use std::time::Instant;

use spea2::sack::SackPool;

fn main() {
    let sack_count = 10;
    let sack_max_weight = 50.0;
    let sack_pool = SackPool::new(sack_count, sack_max_weight);
    let start = Instant::now();
    let model = spea2::evolve(sack_pool);
    let duration = start.elapsed();

    println!("duration: {:?}", duration);
    println!("{:?}", model);
}
