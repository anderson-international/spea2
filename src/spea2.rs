pub mod fitness;
mod item;
pub mod sack;

pub trait Model {
    fn get_decision_vectors(self) -> Vec<Vec<f32>>;
}

pub struct DecisionVector {
    is_maximised: bool,
    values: Vec<f32>,
}

pub fn evolve<T>(model: T)
where
    T: Model,
{
    let population = model.get_decision_vectors();
    let archive: Vec<Vec<f32>> = vec![];
    let union = [&population[..], &archive[..]].concat();
    let (fitness, non_dominated) = fitness::get(&union);

    // archive.clear();
    // for i in non_dominated {
    //     archive.push(union[i].clone());
    // }
}
