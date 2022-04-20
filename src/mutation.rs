use rand::Rng;

use crate::{
    constants::MUTATION_RATE,
    model::{Model, MutOp, Spea2Model},
};

pub fn mutate(model: &mut Model, mutate: MutOp<'_>) {
    let mut rng = rand::thread_rng();
    for index in 0..model.mating_pool.len() {
        let r = rng.gen_range(0f32..1f32);
        if r < *MUTATION_RATE {
            model.mating_pool[index] = mutate(model, index);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::mocks;

    use super::*;

    #[test]
    fn mutation_perform_mutation() {
        let spea2_model = mocks::get_spea2model();
        let mut model = spea2_model.get_model();
        let mutate = spea2_model.get_mutation_operator();
        let before = model.mating_pool[0].clone();
        let mutated = mutate(&mut model, 0);

        assert_ne!(before.values, mutated.values);

        println!("{:?}", before);
        println!("{:?}", mutated);
    }
}
