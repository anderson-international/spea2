use crate::constants::*;
use crate::model::*;
use rand::Rng;

fn model_init(population_count: usize, archive_count: usize) -> Model {
    let mut model = Model::default();
    model.objectives = vec![
        Objective {
            name: "objective_max".to_string(),
            direction: Direction::Maximised,
            min: 0.0,
            max: 255.0,
        },
        Objective {
            name: "objective_min".to_string(),
            direction: Direction::Minimised,
            min: 0.0,
            max: 255.0,
        },
    ];

    model.population = (0..population_count)
        .map(|_| get_modelitem_rnd(&model.objectives))
        .collect();

    model.archive = (0..archive_count)
        .map(|_| get_modelitem_rnd(&model.objectives))
        .collect();

    model
}
pub fn get_modelitem_rnd(objectives: &[Objective]) -> ModelItem {
    let mut rng = rand::thread_rng();
    ModelItem::new(
        vec![
            rng.gen_range(0.0..objectives[0].max),
            rng.gen_range(0.0..objectives[1].max),
        ],
        0.0,
        Some(0),
    )
}
pub fn get_model_basic() -> Model {
    let mut model = model_init(2, 1);
    model.population[0].values = vec![0.0, 0.0];
    model.population[1].values = vec![4.0, 0.0];
    model.archive[0].values = vec![0.0, 3.0];
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
    let mating_pool_count = 100;
    let mut model = model_init(0, 0);
    model.mating_pool = (0..mating_pool_count)
        .map(|i| {
            let v = i as f32;
            let c = mating_pool_count as f32;
            ModelItem::new(vec![v, c - v], 0.0, Some(0))
        })
        .collect();
    model
}

pub fn get_model_for_mating_pool_selection() -> Model {
    model_init(0, 100)
}

pub fn get_dominated() -> Vec<ModelItem> {
    vec![
        ModelItem::new(vec![0.0, 0.0], 0.1, Some(0)),
        ModelItem::new(vec![4.0, 0.0], 0.2, Some(1)),
        ModelItem::new(vec![0.0, 3.0], 0.3, Some(2)),
    ]
}

pub fn get_non_dominated() -> Vec<ModelItem> {
    vec![
        ModelItem::new(vec![5.0, 0.0], 1.0, Some(0)),
        ModelItem::new(vec![0.0, 12.0], 1.1, Some(1)),
        ModelItem::new(vec![7.0, 0.0], 1.2, Some(2)),
        ModelItem::new(vec![0.0, 24.0], 1.3, Some(3)),
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
        custom_data: vec![
            MockCustomData { value: 0.0 },
            MockCustomData { value: 1.0 },
            MockCustomData { value: 2.0 },
            MockCustomData { value: 3.0 },
        ],
    }
}

#[derive(Debug)]
pub struct MockCustomData {
    pub value: f32,
}
impl MockCustomData {
    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.value = rng.gen_range(0.0..10.0);
    }
}
#[derive(Debug)]
pub struct MockSpea2Model {
    pub custom_data: Vec<MockCustomData>,
}
impl Spea2Model for MockSpea2Model {
    fn get_model(&self) -> Model {
        model_init(*POPULATION_COUNT, *ARCHIVE_MAX)
    }

    fn get_mutation_operator(&mut self) -> MutationOperator {
        let mut_op = move |objectives: &[Objective], item: &mut ModelItem| {
            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0..objectives.len());
            let Objective { min, max, .. } = objectives[i];
            let index = item.custom_data_index.unwrap();
            let custom_data_item = self.custom_data.get_mut(index).unwrap();
            custom_data_item.update();
            item.values[i] =
                rng.gen_range(min..=max - custom_data_item.value) + custom_data_item.value
        };
        Box::new(mut_op)
    }
}
