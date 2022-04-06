use crate::constants::ARCHIVE_MAX;
use crate::model::{Distance, Model, ModelItem};

pub fn apply_selection(model: &mut Model) {
    let (dominated, mut non_dominated) = drain_model_by_dominance(model);
    ensure_archive_size(dominated, &mut non_dominated, *ARCHIVE_MAX);
    model.archive = non_dominated;
}

fn drain_model_by_dominance(model: &mut Model) -> (Vec<ModelItem>, Vec<ModelItem>) {
    let mut dominated: Vec<ModelItem> = vec![];
    let mut non_dominated: Vec<ModelItem> = vec![];
    model
        .population
        .drain(..)
        .chain(model.archive.drain(..))
        .for_each(|item| {
            if item.fitness < 1.0 {
                non_dominated.push(item);
            } else {
                dominated.push(item);
            }
        });
    (dominated, non_dominated)
}

fn get_orderable_distances(dominated: &Vec<ModelItem>) -> Vec<Distance> {
    let d_len = dominated.len();
    let mut distances: Vec<Distance> = vec![];

    for i in 0..d_len {
        for j in i + 1..d_len {
            let mut distance: f32 = 0.0;
            dominated[i]
                .values
                .iter()
                .zip(dominated[j].values.iter())
                .for_each(|(a, b)| {
                    distance += (a - b).powf(2.0);
                });
            distance = distance.sqrt();
            distances.push(Distance {
                from: i,
                to: j,
                value: distance,
            });
        }
    }
    distances
}

fn ensure_archive_size(
    mut dominated: Vec<ModelItem>,
    non_dominated: &mut Vec<ModelItem>,
    archive_max: usize,
) {
    let nd_len = non_dominated.len();
    if nd_len < archive_max {
        dominated.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
        dominated.truncate(archive_max - nd_len);
        non_dominated.extend(dominated);
    } else if nd_len > archive_max {
        while non_dominated.len() > archive_max {
            let mut distances = get_orderable_distances(&non_dominated);
            distances.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());
            let distance = distances.pop().unwrap();
            non_dominated.remove(distance.from);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;
    #[test]
    fn test_drain_model_by_dominance() {
        let mut model = mocks::get_model_with_fitness();
        let (dominated, non_dominated) = drain_model_by_dominance(&mut model);
        assert!(dominated.iter().all(|item| item.fitness >= 1.0));
        assert!(non_dominated.iter().all(|item| item.fitness < 1.0));
        assert_eq!(model.population.len(), 0);
        assert_eq!(model.archive.len(), 0);
    }

    #[test]
    fn test_get_orderable_distances() {
        let dominated = mocks::get_dominated();
        let distances = get_orderable_distances(&dominated);
        assert_eq!(
            distances[0],
            Distance {
                from: 0,
                to: 1,
                value: 4.0,
            }
        );
        assert_eq!(
            distances[1],
            Distance {
                from: 0,
                to: 2,
                value: 3.0,
            }
        );
        assert_eq!(
            distances[2],
            Distance {
                from: 1,
                to: 2,
                value: 5.0,
            }
        );
    }

    #[test]
    fn test_ensure_archive_size_extend() {
        test_ensure_archive_size(5);
    }

    #[test]
    fn test_ensure_archive_size_truncate() {
        test_ensure_archive_size(2);
    }

    fn test_ensure_archive_size(archive_max: usize) {
        let dominated = mocks::get_dominated();
        let mut non_dominated = mocks::get_non_dominated();
        ensure_archive_size(dominated, &mut non_dominated, archive_max);
        assert_eq!(non_dominated.len(), archive_max);
        println!("{:?}", non_dominated);
    }
}
