use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARCHIVE_MAX: usize = 10;
    pub static ref POPULATION_COUNT: usize = 10;
    pub static ref ARCHIVE_MAX_BENCH: usize = 100;
    pub static ref POPULATION_COUNT_BENCH: usize = 1000;
    pub static ref GENERATION_COUNT: usize = 1;
    pub static ref BIT_COUNT: usize = 32;
    pub static ref NEIGHBOURHOOD_SIZE: usize = 10;
}
