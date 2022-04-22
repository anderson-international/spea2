use rand::Rng;

use crate::{
    constants::MUTATION_RATE,
    model::{Model, MutOp},
};

pub fn mutate(model: &mut Model, mutate: MutOp<'_>) {
    let mut rng = rand::thread_rng();
    let objectives = &model.objectives[..];
    model.mating_pool.iter_mut().for_each(|item| {
        if rng.gen::<f32>() < *MUTATION_RATE {
            mutate(objectives, item);
        }
    });
}

#[cfg(test)]
mod tests {

    use crate::{mocks, model::Spea2Model};

    #[test]
    fn mutation_perform_mutation() {
        let spea2_model = mocks::get_spea2model();
        let mut model = spea2_model.get_model();
        let mutate = spea2_model.get_mutation_operator();
        let mut item = model.mating_pool.get_mut(0).unwrap();
        let before = item.clone();
        mutate(&model.objectives[..], &mut item);

        assert_ne!(before.values, item.values);

        println!("{:?}", before);
        println!("{:?}", item);
    }
}
