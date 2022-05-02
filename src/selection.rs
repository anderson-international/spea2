use crate::model::{Distance, Model, ModelItem};

pub fn apply_selection(model: &mut Model) {
    let (dominated, mut non_dominated) = drain_model_by_dominance(model);
    ensure_archive_size(dominated, &mut non_dominated, model.population_size);
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

fn ensure_archive_size(
    mut dominated: Vec<ModelItem>,
    non_dominated: &mut Vec<ModelItem>,
    archive_size: usize,
) -> Vec<Distance> {
    let nd_len = non_dominated.len();
    let mut distances: Vec<Distance> = vec![];

    match nd_len.cmp(&archive_size) {
        std::cmp::Ordering::Less => {
            dominated.sort_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap());
            dominated.truncate(archive_size - nd_len);
            non_dominated.extend(dominated);
        }
        std::cmp::Ordering::Greater => {
            while non_dominated.len() > archive_size {
                distances = get_orderable_distances(non_dominated);
                distances.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
                let closest = get_closest(&distances);
                non_dominated.remove(closest.from);
            }
        }
        std::cmp::Ordering::Equal => (),
    }
    distances
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

fn get_closest(distances: &[Distance]) -> &Distance {
    if (distances[0].value == distances[1].value) && (distances[0].from != distances[1].from) {
        match distance_tiebreak(distances, 0, 1) {
            Some(d) => d,
            None => &distances[0],
        }
    } else {
        &distances[0]
    }
}

fn distance_tiebreak(distances: &[Distance], i: usize, j: usize) -> Option<&Distance> {
    let d1: Vec<&Distance> = distances
        .iter()
        .filter(|d| d.from == i || d.to == i)
        .collect();
    let d2: Vec<&Distance> = distances
        .iter()
        .filter(|d| d.from == j || d.to == j)
        .collect();

    for i in 1..d1.len() {
        if d1[i].value < d2[i].value {
            return Some(d1[i]);
        } else if d2[i].value < d1[i].value {
            return Some(d2[i]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mocks;
    #[test]
    fn selection_drain_model_by_dominance() {
        let mut model = mocks::get_model_with_fitness();
        let (dominated, non_dominated) = drain_model_by_dominance(&mut model);
        assert!(dominated.iter().all(|item| item.fitness >= 1.0));
        assert!(non_dominated.iter().all(|item| item.fitness < 1.0));
        assert!(model.population.is_empty());
        assert!(model.archive.is_empty());
    }

    #[test]
    fn selection_get_orderable_distances() {
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
    fn selection_ensure_archive_size_extend() {
        selection_ensure_archive_size(5);
    }

    #[test]
    fn selection_ensure_archive_size_truncate() {
        selection_ensure_archive_size(2);
    }

    fn selection_ensure_archive_size(archive_max: usize) {
        let dominated = mocks::get_dominated();
        let mut non_dominated = mocks::get_non_dominated();
        ensure_archive_size(dominated, &mut non_dominated, archive_max);
        assert_eq!(non_dominated.len(), archive_max);
    }

    #[test]
    fn selection_get_closest() {
        let distances = mocks::get_sorted_distances();
        let closest = get_closest(&distances);
        assert_eq!(closest, &distances[0]);
    }

    #[test]
    fn selection_get_closest_with_tiebreak() {
        let mut distances = mocks::get_distances_with_tie();
        distances.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        let closest = get_closest(&distances);
        assert_eq!(closest.from, 1);
        assert_eq!(closest.to, 3);
    }
}
