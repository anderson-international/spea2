use crate::constants;
use crate::model::{Direction, Model, ModelItem, Objective, Spea2Model};

fn model_init(population_count: usize, archive_count: usize) -> Model {
    let mut model = Model {
        objectives: vec![
            Objective {
                direction: Direction::Maximised,
            },
            Objective {
                direction: Direction::Minimised,
            },
        ],
        population: vec![],
        archive: vec![],
    };

    for _ in 0..population_count {
        model.population.push(ModelItem {
            values: vec![0.0; 2],
            fitness: 0.0,
        });
    }
    for _ in 0..archive_count {
        model.archive.push(ModelItem {
            values: vec![0.0; 2],
            fitness: 0.0,
        });
    }

    model
}

pub fn get_model() -> Model {
    let mut model = model_init(2, 1);
    model.population[0].values = vec![0.0, 0.0];
    model.population[1].values = vec![4.0, 0.0];
    model.archive[0].values = vec![0.0, 3.0];
    model
}

#[derive(Debug)]
pub struct BenchModel {
    pub model: Model,
}
impl BenchModel {
    pub fn new() -> Self {
        let model = model_init(
            *constants::POPULATION_COUNT_BENCH,
            *constants::ARCHIVE_MAX_BENCH,
        );
        Self { model }
    }
}
impl Spea2Model for BenchModel {
    fn get_model(self) -> Model {
        self.model
    }
}
pub fn spea2model_for_bench() -> BenchModel {
    BenchModel::new()
}
