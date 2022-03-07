pub mod fitness;
mod item;
pub mod sack;

use sack::{Sack, SackPool};

pub fn evolve(population: &Vec<Sack>) {
    let archive = vec![];
    let mut union = population.clone();
    union.append(&mut archive.clone());
    let fitness = fitness::get(union);
}

pub fn initialise_population() -> Vec<Sack> {
    let sack_count = 10;
    let sack_max_weight = 50;
    let mut population = SackPool::new();
    population.initialise(sack_count, sack_max_weight);
    population.sacks
}
