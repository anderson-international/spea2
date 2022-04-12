use crate::constants::POPULATION_COUNT;
use crate::model::{Model, ModelItem};
use rand::Rng;

pub fn reproduce(model: &mut Model) {
    mating_selection(model);
    variation(model);
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

fn variation(model: &mut Model) {
    model.population.clear();
    let mut rng = rand::thread_rng();
    let len = model.archive.len();

    for _ in 0..*POPULATION_COUNT {
        let i = rng.gen_range(0..len) as usize;
        let mut j = rng.gen_range(0..len) as usize;
        while j == i {
            j = rng.gen_range(0..len) as usize;
        }
        let p1 = &model.archive[i];
        let p2 = &model.archive[j];
        let mut child = crossover(p1, p2);
        mutate(&mut child);
        model.population.push(child);
    }
}

fn mutate(child: &mut ModelItem) {
    todo!()
}

fn crossover(p1: &ModelItem, p2: &ModelItem) -> ModelItem {
    todo!()
}

fn get_binary_from_values(values: &[f32]) -> String {
    let mut bin = String::with_capacity(values.len() * 32);
    values.iter().for_each(|v| {
        bin.push_str(&format!("{:032b}", v.to_bits()));
    });
    bin
}

fn get_values_from_binary(bin: &str) -> Vec<f32> {
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
    fn reproduction_binary_encoding() {
        let values = vec![0.0, -1.5, f32::MAX, f32::MIN];
        let s = get_binary_from_values(&values);
        let values2 = get_values_from_binary(&s);
        assert_eq!(values, values2);
    }
}
