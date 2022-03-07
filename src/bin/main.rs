use std::time::Instant;

fn main() {
    let start = Instant::now();
    let population = spea2::initialise_population();
    spea2::evolve(&population);
    let duration = start.elapsed();

    println!("duration: {:?}", duration);
}
