use crate::model::{Direction, Model, ModelItem};
use quickersort::sort_floats;

pub fn set_fitness(model: &mut Model) -> (Vec<Vec<f32>>, Vec<Vec<usize>>, Vec<f32>) {
    let mut union: Vec<&mut ModelItem> = model
        .population
        .iter_mut()
        .chain(model.archive.iter_mut())
        .collect();

    let len_union = union.len();
    let len_objectives = model.objectives.len();
    let kth = (len_union as f64).sqrt() as usize;
    let mut distances: Vec<Vec<f32>> = vec![vec![0.0; len_union]; len_union];
    let mut strengths: Vec<f32> = vec![0.0; len_union];
    let mut dominators: Vec<Vec<usize>> = vec![vec![]; len_union];

    for i in 0..len_union {
        for j in i + 1..len_union {
            let mut distance: f32 = 0.0;
            let mut i_dom_j = false;
            let mut j_dom_i = false;
            let mut dominated = false;

            for k in 0..len_objectives {
                distance += (union[i].values[k] - union[j].values[k]).powi(2);
                if !dominated {
                    let (dv1, dv2) = match model.objectives[k].direction {
                        Direction::Maximised => (union[j].values[k], union[i].values[k]),
                        Direction::Minimised => (union[i].values[k], union[j].values[k]),
                    };

                    if dv1 < dv2 {
                        i_dom_j = true;
                        if j_dom_i {
                            dominated = true;
                        }
                    } else if dv2 < dv1 {
                        j_dom_i = true;
                        if i_dom_j {
                            dominated = true;
                        }
                    } else {
                        i_dom_j = false;
                        j_dom_i = false;
                        dominated = true;
                    }
                }
            }

            distance = distance.sqrt();
            distances[i][j] = distance;
            distances[j][i] = distance;

            if i_dom_j && !j_dom_i {
                strengths[i] += 1.0;
                dominators[j].push(i);
            } else if j_dom_i && !i_dom_j {
                strengths[j] += 1.0;
                dominators[i].push(j);
            }
        }
    }

    for i in 0..len_union {
        let mut raw_fitness = 0.0;
        for j in 0..dominators[i].len() {
            raw_fitness += strengths[dominators[i][j]];
        }
        let mut sorted = distances[i].clone();
        sort_floats(&mut sorted);
        let density_estimate = 1.0 / sorted[kth];
        union[i].fitness = raw_fitness + density_estimate;
    }
    (distances, dominators, strengths)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::mocks;

    #[test]
    fn fitness_distances() {
        let mut model = mocks::get_model_basic();
        let (distances, _, _) = set_fitness(&mut model);
        assert_eq!(distances[0][1], 4.0);
        assert_eq!(distances[1][0], 4.0);
        assert_eq!(distances[0][2], 3.0);
        assert_eq!(distances[2][0], 3.0);
        assert_eq!(distances[1][2], 5.0);
        assert_eq!(distances[2][1], 5.0);
    }

    #[test]
    fn fitness_dominators() {
        let mut model = mocks::get_model_basic();
        let (_, dominators, _) = set_fitness(&mut model);

        assert_eq!(dominators[0], vec![]);
        assert_eq!(dominators[1], vec![]);
        assert_eq!(dominators[2], [1]);
    }

    #[test]
    fn fitness_strengths() {
        let mut model = mocks::get_model_basic();
        let (_, _, strengths) = set_fitness(&mut model);
        assert_eq!(strengths[0], 0.0);
        assert_eq!(strengths[1], 1.0);
        assert_eq!(strengths[2], 0.0);
    }

    #[test]
    fn fitness_fitness() {
        let mut model = mocks::get_model_basic();
        set_fitness(&mut model);
        assert_ne!(model.population[0].fitness, 0.0);
        assert_ne!(model.archive[0].fitness, 0.0);
    }
}
