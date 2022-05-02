use crate::model::*;
use rand::Rng;

pub const MOCK_MIN_OBJECTIVE_VALUE: f32 = 0.0;
pub const MOCK_MAX_OBJECTIVE_VALUE: f32 = 100.0;
pub const MOCK_POPULATION_COUNT: usize = 20;

fn get_objectives() -> Vec<Objective> {
    let objectives = vec![
        Objective {
            name: "mock_objective_maximised".to_string(),
            direction: Direction::Maximised,
            min: MOCK_MIN_OBJECTIVE_VALUE,
            max: MOCK_MAX_OBJECTIVE_VALUE,
        },
        Objective {
            name: "mock_objective_minimised".to_string(),
            direction: Direction::Minimised,
            min: MOCK_MIN_OBJECTIVE_VALUE,
            max: MOCK_MAX_OBJECTIVE_VALUE,
        },
    ];
    objectives
}
pub fn get_rnd_model_item_vec(objectives: &[Objective]) -> Vec<ModelItem> {
    let mut rng = rand::thread_rng();
    (0..MOCK_POPULATION_COUNT)
        .map(|_| {
            ModelItem::new(
                vec![
                    rng.gen_range(0.0..objectives[0].max),
                    rng.gen_range(0.0..objectives[1].max),
                ],
                Some(0),
            )
        })
        .collect()
}

pub fn get_model_basic() -> Model {
    let objectives = get_objectives();
    let custom_data_index = Some(0);
    let population = vec![
        ModelItem::new(vec![0.0, 0.0], custom_data_index),
        ModelItem::new(vec![4.0, 0.0], custom_data_index),
    ];
    let mut model = Model::new(objectives, population);
    model.archive = vec![ModelItem::new(vec![0.0, 3.0], custom_data_index)];
    model
}

pub fn get_model_with_fitness() -> Model {
    let mut model = get_model_basic();
    model.population[0].fitness = 0.1;
    model.population[1].fitness = 1.5;
    model.archive[0].fitness = 0.9;
    model
}

pub fn get_model_with_mating_pool() -> Model {
    let objectives = get_objectives();
    let population = vec![];
    let mut model = Model::new(objectives, population);
    model.mating_pool = (0..MOCK_POPULATION_COUNT)
        .map(|i| {
            let v = i as f32;
            let c = MOCK_POPULATION_COUNT as f32;
            ModelItem::new(vec![v, c - v], Some(0))
        })
        .collect();
    model
}

pub fn get_model_with_archive() -> Model {
    let objectives = get_objectives();
    let population = get_rnd_model_item_vec(&objectives);
    let archive = get_rnd_model_item_vec(&objectives);
    let mut model = Model::new(objectives, population);
    model.archive = archive;
    model
}

pub fn get_model_item_with_fitness(values: Vec<f32>, fitness: f32) -> ModelItem {
    let mut model_item = ModelItem::new(values, Some(0));
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

pub fn get_spea2model() -> MockSpea2Model {
    MockSpea2Model {
        custom_data: (0..MOCK_POPULATION_COUNT)
            .map(|_| MockCustomData::default())
            .collect(),
    }
}

#[derive(Debug, Default)]
pub struct MockCustomData {
    pub values: Vec<f32>,
}
impl MockCustomData {
    pub fn default() -> MockCustomData {
        let mut rng = rand::thread_rng();
        MockCustomData {
            values: vec![
                rng.gen_range(MOCK_MIN_OBJECTIVE_VALUE..MOCK_MAX_OBJECTIVE_VALUE),
                rng.gen_range(MOCK_MIN_OBJECTIVE_VALUE..MOCK_MAX_OBJECTIVE_VALUE),
            ]
        }
    }
}
#[derive(Debug)]
pub struct MockSpea2Model {
    pub custom_data: Vec<MockCustomData>,
}
impl Spea2Model for MockSpea2Model {
    fn get_model(&self) -> Model {
        let mut rng = rand::thread_rng();
        let objectives = get_objectives();
        let population = (0..MOCK_POPULATION_COUNT)
            .map(|i| {
                ModelItem::new(
                    vec![
                        rng.gen_range(0.0..objectives[0].max),
                        rng.gen_range(0.0..objectives[1].max),
                    ],
                    Some(i),
                )
            })
            .collect();

        Model::new(objectives, population)
    }

    fn get_mutation_operator(&mut self) -> MutationOperator {
        let mut_op = move |item: &mut ModelItem| {
            let mut rng = rand::thread_rng();
            let index = item.custom_data_index.unwrap();
            let objective_index: usize = rng.gen_range(0..=1);
            let custom_data_item = self.custom_data.get_mut(index).unwrap();
            custom_data_item.values[objective_index] = rng.gen_range(MOCK_MIN_OBJECTIVE_VALUE..MOCK_MAX_OBJECTIVE_VALUE);
            item.values = custom_data_item.values.clone();
        };
        Box::new(mut_op)
    }
}
