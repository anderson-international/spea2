use rand::Rng;

const WEIGHT_MAX: f32 = 25.0;
const WEIGHT_MIN: f32 = 5.0;
const VALUE_MAX: f32 = 25.0;
const VALUE_MIN: f32 = 5.0;
const POOL_SIZE: i32 = 25;

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: f32,
    pub value: f32,
}
impl Item {
    pub fn new(weight: f32, value: f32) -> Self {
        Self { weight, value }
    }
}

#[derive(Debug, Clone)]
pub struct ItemPool {
    pub items: Vec<Item>,
}

impl ItemPool {
    pub fn new() -> Self {
        ItemPool {
            items: (0..POOL_SIZE).map(|_| ItemPool::rnd()).collect(),
        }
    }

    pub fn rnd() -> Item {
        let mut rng = rand::thread_rng();
        Item::new(
            rng.gen_range(WEIGHT_MIN..WEIGHT_MAX),
            rng.gen_range(VALUE_MIN..VALUE_MAX),
        )
    }
}
