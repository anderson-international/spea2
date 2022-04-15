use crate::constants::{BIT_COUNT};
use crate::model::{Model, ModelItem};
extern crate itermore;
use rand::Rng;


pub fn mutate(model: &mut Model) {
    let mut rng = rand::thread_rng();

    model.mating_pool.iter_mut().for_each(|item| {
        let r = rng.gen_range(0f32..1f32);
        if r < 0.06 {
            perform_mutation(item);
        }
    });
}

fn perform_mutation(item: &mut ModelItem) {
    let mut rng = rand::thread_rng();
    let vi = rng.gen_range(0..item.values.len());
    let mut bin = binary_encode(&item.values[vi]);
    let i = rng.gen_range(0..bin.len());
    if bin.as_bytes()[i] as char == '0' {
        bin.replace_range(i..i + 1, "1");
    } else {
        bin.replace_range(i..i + 1, "0");
    }
    item.values[vi] = binary_decode(&bin);
}

fn binary_encode(value: &f32) -> String {
    format!("{:0bit_count$b}", value.to_bits(), bit_count = *BIT_COUNT)
}

fn binary_decode(bin: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(&bin, 2).unwrap())
}

#[cfg(test)]
mod tests {

    use crate::mocks;

    use super::*;

    #[test]
    fn mutation_perform_mutation() {
        let model = mocks::get_model_for_reproduction();
        let mut item = model.mating_pool[0].clone();
        let before = item.values.clone();

        perform_mutation(&mut item);

        assert_ne!(before, item.values);

        println!("{:?}", item.values);
    }
}
