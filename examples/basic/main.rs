mod model;

use model::BasicModel;
use spea2::{canvas::Canvas, model::Spea2Model};

fn main() {
    let mut sack_pool = BasicModel::new();
    let model = sack_pool.get_model();
    let mutation = sack_pool.get_mutation_operator();

    Canvas::new(model, mutation).show();
}
