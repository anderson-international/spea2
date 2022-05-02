use crate::model::{Model, MutationOperator};
use crate::{crossover, mutation};
use rand::Rng;

pub fn reproduce(model: &mut Model, mutation: &mut MutationOperator) {
    select_mating_pool(model);
    crossover::neighbourhood_crossover(model);
    mutation::mutate(model, mutation);
    set_next_population(model);
}

fn select_mating_pool(model: &mut Model) {
    let mut rng = rand::thread_rng();
    let len = model.archive.len();

    for _ in 0..model.population_size {
        let i = rng.gen_range(0..len) as usize;
        let mut j = rng.gen_range(0..len) as usize;
        while j == i {
            j = rng.gen_range(0..len) as usize;
        }
        if model.archive[i].fitness <= model.archive[j].fitness {
            model.mating_pool.push(model.archive[i].clone());
        } else {
            model.mating_pool.push(model.archive[j].clone());
        }
    }
}

fn set_next_population(model: &mut Model) {
    model.population.clear();
    model.population.append(&mut model.mating_pool);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks::{self, MOCK_POPULATION_COUNT};

    #[test]
    fn reproduction_select_mating_pool() {
        let mut model = mocks::get_model_with_archive();

        assert!(model.mating_pool.is_empty());

        select_mating_pool(&mut model);

        assert_eq!(model.mating_pool.len(), model.population_size);
    }

    #[test]
    fn reproduction_set_next_population() {
        let mut model = mocks::get_model_with_mating_pool();

        assert_eq!(model.mating_pool.len(), MOCK_POPULATION_COUNT);

        set_next_population(&mut model);

        assert_eq!(model.population.len(), MOCK_POPULATION_COUNT);
        assert!(model.mating_pool.is_empty());
    }
}
