use quickersort::sort_floats;

use crate::sack::Sack;

pub fn get(union: Vec<Sack>) -> Vec<f32> {
    let len = union.len();
    let (strengths, dominators) = get_strengths_and_dominators(len, &union);
    let raw_fitness = get_raw_fitness(len, &dominators, &strengths);
    let distances = get_distances(len, union);
    let fitness = get_fitness(len, distances, &raw_fitness);
    fitness
}

fn get_fitness(len: usize, mut distances: Vec<Vec<f32>>, raw_fitness: &Vec<f32>) -> Vec<f32> {
    let kth = (len as f32).sqrt() as usize;
    let mut fitness = vec![];
    for i in 0..len {
        sort_floats(&mut distances[i][..]);
        let f = raw_fitness[i] + (1.0 / distances[i][kth]);
        fitness.push(f)
    }
    fitness
}

fn get_distances(len: usize, union: Vec<Sack>) -> Vec<Vec<f32>> {
    let mut distances: Vec<Vec<f32>> = vec![vec![]; len];
    for i in 0..len {
        for j in 0..len {
            if j == i {
                continue;
            };
            let d = ((i32::pow(union[i].weight - union[j].weight, 2)
                + i32::pow(union[i].value - union[j].value, 2)) as f32)
                .sqrt();
            distances[i].push(d);
        }
    }
    distances
}

fn get_raw_fitness(len: usize, dominators: &Vec<Vec<usize>>, strengths: &Vec<f32>) -> Vec<f32> {
    let mut raw_fitness = vec![0.0; len];
    for i in 0..len {
        for j in 0..dominators[i].len() {
            raw_fitness[i] += strengths[dominators[i][j]];
        }
    }
    raw_fitness
}

fn get_strengths_and_dominators(len: usize, union: &Vec<Sack>) -> (Vec<f32>, Vec<Vec<usize>>) {
    let mut strengths = vec![0.0; len];
    let mut dominators: Vec<Vec<usize>> = vec![vec![]; len];

    for i in 0..len {
        for j in i + 1..len {
            if union[i].weight < union[j].weight && union[i].value > union[j].value {
                strengths[i] += 1.0;
                dominators[j].push(i);
            } else if union[j].weight < union[i].weight && union[j].value > union[i].value {
                strengths[j] += 1.0;
                dominators[i].push(j);
            }
        }
    }
    (strengths, dominators)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sack::Sack;

    fn mock_sack(weight: i32, value: i32) -> Sack {
        Sack {
            weight,
            value,
            items: vec![],
        }
    }

    #[test]
    pub fn test_get_strengths_and_dominators() {
        let population = vec![
            mock_sack(2, 4),
            mock_sack(3, 3),
            mock_sack(3, 2),
            mock_sack(1, 5),
        ];

        let (strengths, dominators) = get_strengths_and_dominators(4, &population);

        assert_eq!(strengths, [2.0, 0.0, 0.0, 3.0]);

        assert_eq!(dominators[0], [3]);
        assert_eq!(dominators[1], [0, 3]);
        assert_eq!(dominators[2], [0, 3]);
        assert_eq!(dominators[3], []);
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
    fn test_get_distances() {
        let population = vec![
            mock_sack(0, 0),
            mock_sack(3, 4),
            mock_sack(5, 12),
            mock_sack(7, 24),
        ];

        let distances = get_distances(4, population);
        assert_eq!(distances[0], [5.0, 13.0, 25.0]);
    }

    #[test]
    fn test_get_fitness() {
        let raw_fitness = vec![1.0, 2.0, 3.0, 4.0];
        let distances = vec![
            vec![1.0, 2.0, 4.0],
            vec![5.0, 6.0, 10.0],
            vec![9.0, 10.0, 100.0],
            vec![10.0, 9.0, 5.0],
        ];
        //raw_fitness[i] + (1.0 / distances[i][kth]);

        let fitness = get_fitness(4, distances, &raw_fitness);
        println!("{:?}", fitness);
        assert_eq!(fitness[0], 1.25);
        assert_eq!(fitness[1], 2.1);
        assert_eq!(fitness[2], 3.01);
        assert_eq!(fitness[3], 4.1);
    }
}
