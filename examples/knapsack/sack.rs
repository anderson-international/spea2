use rand::Rng;
use spea2::model::{Direction, Model, ModelItem, Objective, Spea2Model};

use crate::item::{Item, ItemPool};
use lazy_static::lazy_static;

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
    pub fn add(&mut self, item: Item) {
        self.items.push(item);
        self.weight += item.weight;
        self.value += item.value;
    }

    pub fn update(&mut self) {
        self.weight = 0.0;
        self.value = 0.0;
        for item in self.items.iter() {
            self.weight += item.weight;
            self.value += item.value;
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
        let mut rng = rand::thread_rng();
        let item_pool = ItemPool::new();
        let mut sp = Self {
            item_pool,
            sacks: vec![],
        };

        sp.sacks = (0..*SACK_COUNT)
            .map(|_| {
                let mut items = sp.item_pool.items.clone();
                let mut len = items.len();
                let mut sack = Sack::new();
                while sack.weight < *SACK_MAX_WEIGHT && len > 0 {
                    let index = rng.gen_range(0..len) as usize;
                    let item = items[index];
                    if sack.weight + item.weight >= *SACK_MAX_WEIGHT {
                        break;
                    }
                    sack.add(item);
                    items.remove(index);
                    len -= 1;
                }
                sack
            })
            .collect();
        sp
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
            .map(|sack| ModelItem::new(vec![sack.value, sack.weight]))
            .collect();
        model
    }


    fn get_mutation_operator(&self) -> spea2::model::MutOp<'_> {
        todo!()
    }

    fn is_item_feasible(&self, item: &ModelItem) -> bool {
        todo!()
    }
}
