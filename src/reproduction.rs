use rand::Rng;

use crate::{Model, EA};

pub fn reproduce(ea: &mut EA, model: &dyn Model) {
    select_mating_pool(ea);
    // crossover::neighbourhood_crossover(model);
    // mutation::mutate(model, mutation);
    set_next_population(ea);
}

fn select_mating_pool(ea: &mut EA) {
    // let mut rng = rand::thread_rng();
    // let len = model.archive.len();

    // for _ in 0..model.population_size {
    //     let i = rng.gen_range(0..len) as usize;
    //     let mut j = rng.gen_range(0..len) as usize;
    //     while j == i {
    //         j = rng.gen_range(0..len) as usize;
    //     }
    //     if model.archive[i].fitness <= model.archive[j].fitness {
    //         model.mating_pool.push(model.archive[i].clone());
    //     } else {
    //         model.mating_pool.push(model.archive[j].clone());
    //     }
    // }

    ea.mating_pool.clear();
    ea.mating_pool.append(&mut ea.archive.clone());
}

fn set_next_population(ea: &mut EA) {
    ea.population.clear();
    ea.population.append(&mut ea.mating_pool);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::mocks::{self, MOCK_POPULATION_COUNT};

//     #[test]
//     fn reproduction_select_mating_pool() {
//         let mut ea = mocks::get_model_with_archive();

//         assert!(ea.mating_pool.is_empty());

//         select_mating_pool(&mut ea);

//         assert_eq!(ea.mating_pool.len(), ea.population_size);
//     }

//     #[test]
//     fn model_set_next_population() {
//         let mut model = mocks::get_model_with_mating_pool();

//         assert_eq!(model.mating_pool.len(), MOCK_POPULATION_COUNT);

//         set_next_population(&mut model);

//         assert_eq!(model.population.len(), MOCK_POPULATION_COUNT);
//         assert!(model.mating_pool.is_empty());
//     }
// }
