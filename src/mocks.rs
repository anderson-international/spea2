use core::arch;

use crate::model::{Direction, Distance, Model, ModelItem, Objective};

fn model_init(pop_count: usize, arch_count: usize) -> Model {
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

    for _ in 0..pop_count {
        model.population.push(ModelItem {
            values: vec![0.0; 2],
            fitness: 0.0,
        });
    }
    for _ in 0..arch_count {
        model.archive.push(ModelItem {
            values: vec![0.0; 2],
            fitness: 0.0,
        });
    }
    model
}

pub fn model_for_union() -> Model {
    model_init(2, 2)
}

pub fn model_for_set_fitness() -> Model {
    let mut model = model_init(3, 0);
    model.population[0].values = vec![0.0, 0.0];
    model.population[1].values = vec![4.0, 0.0];
    model.population[2].values = vec![0.0, 3.0];
    model
}

pub fn model_for_raw_fitness() -> Model {
    let mut model = model_init(4, 0);
    model.population[0].values = vec![2.0, 4.0];
    model.population[1].values = vec![3.0, 3.0];
    model.population[2].values = vec![3.0, 2.0];
    model.population[3].values = vec![1.0, 5.0];
    model
}
