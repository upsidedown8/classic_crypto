use key::{Key, SetKey, StatefulKey};

use crate::convert;
use crate::util;
use crate::key::key;

pub struct Matrix {
    value: Vec<Vec<i16>>,
    dim_size: usize
}

impl Matrix {
    fn x2_det(a: i16, b: i16, c: i16, d: i16) -> i16 {
        a*d - b*c
    }
    fn x2_det_matrix(matrix: &Vec<Vec<i16>>) -> i16 {
        util::modulo(
            Matrix::x2_det(
                matrix[0][0],
                matrix[0][1],
                matrix[1][0],
                matrix[1][1]
            ),
            26
        )
    }
    fn x2_inv_matrix(matrix: &Vec<Vec<i16>>) -> i16 {
        util::mmi(
            Matrix::x2_det_matrix(&matrix),
            26
        )
    }
    fn x3_det_matrix(matrix: &Vec<Vec<i16>>) -> i16 {
        util::modulo(
            Matrix::calc_a(&matrix) - Matrix::calc_b(&matrix), 
            26
        )
    }
    fn x3_inv_matrix(matrix: &Vec<Vec<i16>>) -> i16 {
        util::mmi(
            Matrix::x3_det_matrix(&matrix),
            26
        )
    }
    fn calc_a(matrix: &Vec<Vec<i16>>) -> i16 {
        matrix[0][0] * matrix[1][1] * matrix[2][2] +
        matrix[0][1] * matrix[1][2] * matrix[2][0] +
        matrix[0][2] * matrix[1][0] * matrix[2][1]
    }
    fn calc_b(matrix: &Vec<Vec<i16>>) -> i16 {
        matrix[0][0] * matrix[1][2] * matrix[2][1] +
        matrix[0][1] * matrix[1][0] * matrix[2][2] +
        matrix[0][2] * matrix[1][1] * matrix[2][0]
    }

    pub fn invert(&self) -> Matrix {
        Matrix {
            value: {
                let mut matrix = vec![vec![0; self.dim_size]; self.dim_size];
                let adj;
                let inv;

                match self.dim_size {
                    2 => {
                        adj = vec![
                            self.value[1][1],
                            util::modulo(-self.value[0][1], 26),
                            util::modulo(-self.value[1][0], 26),
                            self.value[0][0]   
                        ];

                        inv = Matrix::x2_inv_matrix(&matrix);
                    },
                    3 => {
                        adj = vec![
                            util::modulo( Matrix::x2_det(self.value[1][1], self.value[1][2], self.value[2][1], self.value[2][2]), 26),
                            util::modulo(-Matrix::x2_det(self.value[0][1], self.value[0][2], self.value[2][1], self.value[2][2]), 26),
                            util::modulo( Matrix::x2_det(self.value[0][1], self.value[0][2], self.value[1][1], self.value[1][2]), 26),
                        
                            util::modulo(-Matrix::x2_det(self.value[1][0], self.value[1][2], self.value[2][0], self.value[2][2]), 26),
                            util::modulo( Matrix::x2_det(self.value[0][0], self.value[0][2], self.value[2][0], self.value[2][2]), 26),
                            util::modulo(-Matrix::x2_det(self.value[0][0], self.value[0][2], self.value[1][0], self.value[1][2]), 26),
                            
                            util::modulo( Matrix::x2_det(self.value[1][0], self.value[1][1], self.value[2][0], self.value[2][1]), 26),
                            util::modulo(-Matrix::x2_det(self.value[0][0], self.value[0][1], self.value[2][0], self.value[2][1]), 26),
                            util::modulo( Matrix::x2_det(self.value[0][0], self.value[0][1], self.value[1][0], self.value[1][1]), 26)
                        ];
                        
                        inv = Matrix::x3_inv_matrix(&self.value);
                    },
                    _ => {
                        panic!("dim_size must be either 2 (2x2) or 3 (3x3)");
                    }
                }

                for i in 0..self.dim_size {
                    for j in 0..self.dim_size {
                        matrix[i][j] = util::modulo(adj[i*self.dim_size + j] * inv, 26);
                    }
                }

                matrix
            },
            dim_size: self.dim_size
        }
    }

    pub fn is_invertible(&self) -> bool {
        let inv = match self.dim_size {
            2 => {
                Matrix::x3_inv_matrix(&self.value)
            },
            3 => {
                Matrix::x3_inv_matrix(&self.value)
            },
            _ => {
                panic!("dim_size must be either 2 (2x2) or 3 (3x3)");
            }
        };
        inv > 0
    }

    pub fn at(&self, x: usize, y: usize) -> i16 {
        self.value[x][y]
    }
}

impl From<&str> for Matrix {
    fn from(string: &str) -> Matrix {
        let string = String::from(string);
        Matrix::from(&string)
    }
}
impl From<&String> for Matrix {
    fn from(string: &String) -> Matrix {
        let arr = convert::from_str(&string);
        Matrix::from(&arr)
    }
}
impl From<&Vec<i16>> for Matrix {
    fn from(arr: &Vec<i16>) -> Matrix {
        let mut matrix = Matrix::new();
        matrix.set(arr);
        matrix
    }
}
impl From<&Vec<Vec<i16>>> for Matrix {
    fn from(arr: &Vec<Vec<i16>>) -> Matrix {
        Matrix {
            value: arr.clone(),
            dim_size: arr.len()
        }
    }
}

impl SetKey<&String> for Matrix {
    fn set(&mut self, string: &String) {
        let arr = convert::from_string(string);
        self.set(&arr);
    }
}
impl SetKey<&Vec<i16>> for Matrix {
    fn set(&mut self, vec: &Vec<i16>) {
        let dim_size = match vec.len() {
            4 => { 2 },
            9 => { 3 },
            _ => { panic!("Matrix must be either 2x2 (4 letters) or 3x3 (9 letters)"); }
        };

        self.value = {
            let mut matrix = vec![vec![0; dim_size]; dim_size];
            for i in 0..dim_size {
                for j in 0..dim_size {
                    matrix[i][j] = vec[i*dim_size + j];
                }
            }
            matrix
        };
        self.dim_size = dim_size;
    }
}
impl SetKey<&Vec<Vec<i16>>> for Matrix {
    fn set(&mut self, vec: &Vec<Vec<i16>>) {
        self.value = vec.clone();
        self.dim_size = vec.len();
    }
}

impl Key for Matrix {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for arr in &self.value {
            result.push_str(convert::to_string(&arr).as_str());
        }
        result
    }
    fn new() -> Matrix {
        Matrix {
            value: {
                vec![vec![0; 2]; 2]
            },
            dim_size: 2
        }
    }
}

impl StatefulKey for Matrix {
    fn reset(&mut self) {
        self.value = vec![vec![0; self.dim_size]; self.dim_size];
        for i in 0..self.dim_size {
            self.value[i][i] = 1;
        }
    }
    fn randomize(&mut self, rnd: &mut impl rand::Rng) {
        loop {
            for i in 0..self.dim_size {
                for j in 0..self.dim_size {
                    self.value[i][j] = rnd.gen_range(0..26);
                }
            }
            if self.is_invertible() { break; }
        }
    }
}