mod item;
mod sack;

use sack::SackPool;
use spea2::{canvas::Canvas, model::Spea2Model};

fn main() {
    let mut sack_pool = SackPool::new();
    sack_pool.fill();

    let model = sack_pool.get_model();
    let mutation = sack_pool.get_mutation_operator();

    Canvas::new(model, mutation).show();
}
