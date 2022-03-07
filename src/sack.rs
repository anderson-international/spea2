use crate::item::{Item, ItemPool};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Sack {
    pub weight: i32,
    pub value: i32,
    pub items: Vec<Item>,
}
impl Sack {
    pub fn new() -> Self {
        Self {
            weight: 0,
            value: 0,
            items: vec![],
        }
    }
    pub fn add(&mut self, item: Item) {
        self.items.push(item);
        self.weight += item.weight;
        self.value += item.value;
    }

    pub fn update(&mut self) {
        self.weight = 0;
        self.value = 0;
        for item in self.items.iter() {
            self.weight += item.weight;
            self.value += item.value;
        }
    }
}

#[derive(Debug)]
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

    pub fn initialise(&mut self, sack_count: i32, sack_max_weight: i32) {
        let mut rng = rand::thread_rng();

        self.sacks = (0..sack_count)
            .map(|_| {
                let mut items = self.item_pool.items.clone();
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
    }
}
