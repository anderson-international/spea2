use crate::model::{Model, ModelItem, Objective};
extern crate itermore;
use rand::Rng;

pub fn mutate(model: &mut Model, is_item_feasible: &Box<dyn Fn(&ModelItem) -> bool>) {
    let mut rng = rand::thread_rng();

    model.mating_pool.iter_mut().for_each(|item| {
        let r = rng.gen_range(0f32..1f32);
        if r < 0.06 {
            perform_mutation(item, &model.objectives, is_item_feasible);
        }
    });
}

fn perform_mutation(
    item: &mut ModelItem,
    objectives: &[Objective],
    is_item_feasible: &Box<dyn Fn(&ModelItem) -> bool>,
) {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(0..objectives.len());
    let Objective { min, max, .. } = objectives[i];

    loop {
        item.values[i] = rng.gen_range(min..=max);
        if is_item_feasible(item) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::mocks;

    use super::*;

    #[test]
    fn mutation_perform_mutation() {
        let model = mocks::get_model_for_reproduction();
        let mut item = model.mating_pool[0].clone();
        let before = item.values.clone();
        let is_item_feasible = mocks::get_always_feasible();

        perform_mutation(&mut item, &model.objectives, &is_item_feasible);

        assert_ne!(before, item.values);

    }
}
