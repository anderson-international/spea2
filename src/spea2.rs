pub mod mocks;
pub mod model;
pub mod sack;

mod constants;
mod crossover;
mod fitness_function;
mod item;
mod mutation;
mod reproduction;
mod selection;

use model::Spea2Model;

pub fn evolve(spea2_model: impl Spea2Model) -> model::Model {
    let mut model = spea2_model.get_model();
    fitness_function::set_fitness(&mut model);
    selection::apply_selection(&mut model);
    let is_item_feasible = &spea2_model.get_feasibility_test();
    reproduction::reproduce(&mut model, is_item_feasible);
    model
}
