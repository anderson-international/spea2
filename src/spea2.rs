use model::{Model, MutationOperator};

pub mod mocks;
pub mod model;

mod constants;
mod crossover;
mod fitness;
mod mutation;
mod reproduction;
mod selection;

pub fn evolve(model: &mut Model, mutation: &mut MutationOperator) {
    fitness::set_fitness(model);
    selection::apply_selection(model);
    reproduction::reproduce(model, mutation);
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::model::Spea2Model;

    use super::*;

    #[test]
    fn spea2_evolve() {
        let mut spea2_model = mocks::get_spea2model();
        let mut model = spea2_model.get_model();
        let mut mutation = spea2_model.get_mutation_operator();
        let start = Instant::now();

        (0..10).for_each(|_| evolve(&mut model, &mut mutation));

        println!("duration: {:?}", start.elapsed());
    }
}
