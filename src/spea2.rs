mod constants;
pub mod fitness_funtion;
pub mod mocks;
pub mod model;
pub mod reproduction;
pub mod selection;

use model::Spea2Model;
mod item;
pub mod sack;

pub fn evolve(spea2_model: impl Spea2Model) -> model::Model {
    let mut model = spea2_model.get_model();
    fitness_funtion::set_fitness(&mut model);
    selection::apply_selection(&mut model);
    reproduction::reproduce(&mut model);
    model
}
