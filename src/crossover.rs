use crate::constants::{BIT_COUNT, NEIGHBOURHOOD_SIZE};
use crate::model::{Model, ModelItem};
extern crate itermore;
use itermore::Itermore;
use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;

pub fn neighbourhood_crossover(model: &mut Model) {
    let mut rng = rand::thread_rng();
    let binary_string_len = model.objectives.len() * *BIT_COUNT;
    let split_index = rng.gen_range(1..binary_string_len) as usize;

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

fn neighbourhood_shuffle<'a>(model: &'a mut Model, rng: &mut ThreadRng) {
    let pool = model.mating_pool.as_mut_slice();
    for i in 0..pool.len() / *NEIGHBOURHOOD_SIZE {
        let start = i * *NEIGHBOURHOOD_SIZE;
        let end = start + *NEIGHBOURHOOD_SIZE;
        pool[start..end].shuffle(rng);
    }
}

fn perform_crossover(p1: &mut ModelItem, p2: &mut ModelItem, split_index: usize) {
    let bin1 = binary_encode_values(&p1.values);
    let bin2 = binary_encode_values(&p2.values);
    let s1 = bin1.split_at(split_index);
    let s2 = bin2.split_at(split_index);
    p1.values = binary_decode_values(&format!("{}{}", s1.0, s2.1));
    p2.values = binary_decode_values(&format!("{}{}", s2.0, s1.1));
}

fn binary_encode_values(values: &[f32]) -> String {
    let mut bin = String::with_capacity(values.len() * *BIT_COUNT);
    values.iter().for_each(|v| {
        bin.push_str(&binary_encode(&v));
    });
    bin
}

fn binary_encode(value: &f32) -> String {
    format!("{:0bit_count$b}", value.to_bits(), bit_count = *BIT_COUNT)
}

fn binary_decode_values(bin: &str) -> Vec<f32> {
    (0..bin.len() / 32)
        .map(|i| {
            let start = i * *BIT_COUNT;
            let end = start + *BIT_COUNT;
            binary_decode(&bin[start..end])
        })
        .collect()
}

fn binary_decode(bin: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(&bin, 2).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;

    #[test]
    fn crossover_binary_encode_decode() {
        let values = vec![0.0, -1.5, f32::MAX, f32::MIN];
        let s = binary_encode_values(&values);
        let values2 = binary_decode_values(&s);
        assert_eq!(values, values2);
    }

    #[test]
    fn crossover_next_objective_sort_index() {
        let mut model = mocks::get_model_for_reproduction();

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
        let mut model = mocks::get_model_for_reproduction();
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
        let mut model = mocks::get_model_for_reproduction();
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
        let model = mocks::get_model_for_reproduction();
        let split_index = 32;

        let mut p1 = model.mating_pool[0].clone();
        let mut p2 = model.mating_pool[1].clone();

        assert_eq!(p1.values, vec![0.0, 100.0]);
        assert_eq!(p2.values, vec![1.0, 99.0]);

        perform_crossover(&mut p1, &mut p2, split_index);

        assert_eq!(p1.values, vec![0.0, 99.0]);
        assert_eq!(p2.values, vec![1.0, 100.0]);
    }
}
