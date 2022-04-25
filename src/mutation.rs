use rand::Rng;

use crate::{
    constants::MUTATION_RATE,
    model::{Model, MutationOperator},
};

pub fn mutate(model: &mut Model, mutation: &mut MutationOperator) {
    let mut rng = rand::thread_rng();
    let objectives = model.objectives.as_slice();
    model.mating_pool.iter_mut().for_each(|item| {
        if rng.gen::<f32>() < *MUTATION_RATE {
            mutation(objectives, item);
        }
    });
}

#[cfg(test)]
mod tests {

    use crate::{mocks, model::Spea2Model};

    #[test]
    fn mutation_perform_mutation() {
        let mut spea2_model = mocks::get_spea2model();
        let mut mutation = spea2_model.get_mutation_operator();
        let mut model = mocks::get_model_with_mating_pool();
        let item = model.mating_pool.get_mut(0).unwrap();
        let before = item.clone();

        mutation(&model.objectives[..], item);

        assert_ne!(before.values, item.values);
    }
}
