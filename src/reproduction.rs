use crate::constants::POPULATION_COUNT;
use crate::model::{Model, ModelItem};
extern crate itermore;
use itermore::Itermore;
use rand::prelude::SliceRandom;
use rand::Rng;

pub fn reproduce(model: &mut Model) {
    mating_selection(model);
    neighbourhood_crossover(model);
}

fn mating_selection(model: &mut Model) {
    model.mating_pool.clear();
    let mut rng = rand::thread_rng();
    let len = model.archive.len();

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

fn mutate(child: &mut ModelItem) {
    todo!()
}

fn neighbourhood_crossover(model: &mut Model) {
    let mut rng = rand::thread_rng();
    let bin_len = model.objectives.len() * 32;
    let xo_i = model.get_next_crossover_sort_index();
    let pool = model.mating_pool.as_mut_slice();

    pool.sort_by(|a, b| a.values[xo_i].partial_cmp(&b.values[xo_i]).unwrap());
    for i in 0..pool.len() / 10 {
        let start = i * 10;
        let end = start + 10;
        pool[start..end].shuffle(&mut rng);
    }

    for [p1, p2] in pool.iter_mut().array_chunks() {
        let bin1 = binary_encode(&p1.values);
        let bin2 = binary_encode(&p2.values);
        let xo = rng.gen_range(1..bin_len) as usize;
        let s1 = bin1.split_at(xo);
        let s2 = bin2.split_at(xo);

        p1.values = binary_decode(&format!("{}{}", s1.0, s2.1));
        p2.values = binary_decode(&format!("{}{}", s2.0, s1.1));
    }
}

fn binary_encode(values: &[f32]) -> String {
    let mut bin = String::with_capacity(values.len() * 32);
    values.iter().for_each(|v| {
        bin.push_str(&format!("{:032b}", v.to_bits()));
    });
    bin
}

fn binary_decode(bin: &str) -> Vec<f32> {
    (0..bin.len() / 32)
        .map(|i| {
            let start = i * 32;
            let end = start + 32;
            f32::from_bits(u32::from_str_radix(&bin[start..end], 2).unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn reproduction_binary_encode_decode() {
        let values = vec![0.0, -1.5, f32::MAX, f32::MIN];
        let s = binary_encode(&values);
        let values2 = binary_decode(&s);
        assert_eq!(values, values2);
    }
}
