use crate::item::{Item, ItemPool};
use lazy_static::lazy_static;
use rand::{prelude::SliceRandom, Rng};
use spea2::model::{Direction, Model, ModelItem, MutationOperator, Objective, Spea2Model};

lazy_static! {
    pub static ref SACK_COUNT: usize = 10;
    pub static ref SACK_MAX_WEIGHT: f32 = 50.0;
}

#[derive(Debug, Clone, Default)]
pub struct Sack {
    pub weight: f32,
    pub value: f32,
    pub items: Vec<Item>,
}
impl Sack {
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
            let mut sack = Sack::default();
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
                name: "sack value".to_string(),
                direction: Direction::Maximised,
                min: 0.0,
                max: 255.0,
            },
            Objective {
                name: "sack wieght".to_string(),
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
            let sack = self.sacks.get_mut(sack_index).unwrap();
            let item_index = rng.gen_range(0..sack.items.len());
            sack.item_remove(item_index);
            sack.fill(&mut self.item_pool);
            item.values = vec![sack.value, sack.weight];
        };
        Box::new(mut_op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sack_item_add() {
        let mut sack = Sack::default();
        sack.item_add(Item::new(1.0, 3.0));
        sack.item_add(Item::new(2.0, 2.0));
        sack.item_add(Item::new(3.0, 1.0));

        assert_eq!(sack.weight, 6.0);
        assert_eq!(sack.value, 6.0);
    }

    #[test]
    fn sack_item_remove() {
        let mut sack = Sack::default();
        sack.item_add(Item::new(1.0, 3.0));
        sack.item_add(Item::new(2.0, 2.0));
        sack.item_add(Item::new(3.0, 1.0));

        assert_eq!(sack.weight, 6.0);
        assert_eq!(sack.value, 6.0);

        sack.item_remove(2);

        assert_eq!(sack.weight, 3.0);
        assert_eq!(sack.value, 5.0);
    }

    #[test]
    fn sack_fill() {
        let mut sack = Sack::default();
        let mut item_pool = ItemPool::new();
        sack.fill(&mut item_pool);
        assert!(sack.weight <= *SACK_MAX_WEIGHT);
        assert!(sack.value > 0.0);
        assert!(sack.items.len() > 0);
    }

    #[test]
    fn sack_pool_fill() {
        let mut sack_pool = SackPool::new();
        assert!(sack_pool.item_pool.items.len() > 0);

        sack_pool.fill();
        assert!(sack_pool.sacks.len() > 0);
    }

    #[test]
    fn sack_pool_mutation_operator() {
        let mut sack_pool = SackPool::new();
        sack_pool.fill();
        let mut model = sack_pool.get_model();
        let mut mutatation = sack_pool.get_mutation_operator();
        let model_item = model.population.get_mut(0).unwrap();
        let before = model_item.values.clone();

        mutatation(model.objectives.as_slice(), model_item);

        assert!(model_item.values != before);

        println!("before: {:?}", before);
        println!("after: {:?}", model_item.values);
    }
}
