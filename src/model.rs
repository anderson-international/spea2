pub type MutationOperator<'a> = Box<dyn FnMut(&[Objective], &mut ModelItem) + 'a>;

pub trait Spea2Model {
    fn get_model(&self) -> Model;
    fn get_mutation_operator(&mut self) -> MutationOperator<'_>;
}

#[derive(Debug, Default, Clone)]
pub struct Model {
    pub objectives: Vec<Objective>,
    pub population: Vec<ModelItem>,
    pub archive: Vec<ModelItem>,
    pub mating_pool: Vec<ModelItem>,
    pub population_size: usize,
    pub neighbourhood_size: usize,
    objective_sort_index: usize,
}
impl Model {
    pub fn new(objectives: Vec<Objective>, population: Vec<ModelItem>) -> Self {
        let population_size = population.len();
        Self {
            objectives,
            population,
            population_size,
            neighbourhood_size: (population_size / 10) + (population_size % 10 != 0) as usize,
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
    pub custom_data_index: Option<usize>,
}

impl ModelItem {
    pub fn new(values: Vec<f32>, custom_data_index: Option<usize>) -> Self {
        Self {
            values,
            fitness: 0.0,
            custom_data_index,
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

#[cfg(test)]
mod tests {
    use crate::mocks::{self, MOCK_POPULATION_COUNT};

    #[test]
    fn model_next_objective_sort_index() {
        let mut model = mocks::get_model_basic();

        let mut sort_index = model.next_objective_sort_index();
        assert_eq!(sort_index, 0);

        sort_index = model.next_objective_sort_index();
        assert_eq!(sort_index, 1);

        sort_index = model.next_objective_sort_index();
        assert_eq!(sort_index, 0);

        sort_index = model.next_objective_sort_index();
        assert_eq!(sort_index, 1);
    }

    #[test]
    fn model_get_average_archive_values() {
        let model = mocks::get_model_with_archive();
        let average_values = model.get_average_archive_values();
        let expected = MOCK_POPULATION_COUNT as f32 / 2.0;
        assert_eq!(average_values, vec![expected, expected]);
    }
}
