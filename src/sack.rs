use rand::Rng;

use crate::{
    item::{Item, ItemPool},
    DecisionValues, Direction, Model, ModelValues,
};

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
    pub fn new(sack_count: i32, sack_max_weight: f32) -> Self {
        let mut rng = rand::thread_rng();
        let item_pool = ItemPool::new();
        let mut sp = Self {
            item_pool,
            sacks: vec![],
        };

        sp.sacks = (0..sack_count)
            .map(|_| {
                let mut items = sp.item_pool.items.clone();
                let mut len = items.len();
                let mut sack = Sack::new();
                while sack.weight < sack_max_weight && len > 0 {
                    let index = rng.gen_range(0..len) as usize;
                    let item = items[index];
                    if sack.weight + item.weight >= sack_max_weight {
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

impl Model for SackPool {
    fn get_values(self) -> ModelValues {
        let weights = DecisionValues::new(
            Direction::Minimised,
            self.sacks.iter().map(|s| s.weight).collect(),
        );
        let values = DecisionValues::new(
            Direction::Maximised,
            self.sacks.iter().map(|s| s.value).collect(),
        );
        ModelValues {
            count: self.sacks.len(),
            decision_count: 2,
            decisions: vec![weights, values],
        }
    }
}
