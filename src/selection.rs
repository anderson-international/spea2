// use crate::{ModelValues, OrderedValue, ARCHIVE_SIZE};

// pub(crate) fn environmental_selection(
//     archive: &mut ModelValues,
//     union: &ModelValues,
//     fitness: Vec<OrderedValue>,
//     non_dominated: Vec<usize>,
//     distances: Vec<Vec<f32>>,
// ) {
//     add_decisions_by_index(archive, union, non_dominated);
//     if archive.count < *ARCHIVE_SIZE {
//         top_up_archive_with_dominated(archive, union, fitness);
//     } else if archive.count > *ARCHIVE_SIZE {
//         truncate_archive_by_distance(archive, distances);
//     }
// }

// fn truncate_archive_by_distance(archive: &mut ModelValues, distances: Vec<Vec<f32>>) {
//     // let mut remove = vec![];
//     // let mut m: (usize, &Vec<f32>) = (0, &vec![]);
//     // // while count > *ARCHIVE_SIZE {
//     // while remove.len() < 3 {
//     //     let (index, min) = distances
//     //         .iter()
//     //         .enumerate()
//     //         .skip(remove.len())
//     //         .reduce(|(i, accum), (j, item)| {
//     //             for (av, iv) in accum.iter().zip(item) {
//     //                 if av < iv {
//     //                     m = (i, accum);
//     //                     break;
//     //                 } else if av > iv {
//     //                     m = (j, item);
//     //                     break;
//     //                 } else {
//     //                     continue;
//     //                 };
//     //             }
//     //             m
//     //         })
//     //         .unwrap();
//     //     println!("{:?}", min);
//     //     remove.push(index);
//     //     println!("{:?}", remove);
//     // }

// }

// fn top_up_archive_with_dominated(
//     archive: &mut ModelValues,
//     union: &ModelValues,
//     mut fitness: Vec<OrderedValue>,
// ) {
//     let mut dominated: Vec<OrderedValue> = fitness.drain(..).filter(|ov| ov.value >= 1.0).collect();
//     println!("{:#?}", dominated);
//     dominated.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
//     let mut len = *ARCHIVE_SIZE - archive.count;
//     if len > dominated.len() {
//         len = dominated.len()
//     };
//     let dominated_indices = dominated[0..len].iter().map(|ov| ov.index).collect();

//     add_decisions_by_index(archive, union, dominated_indices);
// }

// fn add_decisions_by_index(archive: &mut ModelValues, union: &ModelValues, indices: Vec<usize>) {
//     for i in 0..indices.len() {
//         let decision_value_index = indices[i];
//         for j in 0..archive.decisions.len() {
//             archive.decisions[j]
//                 .values
//                 .push(union.decisions[j].values[decision_value_index]);
//         }
//     }
//     archive.count += indices.len();
// }

// #[cfg(test)]
// mod tests {
//     use crate::{DecisionValues, modname::Direction};

//     use super::*;
//     fn get_mock_union() -> ModelValues {
//         let archive = ModelValues {
//             count: 8,
//             decision_count: 2,
//             decisions: vec![
//                 DecisionValues::new(
//                     Direction::Minimised,
//                     vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
//                 ),
//                 DecisionValues::new(
//                     Direction::Maximised,
//                     vec![8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0],
//                 ),
//             ],
//         };
//         archive
//     }
//     fn get_mock_archive() -> ModelValues {
//         let archive = ModelValues {
//             count: 4,
//             decision_count: 2,
//             decisions: vec![
//                 DecisionValues::new(Direction::Minimised, vec![5.0, 6.0, 7.0, 8.0]),
//                 DecisionValues::new(Direction::Maximised, vec![8.0, 7.0, 6.0, 5.0]),
//             ],
//         };
//         archive
//     }
//     fn get_mock_fitness() -> Vec<OrderedValue> {
//         let fitness = vec![
//             OrderedValue {
//                 index: 4,
//                 value: 0.1,
//             },
//             OrderedValue {
//                 index: 3,
//                 value: 0.9,
//             },
//             OrderedValue {
//                 index: 0,
//                 value: 1.1,
//             },
//             OrderedValue {
//                 index: 2,
//                 value: 2.1,
//             },
//         ];
//         fitness
//     }
//     fn get_mock_distances() -> Vec<Vec<f32>> {
//         let distances = vec![
//             vec![4.0, 5.0, 6.0],
//             vec![7.0, 8.0, 9.0],
//             vec![10.0, 11.0, 12.0],
//             vec![1.0, 2.0, 3.0],
//         ];
//         distances
//     }
//     #[test]
//     pub fn test_add_non_dominated_to_archive() {
//         let union = get_mock_union();
//         let mut archive = get_mock_archive();
//         let non_dominated = vec![0, 2];

//         assert_eq!(archive.count, 4);

//         add_decisions_by_index(&mut archive, &union, non_dominated);

//         assert_eq!(archive.count, 6);
//         assert_eq!(archive.decisions[0].values[4], 1.0);
//         assert_eq!(archive.decisions[1].values[4], 8.0);
//         assert_eq!(archive.decisions[0].values[5], 3.0);
//         assert_eq!(archive.decisions[1].values[5], 6.0);

//         println!("{:#?}", archive);
//     }

//     #[test]
//     fn test_top_up_archive_with_dominated() {
//         let union = get_mock_union();
//         let mut archive = get_mock_archive();
//         let fitness = get_mock_fitness();

//         top_up_archive_with_dominated(&mut archive, &union, fitness);

//         assert_eq!(archive.count, 6);
//         assert_eq!(archive.decisions[0].values[4], 1.0);
//         assert_eq!(archive.decisions[1].values[4], 8.0);
//         assert_eq!(archive.decisions[0].values[5], 3.0);
//         assert_eq!(archive.decisions[1].values[5], 6.0);
//     }

//     #[test]
//     fn test_truncate_archive_by_distance() {
//         let mut archive = get_mock_archive();
//         let distances = get_mock_distances();
//         truncate_archive_by_distance(&mut archive, distances);
//     }
// }
