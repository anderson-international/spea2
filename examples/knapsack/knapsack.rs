mod pool {
    use super::item::Item;

    pub const POOL_SIZE: usize = 10;

    #[derive(Clone)]
    pub struct Pool {
        pub items: Vec<Item>,
    }
    impl Pool {
        pub fn new() -> Self {
            Self {
                items: (0..POOL_SIZE).map(|i| Item::new(i.to_string())).collect(),
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn knapsack_pool_new() {
            let pool = Pool::new();
            assert!(pool.items.len() == POOL_SIZE);
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
        pub fn set_key(&mut self, key: String) {
            self.key = key;
        }
        pub fn set_weight(&mut self, weight: usize) {
            self.weight = weight;
        }
        pub fn set_value(&mut self, value: usize) {
            self.value = value;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn knapsack_item_new() {
            let item = Item::new("test_key".to_string());
            assert_eq!(item.get_key(), "test_key".to_string());
            assert!((WEIGHT_MIN..WEIGHT_MAX).contains(&item.get_weight()));
            assert!((VALUE_MIN..VALUE_MAX).contains(&item.get_value()));
        }
    }
}
mod sack {
    use std::collections::HashMap;

    use super::{item::Item, pool::Pool};
    use rand::{prelude::SliceRandom, thread_rng};
    pub const SACK_WEIGHT_MAX: usize = 100;

    #[derive(Default)]
    pub struct Sack {
        items: HashMap<String, Item>,
    }
    impl Sack {
        pub fn new(pool: &mut Pool) -> Self {
            let mut sack = Sack {
                items: HashMap::new(),
            };
            sack.fill(pool);
            sack
        }
        pub fn fill(&mut self, pool: &mut Pool) {
            let mut items = pool.items.clone();
            items.shuffle(&mut thread_rng());
            let mut weight = self.get_weight();
            for item in items {
                let item_weight = item.get_weight();
                if weight + item_weight > SACK_WEIGHT_MAX {
                    continue;
                }
                weight += item_weight;
                self.add_item(item);
            }
        }
        pub fn add_item(&mut self, item: Item) {
            self.items.insert(item.get_key(), item);
        }
        pub fn remove_item(&mut self, key: &str) {
            self.items.remove(key);
        }
        pub fn get_weight(&self) -> usize {
            self.items.iter().map(|(_, item)| item.get_weight()).sum()
        }
        pub fn get_value(&self) -> usize {
            self.items.iter().map(|(_, item)| item.get_value()).sum()
        }
        pub fn get_key(&self) -> String {
            self.items
                .iter()
                .map(|(key, _)| key.clone())
                .collect::<Vec<String>>()
                .join(":")
        }
        pub fn get_item_count(&self) -> usize {
            self.items.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn knapsack_sack_new() {
            let mut pool = Pool::new();
            let sack = Sack::new(&mut pool);

            assert!(sack.get_weight() <= SACK_WEIGHT_MAX);
        }

        #[test]
        fn knapsack_sack_get_key() {
            let mut sack = Sack::default();
            (0..5).for_each(|i| sack.add_item(Item::new(i.to_string())));
            assert_eq!(sack.get_key(), "0:1:2:3:4");
        }
        #[test]
        fn knapsack_sack_get_weight() {
            let mut sack = Sack::default();
            const ITEM_COUNT: usize = 5;

            (0..ITEM_COUNT).for_each(|i| {
                let mut item = Item::new(i.to_string());
                item.set_value(ITEM_COUNT - (i + 1));
                item.set_weight(i);
                sack.add_item(item);
            });
            let total = (0..ITEM_COUNT).sum();
            assert_eq!(sack.get_weight(), total);
            assert_eq!(sack.get_value(), total);
        }
    }
}

pub mod model {
    use rand::{thread_rng, Rng};

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
            let model_item = spea2::ModelItem::new(
                vec![sack.get_weight() as f32, sack.get_value() as f32],
                sack.get_key(),
            );
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
                    direction: spea2::Direction::Minimised,
                },
                spea2::Objective {
                    index: 1,
                    name: "value".to_string(),
                    min: item::VALUE_MIN as f32,
                    max: item::VALUE_MAX as f32,
                    direction: spea2::Direction::Maximised,
                },
            ]
        }

        fn mutate(&mut self, item: &mut spea2::ModelItem) {
            let (key, mut sack) = self.sacks.remove_entry(&item.key).unwrap();
            loop {
                let key = thread_rng().gen_range(0..sack.get_item_count()).to_string();
                let item = sack.remove_item(&key);
                sack.fill(&mut self.pool);
                if key != sack.get_key() {
                    break;
                }
            }
            let key = sack.get_key();
            item.values[0] = sack.get_weight() as f32;
            item.values[1] = sack.get_value() as f32;
            item.key = key.clone();
            self.sacks.insert(key, sack);
        }

        fn crossover(&mut self, a: &mut spea2::ModelItem, b: &mut spea2::ModelItem) {
            // unimplemented!()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::knapsack::sack::{self, SACK_WEIGHT_MAX};
        use spea2::Model;

        #[test]
        fn knapsack_model_get_model_item() {
            let mut model = super::Model::new();
            let model_item = model.get_model_item();

            assert_eq!(model_item.values.len(), 2);
            assert!(model_item.values[0] <= sack::SACK_WEIGHT_MAX as f32);
            assert!(model.sacks.contains_key(&model_item.key));
        }

        #[test]
        fn knapsack_model_get_objectives() {
            let model = super::Model::new();
            let objectives = model.get_objectives();

            assert_eq!(objectives.len(), 2);
            assert!(matches!(
                objectives[0].direction,
                spea2::Direction::Minimised
            ));
            assert!(matches!(
                objectives[1].direction,
                spea2::Direction::Maximised
            ));
        }
        #[test]
        fn knapsack_model_mutate() {
            let mut model = super::Model::new();
            let mut model_item = model.get_model_item();

            let sack_count_before = model.sacks.len();
            let key_before = model_item.key.clone();
            let weight_before = model_item.values[0];
            let value_before = model_item.values[1];

            model.mutate(&mut model_item);

            let sack_count_after = model.sacks.len();
            let key_after = model_item.key.clone();
            let weight_after = model_item.values[0];
            let value_after = model_item.values[1];

            println!("SACK_WEIGHT_MAX: {}", SACK_WEIGHT_MAX);
            println!(
                "Before [w: {} v: {}] -  k: {} ",
                weight_before, value_before, key_before
            );
            println!(
                "After [w: {} v: {}] -  k: {} ",
                weight_after, value_after, key_after
            );

            assert_eq!(sack_count_before, sack_count_after);
            assert_ne!(key_before, key_after);

            assert!(!model.sacks.contains_key(&key_before));
            assert!(model.sacks.contains_key(&key_after));
            assert!(weight_after <= SACK_WEIGHT_MAX as f32);

            let indices: Vec<&str> = "6:4:1:2:3".split(':').collect();
            println!("{:?}", indices);
            println!("{:?}", indices.contains(&"6"));
        }
    }
}
