pub type MutationOperator<'a> = Box<dyn FnMut(&mut ModelItem) + 'a>;

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
    objective_sort_index: usize,
}
impl Model {

    pub fn new(objectives: Vec<Objective>, population: Vec<ModelItem>) -> Self {
        let population_size = population.len();
        Self {
            objectives,
            population,
            population_size,
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
}
#[derive(Debug, Clone)]
pub struct Objective {
    pub name: String,
    pub direction: Direction,
    pub min: f32,
    pub max: f32,
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
