use rand::Rng;

static MAX_WEIGHT: i32 = 10;
static MAX_VALUE: i32 = 10;
static POOL_SIZE: i32 = 10;

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: i32,
    pub value: i32,
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
        Item {
            weight: rng.gen_range(1..MAX_WEIGHT),
            value: rng.gen_range(1..MAX_VALUE),
        }
    }
}
