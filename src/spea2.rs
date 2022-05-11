use model::{Model, MutationOperator};

pub mod canvas;
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

    use crate::model::{Direction, Spea2Model};

    use super::*;

    #[test]
    fn spea2_evolve_average_archive_values() {
        let mut spea2_model = mocks::get_spea2model();
        let mut model = spea2_model.get_model();
        let mut mutation = spea2_model.get_mutation_operator();
        let gen = 100;

        println!("Pop: {}", model.population_size);
        println!("Gen: {}", gen);

        //run once to create an archive
        evolve(&mut model, &mut mutation);

        let before = model.get_average_archive_values();

        let start = Instant::now();
        let mut success = true;

        (0..gen).for_each(|count| {
            evolve(&mut model, &mut mutation);
            let after = model.get_average_archive_values();
            model.objectives.iter().for_each(|objective| {
                let i = objective.index;
                let dir = &objective.direction;
                match dir {
                    Direction::Maximised => success = before[i] <= after[i],
                    Direction::Minimised => success = before[i] >= after[i],
                };
                assert!(
                    success,
                    "{}. {:#?}({}): {} - {}",
                    count, dir, i, before[i], after[i]
                );
            });
        });
        println!("duration: {:?}", start.elapsed());
    }

    #[test]
    fn spea2_evolve_average_fitness() {
        let mut spea2_model = mocks::get_spea2model();
        let mut model = spea2_model.get_model();
        let mut mutation = spea2_model.get_mutation_operator();
        let gen = 10;

        println!("Pop: {}", model.population_size);
        println!("Gen: {}", gen);

        //run once to initialse
        evolve(&mut model, &mut mutation);

        let before = model.get_average_fitness();

        let start = Instant::now();
        (0..gen).for_each(|_| {
            evolve(&mut model, &mut mutation);
            let after = model.get_average_fitness();
            println!("{} - {}", before, after);
            // assert!(after <= before);
        });

        println!("duration: {:?}", start.elapsed());
    }
}
