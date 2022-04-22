pub mod mocks;
pub mod model;

mod constants;
mod crossover;
mod fitness;
mod mutation;
mod reproduction;
mod selection;

use model::Spea2Model;

pub fn evolve<T: Spea2Model>(spea2_model: T) {
    let mut model = spea2_model.get_model();
    let mutate = spea2_model.get_mutation_operator();

    fitness::set_fitness(&mut model);
    selection::apply_selection(&mut model);
    reproduction::reproduce(&mut model, mutate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spea2_evolve() {
        let spea2_model = mocks::get_spea2model();
        evolve(spea2_model);
    }
}
