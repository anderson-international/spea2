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
    fn spea2_evolve() {
        let mut spea2_model = mocks::get_spea2model();
        let mut model = spea2_model.get_model();
        let mut mutation = spea2_model.get_mutation_operator();

        //run once to create an archive
        evolve(&mut model, &mut mutation);

        let before = model.clone();

        let start = Instant::now();

        (0..20).for_each(|_| evolve(&mut model, &mut mutation));

        println!("duration: {:?}", start.elapsed());

        let before_archive_len = before.archive.len();
        let after_archive_len = model.archive.len();

        println!("Pop: {}", model.population_size);

        let avg_0_before = before
            .archive
            .iter()
            .map(|item| item.values[0])
            .sum::<f32>()
            / before_archive_len as f32;

        let avg_1_before = before
            .archive
            .iter()
            .map(|item| item.values[1])
            .sum::<f32>()
            / before.archive.len() as f32;

        let avg_0_after =
            model.archive.iter().map(|item| item.values[0]).sum::<f32>() / after_archive_len as f32;

        let avg_1_after =
            model.archive.iter().map(|item| item.values[1]).sum::<f32>() / after_archive_len as f32;

        println!(
            "{}: {} - {}",
            model.objectives[0].name, avg_0_before, avg_0_after
        );
        println!(
            "{}: {} - {}",
            model.objectives[1].name, avg_1_before, avg_1_after
        );

        match model.objectives[0].direction {
            Direction::Maximised => assert!(avg_0_after > avg_0_before),
            Direction::Minimised => assert!(avg_0_after < avg_0_before),
        }

        match model.objectives[1].direction {
            Direction::Maximised => assert!(avg_1_after > avg_1_before),
            Direction::Minimised => assert!(avg_1_after < avg_1_before),
        }
    }
}
