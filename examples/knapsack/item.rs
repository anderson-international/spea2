use rand::Rng;

static MAX_WEIGHT: f32 = 25.0;
static MAX_VALUE: f32 = 25.0;
static POOL_SIZE: i32 = 25;

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
            rng.gen_range(1.0..MAX_WEIGHT),
            rng.gen_range(1.0..MAX_VALUE),
        )
    }
}
