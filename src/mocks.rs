use crate::constants::*;
use crate::model::*;
use rand::Rng;

fn model_init(population_count: usize, archive_count: usize, mating_pool_count: usize) -> Model {
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
        .map(|_| ModelItem::new(vec![0.0, 0.0]))
        .collect();

    model.archive = (0..archive_count)
        .map(|_| ModelItem::new(vec![0.0, 0.0]))
        .collect();

    model.mating_pool = (0..mating_pool_count)
        .map(|i| {
            let v = i as f32;
            let c = mating_pool_count as f32;
            ModelItem::new(vec![v, c - v])
        })
        .collect();

    model
}

pub fn get_model() -> Model {
    let mut model = model_init(2, 1, 0);
    model.population[0].values = vec![0.0, 0.0];
    model.population[1].values = vec![4.0, 0.0];
    model.archive[0].values = vec![0.0, 3.0];
    model
}

pub fn get_model_with_fitness() -> Model {
    let mut model = get_model();
    model.population[0].fitness = 0.1;
    model.population[1].fitness = 1.5;
    model.archive[0].fitness = 0.9;
    model
}

pub fn get_model_for_reproduction() -> Model {
    model_init(0, 0, 100)
}

pub fn get_model_for_mating_pool_selection() -> Model {
    model_init(0, 100, 0)
}

pub fn get_dominated() -> Vec<ModelItem> {
    let mut dominated: Vec<ModelItem> = vec![];

    dominated.push(ModelItem {
        values: vec![0.0, 0.0],
        fitness: 0.1,
    });
    dominated.push(ModelItem {
        values: vec![4.0, 0.0],
        fitness: 0.2,
    });
    dominated.push(ModelItem {
        values: vec![0.0, 3.0],
        fitness: 0.3,
    });
    dominated
}

pub fn get_non_dominated() -> Vec<ModelItem> {
    let mut non_dominated: Vec<ModelItem> = vec![];

    non_dominated.push(ModelItem {
        values: vec![5.0, 0.0],
        fitness: 1.0,
    });
    non_dominated.push(ModelItem {
        values: vec![0.0, 12.0],
        fitness: 1.1,
    });
    non_dominated.push(ModelItem {
        values: vec![7.0, 0.0],
        fitness: 1.2,
    });
    non_dominated.push(ModelItem {
        values: vec![0.0, 24.0],
        fitness: 1.3,
    });

    non_dominated
}

pub fn get_sorted_distances() -> Vec<Distance> {
    let mut distances: Vec<Distance> = vec![];

    distances.push(Distance {
        from: 0,
        to: 1,
        value: 1.0,
    });

    distances.push(Distance {
        from: 0,
        to: 2,
        value: 1.0,
    });
    distances.push(Distance {
        from: 0,
        to: 3,
        value: 3.0,
    });
    distances.push(Distance {
        from: 1,
        to: 2,
        value: 4.0,
    });
    distances.push(Distance {
        from: 1,
        to: 3,
        value: 5.0,
    });
    distances.push(Distance {
        from: 3,
        to: 2,
        value: 6.0,
    });

    distances
}

pub fn get_distances_with_tie() -> Vec<Distance> {
    let mut distances: Vec<Distance> = vec![];

    distances.push(Distance {
        from: 0,
        to: 1,
        value: 1.0,
    });

    distances.push(Distance {
        from: 0,
        to: 2,
        value: 2.0,
    });
    distances.push(Distance {
        from: 0,
        to: 3,
        value: 4.0,
    });
    distances.push(Distance {
        from: 1,
        to: 2,
        value: 3.0,
    });
    distances.push(Distance {
        from: 1,
        to: 3,
        value: 1.0,
    });
    distances.push(Distance {
        from: 3,
        to: 2,
        value: 5.0,
    });
    distances
}
pub fn get_spea2model() -> MockSpea2Model {
    MockSpea2Model {}
}
#[derive(Debug, Clone)]
pub struct MockSpea2Model {}
impl Spea2Model for MockSpea2Model {
    fn get_model(&self) -> Model {
        model_init(*POPULATION_COUNT, *ARCHIVE_MAX, 10)
    }

    fn get_mutation_operator(&self) -> MutOp<'_> {
        let mut_op = move |model: &mut Model, index: usize| -> ModelItem {
            let mut rng = rand::thread_rng();
            let mut item = model.mating_pool[index].clone();
            let i = rng.gen_range(0..model.objectives.len());
            let Objective { min, max, .. } = model.objectives[i];

            loop {
                item.values[i] = rng.gen_range(min..=max);
                if self.is_item_feasible(&item) {
                    break;
                }
            }
            item
        };
        Box::new(mut_op)
    }

    fn is_item_feasible(&self, item: &ModelItem) -> bool {
        item.values[0] >= 0.0
    }
}
