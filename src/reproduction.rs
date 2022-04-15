use crate::constants::POPULATION_COUNT;
use crate::crossover;
use crate::model::Model;
use crate::mutation;
extern crate itermore;
use rand::Rng;

pub fn reproduce(model: &mut Model) {
    select_mating_pool(model);
    crossover::neighbourhood_crossover(model);
    mutation::mutate(model);
}

fn select_mating_pool(model: &mut Model) {
    let mut rng = rand::thread_rng();
    let len = model.archive.len();

    model.mating_pool.clear();

    for _ in 0..*POPULATION_COUNT {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;

    #[test]
    fn reproduction_select_mating_pool() {
        let mut model = mocks::get_model_for_mating_pool_selection();

        assert_eq!(model.mating_pool.len(), 0);

        select_mating_pool(&mut model);

        assert_eq!(model.mating_pool.len(), *POPULATION_COUNT);
    }
}
