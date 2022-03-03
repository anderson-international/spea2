use crate::sack::SackPool;

pub fn evolve() {
    let sack_count = 10;
    let sack_max_weight = 50;

    let mut sack_pool = SackPool::new();
    sack_pool.initialise(sack_count, sack_max_weight);
    let p = sack_pool.sacks;
    let mut s = vec![0; p.len()];

    for i in 0..p.len() {
        for j in i + 1..p.len() {
            if p[i].weight < p[j].weight && p[i].value > p[j].value {
                s[i] += 1;
            } else if p[j].weight < p[i].weight && p[j].value > p[i].value {
                s[j] += 1;
            }
        }
    }

    println!("{:#?}",);
    println!("{:?}", s);
}
