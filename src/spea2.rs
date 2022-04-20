pub mod mocks;
pub mod model;

mod constants;
mod crossover;
mod fitness_function;
mod mutation;
mod reproduction;
mod selection;

use model::Spea2Model;

pub fn evolve<T: Spea2Model>(spea2_model: T) {
    let mut model = spea2_model.get_model();

    let mutate = spea2_model.get_mutation_operator();

    fitness_function::set_fitness(&mut model);
    selection::apply_selection(&mut model);
    reproduction::reproduce(&mut model, mutate);
}
