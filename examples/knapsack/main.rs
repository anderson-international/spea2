mod item;
mod sack;

use sack::SackPool;
use spea2::model::Spea2Model;
use std::time::Instant;

fn main() {
    let mut sack_pool = SackPool::new();
    sack_pool.fill();

    let mut model = sack_pool.get_model();
    let mut mutation = sack_pool.get_mutation_operator();
    let start = Instant::now();

    let model = spea2::evolve(&mut model, &mut mutation);

    let elapsed = start.elapsed();

    println!("duration: {:?}", elapsed);
    println!("{:?}", model);
}
