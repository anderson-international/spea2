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

// fn get_binary_from_values(values: &[f32]) -> String {
//     let mut bin = String::new();
//     values.iter().for_each(|v| {
//         bin.push_str(format!("{:#b}", (v * 100.0) as u32).as_str());
//     });
//     bin
// }

// fn get_values_from_binary(bin: &str) -> Vec<f32> {
//     let mut values: Vec<f32> = vec![];
//     bin.split("0b").skip(1).for_each(|word| {
//         let value = (i32::from_str_radix(&word, 2).unwrap() as f32) / 100.0;
//         values.push(value)
//     });
//     values
// }

fn get_binary_from_values(values: &[f32]) -> String {
    let bits: Vec<_> = values
        .iter()
        .map(|v| v.to_bits().to_string())
        .collect();
    bits.join(";")
}

fn get_values_from_binary(bin: &str) -> Vec<f32> {
    bin.split(";")
        .map(|bits| {
            f32::from_bits(bits.parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::mocks;

    use super::*;

    #[test]
    fn reproduction_get_binary_from_values() {
        let (vals, bin) = mocks::get_values_and_binary();
        let b = get_binary_from_values(&vals);
        println!("{:?}", b);
        assert_eq!(b, bin);
    }

    #[test]
    fn reproduction_get_values_from_binary() {
        let (vals, bin) = mocks::get_values_and_binary();
        let v = get_values_from_binary(bin.as_str());
        assert_eq!(v, vals);
        println!("{:?}", v);
    }
    #[test]
    fn reproduction_negatives() {
        let values = vec![0f32, 1f32, f32::MAX, f32::MIN];
        let s = get_binary_from_values(&values);
        let values2 = get_values_from_binary(&s);
        assert_eq!(values, values2);
    }
}
