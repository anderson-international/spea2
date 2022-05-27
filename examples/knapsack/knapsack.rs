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
mod sack {
    use super::{item::Item, pool::Pool};
    use rand::{prelude::SliceRandom, thread_rng};
    pub const SACK_WEIGHT_MAX: usize = 100;

    #[derive(Default)]
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
                if weight + item_weight > SACK_WEIGHT_MAX {
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
            self.items
                .iter()
                .map(|item| item.get_key())
                .collect::<Vec<String>>()
                .join(":")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn knapsack_sack_get_key() {
            let mut sack = Sack::default();
            (0..5).for_each(|i| sack.add_item(Item::new(i.to_string())));
            assert_eq!(sack.get_key(), "0:1:2:3:4");
        }
        #[test]
        fn knapsack_sack_get_weight() {
            let mut sack = Sack::default();
            const ITEM_COUNT:usize = 5;
            
            (0..ITEM_COUNT).for_each(|i| {
                let mut item = Item::new(i.to_string());
                item.set_value(ITEM_COUNT-(i+1));
                item.set_weight(i);
                sack.add_item(item);
            });
            let total = (0..ITEM_COUNT).sum();
            assert_eq!(sack.get_weight(), total);
            assert_eq!(sack.get_value(), total);
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

        fn mutate(&self, item: &mut spea2::ModelItem) {
            // unimplemented!()
        }

        fn crossover(&mut self, a: &mut spea2::ModelItem, b: &mut spea2::ModelItem) {
            // unimplemented!()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::knapsack::sack;
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
    }
}
