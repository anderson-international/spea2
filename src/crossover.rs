use crate::constants::NEIGHBOURHOOD_SIZE;
use crate::model::{Model, ModelItem};
extern crate itermore;
use itermore::Itermore;
use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;

pub fn neighbourhood_crossover(model: &mut Model) {
    let mut rng = rand::thread_rng();
    let split_index = rng.gen_range(0..model.objectives.len()) as usize;

    sort_pool_by_objective(model);
    neighbourhood_shuffle(model, &mut rng);

    for [p1, p2] in model.mating_pool.iter_mut().array_chunks() {
        perform_crossover(p1, p2, split_index);
    }
}

fn sort_pool_by_objective(model: &mut Model) {
    let i = model.next_objective_sort_index();
    model
        .mating_pool
        .sort_by(|a, b| a.values[i].partial_cmp(&b.values[i]).unwrap());
}

fn neighbourhood_shuffle(model: &'_ mut Model, rng: &mut ThreadRng) {
    let pool = model.mating_pool.as_mut_slice();
    for i in 0..pool.len() / *NEIGHBOURHOOD_SIZE {
        let start = i * *NEIGHBOURHOOD_SIZE;
        let end = start + *NEIGHBOURHOOD_SIZE;
        pool[start..end].shuffle(rng);
    }
}

fn perform_crossover(p1: &mut ModelItem, p2: &mut ModelItem, split_index: usize) {
    let clone1 = p1.values.clone();
    let clone2 = p2.values.clone();

    let (p1_left, p1_right) = clone1.split_at(split_index);
    let (p2_left, p2_right) = clone2.split_at(split_index);

    p1.values = [p1_left, p2_right].concat();
    p2.values = [p2_left, p1_right].concat();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;

    #[test]
    fn crossover_next_objective_sort_index() {
        let mut model = mocks::get_model_with_mating_pool();

        let mut sort_index = model.next_objective_sort_index();
        assert_eq!(sort_index, 0);

        sort_index = model.next_objective_sort_index();
        assert_eq!(sort_index, 1);

        sort_index = model.next_objective_sort_index();
        assert_eq!(sort_index, 0);
    }

    #[test]
    fn crossover_sort_pool_by_objective() {
        let mut rng = rand::thread_rng();
        let mut model = mocks::get_model_with_mating_pool();
        model.mating_pool.shuffle(&mut rng);

        sort_pool_by_objective(&mut model);
        for [p1, p2] in model.mating_pool.iter().array_chunks() {
            assert!(p1.values[0] < p2.values[0]);
        }

        sort_pool_by_objective(&mut model);
        for [p1, p2] in model.mating_pool.iter().array_chunks() {
            assert!(p1.values[1] < p2.values[1]);
        }
    }

    #[test]
    fn crossover_neighbourhood_shuffle() {
        let mut model = mocks::get_model_with_mating_pool();
        let mut rng = rand::thread_rng();
        let sort_index = 0;

        neighbourhood_shuffle(&mut model, &mut rng);

        for i in 0..model.mating_pool.len() / *NEIGHBOURHOOD_SIZE {
            let start = i * 10;
            let end = start + 10;
            let neighbours = model.mating_pool[start..end]
                .iter()
                .map(|p| p.values[sort_index])
                .collect::<Vec<_>>();
            let s = start as f32;
            let e = end as f32;
            assert!(neighbours.iter().all(|n2| n2 >= &s && n2 < &e));
        }
    }

    #[test]
    fn crossover_perform_crossover() {
        let model = mocks::get_model_with_mating_pool();
        let split_index = 1;

        let mut p1 = model.mating_pool[0].clone();
        let mut p2 = model.mating_pool[1].clone();

        assert_eq!(p1.values, vec![0.0, 100.0]);
        assert_eq!(p2.values, vec![1.0, 99.0]);

        perform_crossover(&mut p1, &mut p2, split_index);

        assert_eq!(p1.values, vec![0.0, 99.0]);
        assert_eq!(p2.values, vec![1.0, 100.0]);
    }
}
