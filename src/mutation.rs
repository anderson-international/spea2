use rand::Rng;

use crate::{constants::MUTATION_RATE, Model, EA};

pub fn mutate(ea: &mut EA, model: &mut Box<dyn Model>) {
    let mut rng = rand::thread_rng();
    let objectives = &ea.objectives;
    ea.mating_pool.iter_mut().for_each(|item| {
        if rng.gen_bool(MUTATION_RATE) {
            model.mutate(item);
        }
    });
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         mocks::{self},
//         Direction, Model,
//     };
//     #[test]
//     fn mutation_perform_mutation() {
//         let mut model = mocks::get_model();
//         let objectives = model.get_objectives();

//         (0..100).for_each(|count| {
//             model.population.iter().for_each(|item| {
//                 let before = item.clone();
//                 let mut after = item.clone();

//                 mutation(objectives, &mut after);

//                 model.objectives.iter().for_each(|objective| {
//                     let b_val = before.values[objective.index];
//                     let a_val = item.values[objective.index];
//                     let dir = &objective.direction;
//                     match dir {
//                         Direction::Maximised => {
//                             if a_val < b_val {
//                                 println!("{}. {:#?}: {} - {}", count, dir, b_val, a_val);
//                                 panic!();
//                             }
//                         }
//                         Direction::Minimised => {
//                             if a_val > b_val {
//                                 println!("{}. {:#?}: {} - {}", count, dir, b_val, a_val);
//                                 panic!();
//                             }
//                         }
//                     }
//                 });
//             });
//         });
//     }
// }
