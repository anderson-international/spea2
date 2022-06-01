use constants::POPULATION_SIZE;

pub mod canvas;
pub mod mocks;

mod constants;
mod crossover;
mod fitness;
mod mutation;
mod reproduction;
mod selection;

pub fn evolve(ea: &mut EA, model: &dyn Model) {
    fitness::set_fitness(ea);
    selection::apply_selection(ea);
    reproduction::reproduce(ea, model);
}

pub trait Model {
    fn get_model_item(&mut self) -> ModelItem;
    fn get_objectives(&self) -> Vec<Objective>;
    fn mutate(&mut self, item: &mut ModelItem);
    fn crossover(&mut self, a: &mut ModelItem, b: &mut ModelItem);
}

#[derive(Debug, Default, Clone)]
pub struct EA {
    pub objectives: Vec<Objective>,
    pub population: Vec<ModelItem>,
    pub archive: Vec<ModelItem>,
    pub mating_pool: Vec<ModelItem>,
    pub neighbourhood_size: usize,
    objective_sort_index: usize,
}
impl EA {
    pub fn new(model: &mut dyn Model) -> Self {
        let population = (0..POPULATION_SIZE)
            .map(|_| model.get_model_item())
            .collect();
        let objectives = model.get_objectives();
        Self {
            objectives,
            population,
            neighbourhood_size: (POPULATION_SIZE / 10) + (POPULATION_SIZE % 10 != 0) as usize,
            ..Default::default()
        }
    }

    pub fn next_objective_sort_index(&mut self) -> usize {
        let index = self.objective_sort_index;
        self.objective_sort_index += 1;
        if self.objective_sort_index >= self.objectives.len() {
            self.objective_sort_index = 0;
        }
        index
    }

    pub fn get_average_archive_values(&self) -> Vec<f32> {
        let len = self.archive.len();
        self.objectives
            .iter()
            .map(|objective| {
                self.archive
                    .iter()
                    .map(|item| item.values[objective.index])
                    .sum::<f32>()
                    / len as f32
            })
            .collect()
    }

    pub fn get_average_fitness(&self) -> f32 {
        let len = self.archive.len();
        self.archive.iter().map(|item| item.fitness).sum::<f32>() / len as f32
    }
}

#[derive(Debug, Clone)]
pub struct Objective {
    pub name: String,
    pub direction: Direction,
    pub min: f32,
    pub max: f32,
    pub index: usize,
}

#[derive(Debug, Default, Clone)]
pub struct ModelItem {
    pub values: Vec<f32>,
    pub fitness: f32,
    pub key: String,
}

impl ModelItem {
    pub fn new(values: Vec<f32>, key: String) -> Self {
        Self {
            values,
            fitness: 0.0,
            key,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Distance {
    pub from: usize,
    pub to: usize,
    pub value: f32,
}

impl Distance {
    pub fn new(from: usize, to: usize, value: f32) -> Self {
        Self { from, to, value }
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Maximised,
    Minimised,
}

// #[cfg(test)]
// mod tests {
//     use crate::mocks::{self, MOCK_POPULATION_COUNT};

//     #[test]
//     fn model_next_objective_sort_index() {
//         let mut model = mocks::get_ea_basic();

//         let mut sort_index = model.next_objective_sort_index();
//         assert_eq!(sort_index, 0);

//         sort_index = model.next_objective_sort_index();
//         assert_eq!(sort_index, 1);

//         sort_index = model.next_objective_sort_index();
//         assert_eq!(sort_index, 0);

//         sort_index = model.next_objective_sort_index();
//         assert_eq!(sort_index, 1);
//     }

//     #[test]
//     fn model_get_average_archive_values() {
//         let model = mocks::get_model_with_archive();
//         let average_values = model.get_average_archive_values();
//         let expected = MOCK_POPULATION_COUNT as f32 / 2.0;
//         assert_eq!(average_values, vec![expected, expected]);
//     }

// #[test]
// fn spea2_evolve_average_archive_values() {
//     let mut spea2_model = mocks::get_spea2model();
//     let mut model = spea2_model.get_model();
//     let mut mutation = spea2_model.get_mutation_operator();
//     let gen = 100;

//     println!("Pop: {}", model.population_size);
//     println!("Gen: {}", gen);

//     //run once to create an archive
//     evolve(&mut model, &mut mutation);

//     let before = model.get_average_archive_values();

//     let start = Instant::now();
//     let mut success = true;

//     (0..gen).for_each(|count| {
//         evolve(&mut model, &mut mutation);
//         let after = model.get_average_archive_values();
//         model.objectives.iter().for_each(|objective| {
//             let i = objective.index;
//             let dir = &objective.direction;
//             match dir {
//                 Direction::Maximised => success = before[i] <= after[i],
//                 Direction::Minimised => success = before[i] >= after[i],
//             };
//             assert!(
//                 success,
//                 "{}. {:#?}({}): {} - {}",
//                 count, dir, i, before[i], after[i]
//             );
//         });
//     });
//     println!("duration: {:?}", start.elapsed());
// }

// #[test]
// fn spea2_evolve_average_fitness() {
//     let mut model = mocks::get_model();
//     let gen = 10;

//     println!("Pop: {}", model.population_size);
//     println!("Gen: {}", gen);

//     //run once to initialse
//     evolve(&mut model, &mut mutation);

//     let before = model.get_average_fitness();

//     let start = Instant::now();
//     (0..gen).for_each(|_| {
//         evolve(&mut model, &mut mutation);
//         let after = model.get_average_fitness();
//         println!("{} - {}", before, after);
//         // assert!(after <= before);
//     });

//     println!("duration: {:?}", start.elapsed());
// }
// }
