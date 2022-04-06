mod constants;
pub mod environmental_selection;
pub mod fitness_funtion;
pub mod mocks;
pub mod model;
mod selection;

use model::Spea2Model;
mod item;
pub mod sack;

pub fn evolve(spea2_model: impl Spea2Model) -> model::Model {
    let mut model = spea2_model.get_model();
    fitness_funtion::set_fitness(&mut model);
    model
}
