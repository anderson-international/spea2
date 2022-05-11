use rand::Rng;

use crate::{
    constants::MUTATION_RATE,
    model::{Model, MutationOperator},
};

pub fn mutate(model: &mut Model, mutation: &mut MutationOperator) {
    let mut rng = rand::thread_rng();
    let objectives = &model.objectives;
    model.mating_pool.iter_mut().for_each(|item| {
        if rng.gen_bool(MUTATION_RATE) {
            mutation(objectives, item);
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::{self},
        model::{Direction, Spea2Model},
    };
    #[test]
    fn mutation_perform_mutation() {
        let mut spea2_model = mocks::get_spea2model();
        let model = spea2_model.get_model();
        let mut mutation = spea2_model.get_mutation_operator();
        let objectives = &&model.objectives;

        (0..100).for_each(|count| {
            model.population.iter().for_each(|item| {
                let before = item.clone();
                let mut after = item.clone();

                mutation(objectives, &mut after);

                model.objectives.iter().for_each(|objective| {
                    let b_val = before.values[objective.index];
                    let a_val = item.values[objective.index];
                    let dir = &objective.direction;
                    match dir {
                        Direction::Maximised => {
                            if a_val < b_val {
                                println!("{}. {:#?}: {} - {}", count, dir, b_val, a_val);
                                panic!();
                            }
                        }
                        Direction::Minimised => {
                            if a_val > b_val {
                                println!("{}. {:#?}: {} - {}", count, dir, b_val, a_val);
                                panic!();
                            }
                        }
                    }
                });
            });
        });
    }
}
