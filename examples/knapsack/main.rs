use knapsack::model::Model;
use spea2::{canvas::Canvas, EA};

mod knapsack;

fn main() {
    let mut model = Model::new();
    let ea = EA::new(&mut model);
    Canvas::new(ea, &model).show();
}
