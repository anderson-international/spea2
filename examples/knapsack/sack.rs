use crate::item::{Item, ItemPool};
use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, Rng};
use spea2::model::{Direction, Model, ModelItem, MutationOperator, Objective, Spea2Model};

lazy_static! {
    pub static ref SACK_COUNT: usize = 10;
    pub static ref SACK_MAX_WEIGHT: f32 = 50.0;
}

#[derive(Debug, Clone)]
pub struct Sack {
    pub weight: f32,
    pub value: f32,
    pub items: Vec<Item>,
}
impl Sack {
    pub fn new() -> Self {
        Self {
            weight: 0.0,
            value: 0.0,
            items: vec![],
        }
    }

    pub fn item_add(&mut self, item: Item) {
        self.items.push(item);
        self.weight += item.weight;
        self.value += item.value;
    }

    pub fn item_remove(&mut self, index: usize) {
        let item = self.items.swap_remove(index);
        self.weight -= item.weight;
        self.value -= item.value;
    }

    pub fn fill(&mut self, item_pool: &mut ItemPool) {
        item_pool.items.shuffle(&mut rand::thread_rng());
        for item in item_pool.items.iter() {
            if self.weight + item.weight > *SACK_MAX_WEIGHT {
                continue;
            }
            self.item_add(*item);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SackPool {
    pub item_pool: ItemPool,
    pub sacks: Vec<Sack>,
}

impl SackPool {
    pub fn new() -> Self {
        let item_pool = ItemPool::new();
        Self {
            item_pool,
            sacks: vec![],
        }
    }
    pub fn fill(&mut self) {
        for _ in 0..*SACK_COUNT {
            let mut sack = Sack::new();
            sack.fill(&mut self.item_pool);
            self.sacks.push(sack);
        }
    }
}

impl Spea2Model for SackPool {
    fn get_model(&self) -> Model {
        let mut model = Model::default();
        model.objectives = vec![
            Objective {
                name: "value".to_string(),
                direction: Direction::Maximised,
                min: 0.0,
                max: 255.0,
            },
            Objective {
                name: "wieght".to_string(),
                direction: Direction::Minimised,
                min: 0.0,
                max: *SACK_MAX_WEIGHT,
            },
        ];
        model.population = self
            .sacks
            .iter()
            .enumerate()
            .map(|(index, sack)| ModelItem::new(vec![sack.value, sack.weight], 0.0, Some(index)))
            .collect();
        model
    }

    fn get_mutation_operator(&mut self) -> MutationOperator<'_> {
        let mut_op = move |_: &[Objective], item: &mut ModelItem| {
            let mut rng = rand::thread_rng();
            let sack_index = item.custom_data_index.unwrap();
            let item_index = rng.gen_range(0..self.sacks.len());
            let sack = self.sacks.get_mut(sack_index).unwrap();

            sack.item_remove(item_index);
            sack.fill(&mut self.item_pool);
            item.values = vec![sack.value, sack.weight];
        };
        Box::new(mut_op)
    }
}
