use rand::Rng;

use crate::{Direction, Distance, Model, ModelItem, Objective, EA};

pub const MOCK_MIN_OBJECTIVE_VALUE: f32 = 0.0;
pub const MOCK_MAX_OBJECTIVE_VALUE: f32 = 100.0;
pub const MOCK_POPULATION_COUNT: usize = 10;

pub fn get_sequential_model_item_vec() -> Vec<ModelItem> {
    (0..=MOCK_POPULATION_COUNT)
        .map(|i| {
            ModelItem::new(
                vec![MOCK_POPULATION_COUNT as f32 - i as f32, i as f32],
                String::new(),
            )
        })
        .collect()
}

// pub fn get_ea_basic() -> EA {
//     let objectives = get_objectives();
//     let custom_data_index = Some(0);
//     let population = vec![
//         ModelItem::new(vec![0.0, 0.0], custom_data_index),
//         ModelItem::new(vec![4.0, 0.0], custom_data_index),
//     ];
//     let mut model = EA::new(objectives, population);
//     model.archive = vec![ModelItem::new(vec![0.0, 3.0], custom_data_index)];
//     model
// }

// pub fn get_model_with_fitness() -> EA {
//     let mut model = get_ea_basic();
//     model.population[0].fitness = 0.1;
//     model.population[1].fitness = 1.5;
//     model.archive[0].fitness = 0.9;
//     model
// }

// pub fn get_model_with_mating_pool() -> EA {
//     let objectives = get_objectives();
//     let population = get_rnd_model_item_vec(&objectives);
//     let mut model = EA::new(objectives, population);
//     model.mating_pool = (0..MOCK_POPULATION_COUNT)
//         .map(|i| {
//             let v = i as f32;
//             let c = MOCK_POPULATION_COUNT as f32;
//             ModelItem::new(vec![v, c - v], Some(0))
//         })
//         .collect();
//     model
// }

// pub fn get_model_with_archive() -> EA {
//     let objectives = get_objectives();
//     let population = get_rnd_model_item_vec(&objectives);
//     let archive = get_sequential_model_item_vec();
//     let mut model = EA::new(objectives, population);
//     model.archive = archive;
//     model
// }

pub fn get_model_item_with_fitness(values: Vec<f32>, fitness: f32) -> ModelItem {
    let mut model_item = ModelItem::new(values, String::new());
    model_item.fitness = fitness;
    model_item
}

pub fn get_dominated() -> Vec<ModelItem> {
    vec![
        get_model_item_with_fitness(vec![0.0, 0.0], 0.1),
        get_model_item_with_fitness(vec![4.0, 0.0], 0.2),
        get_model_item_with_fitness(vec![0.0, 3.0], 0.3),
    ]
}

pub fn get_non_dominated() -> Vec<ModelItem> {
    vec![
        get_model_item_with_fitness(vec![5.0, 0.0], 1.0),
        get_model_item_with_fitness(vec![0.0, 12.0], 1.1),
        get_model_item_with_fitness(vec![7.0, 0.0], 1.2),
        get_model_item_with_fitness(vec![0.0, 24.0], 1.3),
    ]
}

pub fn get_sorted_distances() -> Vec<Distance> {
    vec![
        Distance::new(0, 1, 1.0),
        Distance::new(0, 2, 1.0),
        Distance::new(0, 3, 3.0),
        Distance::new(1, 2, 4.0),
        Distance::new(1, 3, 5.0),
        Distance::new(3, 2, 6.0),
    ]
}

pub fn get_distances_with_tie() -> Vec<Distance> {
    vec![
        Distance::new(0, 1, 1.0),
        Distance::new(0, 2, 2.0),
        Distance::new(0, 3, 4.0),
        Distance::new(1, 2, 3.0),
        Distance::new(1, 3, 1.0),
        Distance::new(3, 2, 5.0),
    ]
}

pub fn get_model() -> MockModel {
    MockModel::default()
}

#[derive(Debug, Default)]
pub struct MockModel {}
impl Model for MockModel {
    fn get_model_item(&mut self) -> ModelItem {
        let mut rng = rand::thread_rng();
        ModelItem::new(
            vec![
                rng.gen_range(MOCK_MIN_OBJECTIVE_VALUE..MOCK_MAX_OBJECTIVE_VALUE),
                rng.gen_range(MOCK_MIN_OBJECTIVE_VALUE..MOCK_MAX_OBJECTIVE_VALUE),
            ],
            String::new(),
        )
    }

    fn mutate(&self, item: &mut ModelItem) {
        todo!()
    }

    fn get_objectives(&self) -> Vec<Objective> {
        vec![
            Objective {
                name: "mock_objective_maximised".to_string(),
                direction: Direction::Maximised,
                min: MOCK_MIN_OBJECTIVE_VALUE,
                max: MOCK_MAX_OBJECTIVE_VALUE,
                index: 0,
            },
            Objective {
                name: "mock_objective_minimised".to_string(),
                direction: Direction::Minimised,
                min: MOCK_MIN_OBJECTIVE_VALUE,
                max: MOCK_MAX_OBJECTIVE_VALUE,
                index: 1,
            },
        ]
    }

    fn crossover(&mut self, a: &mut ModelItem, b: &mut ModelItem) {
        todo!()
    }
}
