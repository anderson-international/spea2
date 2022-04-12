use std::fmt::format;

pub trait Spea2Model {
    fn get_model(self) -> Model;
}

#[derive(Debug, Clone)]
pub struct Model {
    pub objectives: Vec<Objective>,
    pub population: Vec<ModelItem>,
    pub archive: Vec<ModelItem>,
    pub mating_pool: Vec<ModelItem>,
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
