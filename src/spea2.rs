mod constants;
pub mod fitness;
mod mocks;
pub mod model;
mod selection;
use model::Spea2Model;
mod item;
mod sack;

pub fn evolve<T>(model: T)
where
    T: Spea2Model,
{
    // let model = model.get_model();
    // let (strengths, dominators, distances) =
    //     fitness::get_strengths_dominators_distances(model.population);
    // let raw_fitness = get_raw_fitness(population.count, &dominators, &strengths);

    // let (fitness, non_dominated, distances) =
    //     get_fitness_and_non_dominated(population.count, distances, &raw_fitness);
    // selection::environmental_selection(&mut archive, &union, fitness, non_dominated, distances);
}

