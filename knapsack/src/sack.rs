use crate::item::{Item, ItemPool};
use rand::Rng;

static CAPACITY: i32 = 10;
static POOL_SIZE: i32 = 4;

#[derive(Debug)]
pub struct Sack {
    pub weight: i32,
    pub value: i32,
    items: Vec<Item>,
}
impl Sack {
    pub fn new() -> Self {
        Self {
            weight: 0,
            value: 0,
            items: vec![],
        }
    }
}
#[derive(Debug)]
pub struct SackPool {
    item_pool: ItemPool,
    sacks: Vec<Sack>,
}

impl SackPool {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let item_pool = ItemPool::new();
        let sacks = (0..POOL_SIZE)
            .map(|_| {
                let mut ip = item_pool.items.clone();
                let mut sack = Sack::new();
                while sack.weight < CAPACITY {
                    let index = rng.gen_range(0..ip.len()) as usize;
                    let item = ip[index];
                    if sack.weight + item.weight >= CAPACITY {
                        break;
                    }
                    sack.weight += item.weight;
                    sack.value += item.value;
                    sack.items.push(item);
                    ip.remove(index);
                }
                sack
            })
            .collect();
        Self { item_pool, sacks }
    }
}
