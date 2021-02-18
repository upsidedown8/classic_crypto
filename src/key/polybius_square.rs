// use key::{Key, SetKey, StatefulKey};

// use crate::convert;
// use crate::util;
// use crate::key::key;

// pub struct PolybiusSquare {
//     square: Vec<Vec<i16>>,
//     dim_size: usize,
//     column_labels: Vec<i16>,
//     row_labels: Vec<i16>
// }

// impl PolybiusSquare {
//     fn from_string(square: &String, col_labels: &String, row_labels: &String) -> PolybiusSquare {
//         PolybiusSquare::from_vector(
//             &convert::from_string(square), 
//             &convert::from_string(col_labels), 
//             &convert::from_string(row_labels)
//         )
//     }
//     fn from_vector(square: &Vec<i16>, col_labels: &Vec<i16>, row_labels: &Vec<i16>) -> PolybiusSquare {
//         match square.len() {
//             25 => {
//                 PolybiusSquare {
//                     square: {
//                         let mut square = vec![vec![0; 5]; 5];

//                         square
//                     },
//                     dim_size: 5,
//                     column_labels: col_labels.clone(),
//                     row_labels: row_labels.clone()
//                 }
//             },
//             36 => {
//                 PolybiusSquare {
//                     square: {
//                         let mut square = vec![vec![0; 6]; 6];
                        
//                         square
//                     },
//                     dim_size: 5,
//                     column_labels: col_labels.clone(),
//                     row_labels: row_labels.clone()
//                 }
//             },
//             _ => {
//                 panic!("");
//             }
//         }
//     }
// }

// impl SetKey<&String> for PolybiusSquare {
//     fn set(&mut self, string: &String) {
//         self.value = convert::from_string(&string);
//     }
// }
// impl SetKey<&Vec<i16>> for PolybiusSquare {
//     fn set(&mut self, key: &Vec<i16>) {
//         self.value = key.clone();
//     }
// }

// impl Key for PolybiusSquare {
//     fn to_string(&self) -> String {
//         convert::to_string(&self.value)
//     }
//     fn new() -> PolybiusSquare {
//         PolybiusSquare {
//             value: vec![0]
//         }
//     }
// }

// impl StatefulKey for PolybiusSquare {
//     fn reset(&mut self) {
//         self.value = vec![0];
//     }
//     fn randomize(&mut self, rng: &mut impl rand::Rng) {
//         let length = rng.gen_range(3..12);
//         self.value.resize(length, 0);
//         util::fill_random_array(&mut self.value, rng, 26);
//     }
// }