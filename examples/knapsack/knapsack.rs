mod pool {
    use super::item::Item;
    
    const SIZE: usize = 100;

    #[derive(Clone)]
    pub struct Pool {
        pub items: Vec<Item>,
    }
    impl Pool {
        pub fn new() -> Self {
            Self {
                items: (0..SIZE).map(|i| Item::new(i.to_string())).collect(),
            }
        }
    }
}
mod sack {
    use super::{item::Item, pool::Pool};
    use rand::{prelude::SliceRandom, thread_rng};
    const WEIGHT_MAX: usize = 100;
    pub struct Sack {
        items: Vec<Item>,
    }
    impl Sack {
        pub fn new(pool: &mut Pool) -> Self {
            let mut sack = Sack { items: Vec::new() };
            let mut items = pool.items.clone();
            items.shuffle(&mut thread_rng());
            let mut weight = 0;
            for item in items {
                let item_weight = item.get_weight();
                if weight + item_weight > WEIGHT_MAX {
                    continue;
                }
                weight += item_weight;
                sack.add_item(item);
            }
            sack
        }
        pub fn add_item(&mut self, item: Item) {
            self.items.push(item);
        }
        pub fn get_weight(&self) -> usize {
            self.items.iter().map(|item| item.get_weight()).sum()
        }
        pub fn get_value(&self) -> usize {
            self.items.iter().map(|item| item.get_value()).sum()
        }
        pub fn get_key(&self) -> String {
            let mut key = self
                .items
                .iter()
                .map(|item| item.get_key())
                .collect::<Vec<String>>()
                .join("-");
            key.pop();
            key
        }
    }
}

mod item {
    use rand::Rng;

    pub const WEIGHT_MIN: usize = 1;
    pub const WEIGHT_MAX: usize = 50;
    pub const VALUE_MIN: usize = 1;
    pub const VALUE_MAX: usize = 50;
    #[derive(Clone)]
    pub struct Item {
        key: String,
        weight: usize,
        value: usize,
    }
    impl Item {
        pub fn new(key: String) -> Self {
            let mut rng = rand::thread_rng();
            Self {
                key,
                weight: rng.gen_range(WEIGHT_MIN..WEIGHT_MAX),
                value: rng.gen_range(VALUE_MIN..VALUE_MAX),
            }
        }
        pub fn get_weight(&self) -> usize {
            self.weight
        }
        pub fn get_value(&self) -> usize {
            self.value
        }
        pub fn get_key(&self) -> String {
            self.key.clone()
        }
    }
}

pub mod model {
    use super::{
        item::{self},
        pool::Pool,
        sack::Sack,
    };
    use std::collections::HashMap;

    pub struct Model {
        pool: Pool,
        sacks: HashMap<String, Sack>,
    }

    impl Model {
        pub fn new() -> Self {
            Self {
                pool: Pool::new(),
                sacks: HashMap::new(),
            }
        }
    }

    impl spea2::Model for Model {
        fn get_model_item(&mut self) -> spea2::ModelItem {
            let sack = Sack::new(&mut self.pool);
            let model_item = spea2::ModelItem::new(vec![sack.get_weight() as f32, sack.get_value() as f32], sack.get_key());
            self.sacks.insert(sack.get_key(), sack);
            model_item
        }

        fn get_objectives(&self) -> Vec<spea2::Objective> {
            vec![
                spea2::Objective {
                    index: 0,
                    name: "wieght".to_string(),
                    min: item::WEIGHT_MIN as f32,
                    max: item::WEIGHT_MAX as f32,
                    direction: spea2::Direction::Maximised,
                },
                spea2::Objective {
                    index: 1,
                    name: "value".to_string(),
                    min: item::VALUE_MIN as f32,
                    max: item::VALUE_MAX as f32,
                    direction: spea2::Direction::Minimised,
                },
            ]
        }

        fn mutate(&self, item: &mut spea2::ModelItem) {
            // unimplemented!()
        }

        fn crossover(&mut self, a: &mut spea2::ModelItem, b: &mut spea2::ModelItem) {
            // unimplemented!()
        }
    }
}
