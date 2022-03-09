use quickersort::sort_floats;

pub fn get(union: &Vec<Vec<f32>>) -> (Vec<f32>, Vec<usize>) {
    let decision_variable_count = union.len();
    let population_count = union[0].len() - 1;
    let (strengths, dominators, distances) =
        get_strengths_and_dominators(decision_variable_count, population_count, &union);
    let raw_fitness = get_raw_fitness(population_count, &dominators, &strengths);

    get_fitness_and_non_dominated(population_count, distances, &raw_fitness)
}

fn get_strengths_and_dominators(
    decision_variable_count: usize,
    population_count: usize,
    union: &Vec<Vec<f32>>,
) -> (Vec<f32>, Vec<Vec<usize>>, Vec<Vec<f32>>) {
    let mut strengths = vec![0.0; population_count];
    let mut dominators: Vec<Vec<usize>> = vec![vec![]; population_count];
    let mut distances: Vec<Vec<f32>> = vec![vec![]; population_count];
    let mut i_dom_j;
    let mut j_dom_i;

    for i in 1..=population_count {
        for j in i + 1..=population_count {
            i_dom_j = false;
            j_dom_i = false;

            for k in 0..decision_variable_count {
                let (u1, u2) = if union[k][0] > 0.0 {
                    (union[k][j], union[k][i])
                } else {
                    (union[k][i], union[k][j])
                };
                if u1 < u2 {
                    i_dom_j = true;
                    if j_dom_i {
                        break;
                    }
                } else if u2 < u1 {
                    j_dom_i = true;
                    if i_dom_j {
                        break;
                    }
                } else {
                    i_dom_j = false;
                    j_dom_i = false;
                    break;
                }
            }
            if i_dom_j && !j_dom_i {
                strengths[i - 1] += 1.0;
                dominators[j - 1].push(i - 1);
            } else if j_dom_i && !i_dom_j {
                strengths[j - 1] += 1.0;
                dominators[i - 1].push(j - 1);
            }

            let mut distance: f32 = 0.0;
            for k in 0..decision_variable_count {
                distance += (union[k][i] - union[k][j]).powi(2);
            }
            distance = distance.sqrt();
            distances[i - 1].push(distance);
            distances[j - 1].push(distance);
        }
    }
    (strengths, dominators, distances)
}

fn get_raw_fitness(
    population_count: usize,
    dominators: &Vec<Vec<usize>>,
    strengths: &Vec<f32>,
) -> Vec<f32> {
    let mut raw_fitness = vec![0.0; population_count];
    for i in 0..population_count {
        for j in 0..dominators[i].len() {
            raw_fitness[i] += strengths[dominators[i][j]];
        }
    }
    raw_fitness
}

fn get_fitness_and_non_dominated(
    population_count: usize,
    mut distances: Vec<Vec<f32>>,
    raw_fitness: &Vec<f32>,
) -> (Vec<f32>, Vec<usize>) {
    let kth = (population_count as f32).sqrt() as usize;
    let mut fitness = vec![];
    let mut non_dominated = vec![];
    for i in 0..population_count {
        sort_floats(&mut distances[i][..]);
        let f = raw_fitness[i] + (1.0 / distances[i][kth]);
        fitness.push(f);
        if f < 1.0 {
            non_dominated.push(i);
        }
    }
    (fitness, non_dominated)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_strengths_and_dominators() {
        let population: Vec<Vec<f32>> =
            vec![vec![0.0, 2.0, 3.0, 3.0, 1.0], vec![1.0, 4.0, 3.0, 2.0, 5.0]];
        let (strengths, dominators, _) = get_strengths_and_dominators(2, 4, &population);

        assert_eq!(strengths, [2.0, 0.0, 0.0, 3.0]);

        assert_eq!(dominators[0], [3]);
        assert_eq!(dominators[1], [0, 3]);
        assert_eq!(dominators[2], [0, 3]);
        assert_eq!(dominators[3], []);
    }

    #[test]
    pub fn test_distances() {
        let population: Vec<Vec<f32>> = vec![
            vec![0.0, 0.0, 3.0, 5.0, 7.0],
            vec![1.0, 0.0, 4.0, 12.0, 24.0],
        ];
        let (_, _, distances) = get_strengths_and_dominators(2, 4, &population);

        assert_eq!(distances[0], [5.0, 13.0, 25.0]);
    }

    #[test]
    pub fn test_get_raw_fitness() {
        let strengths = vec![2.0, 0.0, 0.0, 3.0];
        let dominators = vec![vec![3], vec![0, 3], vec![0, 3], vec![]];
        let raw_fitness = get_raw_fitness(4, &dominators, &strengths);

        assert_eq!(raw_fitness[0], 3.0);
        assert_eq!(raw_fitness[1], 5.0);
        assert_eq!(raw_fitness[2], 5.0);
        assert_eq!(raw_fitness[3], 0.0);
    }

    #[test]
    fn test_get_fitness_and_non_dominated() {
        let raw_fitness = vec![0.1, 2.0, 0.5, 4.0];
        let distances = vec![
            vec![1.0, 2.0, 4.0],
            vec![5.0, 6.0, 10.0],
            vec![9.0, 10.0, 100.0],
            vec![10.0, 9.0, 5.0],
        ];

        let (fitness, non_dominated) = get_fitness_and_non_dominated(4, distances, &raw_fitness);

        assert_eq!(fitness[0], 0.35);
        assert_eq!(fitness[1], 2.1);
        assert_eq!(fitness[2], 0.51);
        assert_eq!(fitness[3], 4.1);

        assert_eq!(non_dominated.len(), 2);
        assert_eq!(non_dominated[0], 0);
        assert_eq!(non_dominated[1], 2);
    }
}
