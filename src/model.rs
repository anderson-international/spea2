pub trait Spea2Model {
    fn get_model(self) -> Model;
}

#[derive(Debug, Clone)]
pub struct Model {
    pub objectives: Vec<Objective>,
    pub population: Vec<ModelItem>,
    pub archive: Vec<ModelItem>,
    pub mating_pool: Vec<ModelItem>,
    crossover_sort_index: usize,
}
impl Model {
    pub fn next_objective_sort_index(&mut self) -> usize {
        let index = self.crossover_sort_index;
        self.crossover_sort_index += 1;
        if self.crossover_sort_index >= self.objectives.len() {
            self.crossover_sort_index = 0;
        }
        index
    }
}
impl Default for Model {
    fn default() -> Self {
        Self {
            objectives: Vec::new(),
            population: Vec::new(),
            archive: Vec::new(),
            mating_pool: Vec::new(),
            crossover_sort_index: 0,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Objective {
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub struct ModelItem {
    pub values: Vec<f32>,
    pub fitness: f32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Distance {
    pub from: usize,
    pub to: usize,
    pub value: f32,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Maximised,
    Minimised,
}
