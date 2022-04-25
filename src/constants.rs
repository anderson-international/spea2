use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARCHIVE_MAX: usize = 100;
    pub static ref MUTATION_RATE: f32 = 0.06;
    pub static ref POPULATION_COUNT: usize = 250;
    pub static ref GENERATION_COUNT: usize = 1000;
    pub static ref NEIGHBOURHOOD_SIZE: usize = 10;
}

