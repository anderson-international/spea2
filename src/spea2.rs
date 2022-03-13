pub mod fitness;
mod item;
pub mod sack;
mod selection;

use lazy_static::lazy_static;
lazy_static! {
    pub static ref ARCHIVE_SIZE: usize = 10;
}
pub trait Model {
    fn get_values(self) -> ModelValues;
}
#[derive(Debug)]
pub struct ModelValues {
    count: usize,
    decision_count: usize,
    decisions: Vec<DecisionValues>,
}
impl ModelValues {
    pub fn clone_empty(&self) -> Self {
        let mut decisions: Vec<DecisionValues> = vec![];
        for i in 0..self.decisions.len() {
            decisions.push(DecisionValues::new(self.decisions[i].direction, vec![]));
        }
        Self {
            count: 0,
            decision_count: self.decision_count,
            decisions,
        }
    }

    pub fn union(&self, other: &ModelValues) -> ModelValues {
        let mut decisions: Vec<DecisionValues> = vec![];
        for i in 0..self.decisions.len() {
            decisions.push(DecisionValues::new(
                self.decisions[i].direction,
                [
                    self.decisions[i].values.clone(),
                    other.decisions[i].values.clone(),
                ]
                .concat(),
            ));
        }

        ModelValues {
            count: self.count + other.count,
            decision_count: self.decision_count,
            decisions,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DecisionValues {
    direction: Direction,
    values: Vec<f32>,
}

impl DecisionValues {
    pub fn new(direction: Direction, values: Vec<f32>) -> Self {
        Self { direction, values }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Maximised,
    Minimised,
}
#[derive(Debug, Clone)]
pub struct OrderedValue {
    value: f32,
    index: usize,
}

pub fn evolve<T>(model: T)
where
    T: Model,
{
    let model_values = model.get_values();
    let mut archive = model_values.clone_empty();

    let union = model_values.union(&archive);
    let (fitness, non_dominated, distances) = fitness::get(&union);
    selection::environmental_selection(&mut archive, &union, fitness, non_dominated, distances);
}

#[cfg(test)]
pub mod test {

    use super::*;

    fn get_mock_model() -> ModelValues {
        let model = ModelValues {
            count: 4,
            decision_count: 2,
            decisions: vec![
                DecisionValues::new(Direction::Minimised, vec![1.0, 2.0, 3.0, 4.0]),
                DecisionValues::new(Direction::Maximised, vec![4.0, 3.0, 2.0, 1.0]),
            ],
        };
        model
    }

    fn get_mock_archive() -> ModelValues {
        let archive = ModelValues {
            count: 4,
            decision_count: 2,
            decisions: vec![
                DecisionValues::new(Direction::Minimised, vec![5.0, 6.0, 7.0, 8.0]),
                DecisionValues::new(Direction::Maximised, vec![8.0, 7.0, 6.0, 5.0]),
            ],
        };
        archive
    }

    #[test]
    pub fn test_union() {
        let model = get_mock_model();
        let archive = get_mock_archive();
        let union = model.union(&archive);

        assert_eq!(union.count, 8);
        assert_eq!(union.decision_count, 2);
        assert_eq!(union.decisions.len(), 2);
    }
}
