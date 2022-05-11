use rand::{prelude::ThreadRng, Rng};
use spea2::model::*;

const ITEM_COUNT: usize = 20;
const MAX_STRENGTH: usize = 90;
const MAX_WEIGHT: usize = 90;
const MIN_STRENGTH: usize = 10;
const MIN_WEIGHT: usize = 10;

#[derive(Debug)]
pub struct BasicModel {
    items: Vec<BasicModelItem>,
    rnd: ThreadRng,
}

impl BasicModel {
    pub fn new() -> Self {
        let mut rnd = rand::thread_rng();
        BasicModel {
            items: (0..ITEM_COUNT)
                .map(|_| {
                    let strength = rnd.gen_range(MIN_STRENGTH..MAX_STRENGTH) as f32;
                    let ratio = rnd.gen_range(0.0..1.0);
                    BasicModelItem::new(strength, ratio)
                })
                .collect(),
            rnd,
        }
    }
}
#[derive(Debug, Default)]
pub struct BasicModelItem {
    strength: f32,
    weight: f32,
    ratio: f32,
}
impl BasicModelItem {
    pub fn new(strength: f32, ratio: f32) -> Self {
        Self {
            strength,
            weight: strength * ratio,
            ratio,
        }
    }
    fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio;
        self.weight = self.strength * ratio;
    }
    fn set_strength(&mut self, strength: f32) {
        self.strength = strength;
        self.weight = strength * self.ratio;
    }
    fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
        self.strength = weight * (1f32 / self.ratio);
    }
}

impl Spea2Model for BasicModel {
    fn get_model(&self) -> Model {
        let objectives = vec![
            Objective {
                name: "strength".to_string(),
                direction: Direction::Maximised,
                min: MIN_STRENGTH as f32,
                max: MAX_STRENGTH as f32,
                index: 0,
            },
            Objective {
                name: "wieght".to_string(),
                direction: Direction::Minimised,
                min: MIN_WEIGHT as f32,
                max: MAX_WEIGHT as f32,
                index: 1,
            },
        ];
        let population = self
            .items
            .iter()
            .enumerate()
            .map(|(index, item)| ModelItem::new(vec![item.strength, item.weight], Some(index)))
            .collect();

        Model::new(objectives, population)
    }

    fn get_mutation_operator(&mut self) -> MutationOperator {
        let mut_op = move |_: &[Objective], model_item: &mut ModelItem| {
            let item_index = model_item.custom_data_index.unwrap();
            let basic_item = self.items.get_mut(item_index).unwrap();
            basic_item.set_ratio(self.rnd.gen_range(0.0..1.0));
            if self.rnd.gen_bool(0.5) {
                basic_item.set_strength(self.rnd.gen_range(MIN_STRENGTH..MAX_STRENGTH) as f32);
            } else {
                basic_item.set_weight(self.rnd.gen_range(MIN_WEIGHT..MAX_WEIGHT) as f32);
            }
            model_item.values = vec![basic_item.strength, basic_item.weight];
        };
        Box::new(mut_op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_model_new() {
        let basic_model = BasicModel::new();
        assert_eq!(basic_model.items.len(), ITEM_COUNT);
        basic_model.items.iter().for_each(|item| {
            let item_ratio = ((item.weight / item.strength) * 1000.0).round() / 1000.0;
            let ratio = (item.ratio * 1000.0).round() / 1000.0;
            assert_eq!(item_ratio, ratio);
        });
    }

    #[test]
    fn basic_mutation() {
        let mut basic_model = BasicModel::new();
        let spea2_model = basic_model.get_model();
        let mut mut_op = basic_model.get_mutation_operator();

        let before = spea2_model.population[0].clone();
        let mut after = spea2_model.population[0].clone();

        mut_op(&spea2_model.objectives, &mut after);

        assert_ne!(before.values, after.values);
    }
}
