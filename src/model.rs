pub type MutOp<'a> = Box<dyn Fn(&mut Model, usize) -> ModelItem + 'a>;

pub trait Spea2Model {
    fn get_model(&self) -> Model;
    fn get_mutation_operator(&self) -> MutOp<'_>;
    fn is_item_feasible(&self, item: &ModelItem) -> bool;
}

#[derive(Debug)]
pub struct Model {
    pub objectives: Vec<Objective>,
    pub population: Vec<ModelItem>,
    pub archive: Vec<ModelItem>,
    pub mating_pool: Vec<ModelItem>,
    objective_sort_index: usize,
}
impl Model {
    pub fn next_objective_sort_index(&mut self) -> usize {
        let index = self.objective_sort_index;
        self.objective_sort_index += 1;
        if self.objective_sort_index >= self.objectives.len() {
            self.objective_sort_index = 0;
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
            objective_sort_index: 0,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Objective {
    pub name: String,
    pub direction: Direction,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone)]
pub struct ModelItem {
    pub values: Vec<f32>,
    pub fitness: f32,
}

impl ModelItem {
    pub fn new(values: Vec<f32>) -> Self {
        Self {
            values,
            fitness: 0.0,
        }
    }
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
