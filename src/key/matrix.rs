use crate::key::{Key, KeyFrom, SetKey, StatefulKey};

use crate::lang::Language;
use crate::util;

/// Represents a Matrix (See Hill Cipher)
///
pub struct Matrix {
    value: Vec<Vec<i16>>,
    dim_size: usize,
}

impl Matrix {
    /// Determinant of a 2x2 matrix
    ///
    /// # Arguments
    ///
    /// * `a` The first element
    /// * `b` The second element
    /// * `c` The third element
    /// * `d` The fourth element
    ///
    fn x2_det(a: i16, b: i16, c: i16, d: i16) -> i16 {
        a * d - b * c
    }

    /// Determinant of a 2x2 matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` The matrix to calculate determinant of
    ///
    fn x2_det_matrix(matrix: &[Vec<i16>]) -> i16 {
        util::modulo(
            Matrix::x2_det(matrix[0][0], matrix[0][1], matrix[1][0], matrix[1][1]),
            26,
        )
    }

    /// MMI of a 2x2 matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` The matrix to calculate MMI of
    ///
    fn x2_inv_matrix(matrix: &[Vec<i16>]) -> Option<i16> {
        util::mmi(Matrix::x2_det_matrix(&matrix), 26)
    }

    /// Determinant of a 3x3 matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` The matrix to calculate determinant of
    ///
    fn x3_det_matrix(matrix: &[Vec<i16>]) -> i16 {
        util::modulo(Matrix::calc_a(matrix) - Matrix::calc_b(matrix), 26)
    }

    /// MMI of a 3x3 matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` The matrix to calculate MMI of
    ///
    fn x3_inv_matrix(matrix: &[Vec<i16>]) -> Option<i16> {
        util::mmi(Matrix::x3_det_matrix(&matrix), 26)
    }

    /// Helper for x3_det_matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` The matrix to calculate A from
    ///
    fn calc_a(matrix: &[Vec<i16>]) -> i16 {
        matrix[0][0] * matrix[1][1] * matrix[2][2]
            + matrix[0][1] * matrix[1][2] * matrix[2][0]
            + matrix[0][2] * matrix[1][0] * matrix[2][1]
    }

    /// Helper for x3_det_matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` The matrix to calculate B from
    ///
    fn calc_b(matrix: &[Vec<i16>]) -> i16 {
        matrix[0][0] * matrix[1][2] * matrix[2][1]
            + matrix[0][1] * matrix[1][0] * matrix[2][2]
            + matrix[0][2] * matrix[1][1] * matrix[2][0]
    }

    /// Inverts the matrix, if possible to do so
    ///
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
                            self.value[0][0],
                        ];

                        inv = Matrix::x2_inv_matrix(&matrix);
                    }
                    3 => {
                        adj = vec![
                            util::modulo(
                                Matrix::x2_det(
                                    self.value[1][1],
                                    self.value[1][2],
                                    self.value[2][1],
                                    self.value[2][2],
                                ),
                                26,
                            ),
                            util::modulo(
                                -Matrix::x2_det(
                                    self.value[0][1],
                                    self.value[0][2],
                                    self.value[2][1],
                                    self.value[2][2],
                                ),
                                26,
                            ),
                            util::modulo(
                                Matrix::x2_det(
                                    self.value[0][1],
                                    self.value[0][2],
                                    self.value[1][1],
                                    self.value[1][2],
                                ),
                                26,
                            ),
                            util::modulo(
                                -Matrix::x2_det(
                                    self.value[1][0],
                                    self.value[1][2],
                                    self.value[2][0],
                                    self.value[2][2],
                                ),
                                26,
                            ),
                            util::modulo(
                                Matrix::x2_det(
                                    self.value[0][0],
                                    self.value[0][2],
                                    self.value[2][0],
                                    self.value[2][2],
                                ),
                                26,
                            ),
                            util::modulo(
                                -Matrix::x2_det(
                                    self.value[0][0],
                                    self.value[0][2],
                                    self.value[1][0],
                                    self.value[1][2],
                                ),
                                26,
                            ),
                            util::modulo(
                                Matrix::x2_det(
                                    self.value[1][0],
                                    self.value[1][1],
                                    self.value[2][0],
                                    self.value[2][1],
                                ),
                                26,
                            ),
                            util::modulo(
                                -Matrix::x2_det(
                                    self.value[0][0],
                                    self.value[0][1],
                                    self.value[2][0],
                                    self.value[2][1],
                                ),
                                26,
                            ),
                            util::modulo(
                                Matrix::x2_det(
                                    self.value[0][0],
                                    self.value[0][1],
                                    self.value[1][0],
                                    self.value[1][1],
                                ),
                                26,
                            ),
                        ];

                        inv = Matrix::x3_inv_matrix(&self.value);
                    }
                    _ => {
                        panic!("dim_size must be either 2 (2x2) or 3 (3x3)");
                    }
                }

                let inv = inv.expect("Failed to calculate matrix inverse");

                for i in 0..self.dim_size {
                    for j in 0..self.dim_size {
                        matrix[i][j] = util::modulo(adj[i * self.dim_size + j] * inv, 26);
                    }
                }

                matrix
            },
            dim_size: self.dim_size,
        }
    }

    /// Is this matrix invertible?
    ///
    pub fn is_invertible(&self) -> bool {
        let inv = match self.dim_size {
            2 => Matrix::x3_inv_matrix(&self.value),
            3 => Matrix::x3_inv_matrix(&self.value),
            _ => {
                panic!("dim_size must be either 2 (2x2) or 3 (3x3)");
            }
        };
        inv != None
    }

    /// Gets the element at (x,y) in the matrix
    ///
    /// # Arguments
    ///
    /// * `x` The horizontal coordinate
    /// * `y` The vertical coordinate
    ///
    pub fn at(&self, x: usize, y: usize) -> i16 {
        self.value[x][y]
    }
}

impl KeyFrom<&String> for Matrix {
    fn create_from(language: &mut Language, string: &String) -> Matrix {
        let arr: Vec<i16> = language.string_to_vec(&string);
        KeyFrom::create_from(language, &arr)
    }
}
impl KeyFrom<&Vec<i16>> for Matrix {
    fn create_from(language: &mut Language, arr: &Vec<i16>) -> Matrix {
        let mut matrix = Matrix::new(language);
        matrix.set_key(language, arr);
        matrix
    }
}
impl KeyFrom<&Vec<Vec<i16>>> for Matrix {
    fn create_from(_language: &mut Language, arr: &Vec<Vec<i16>>) -> Matrix {
        Matrix {
            value: arr.clone(),
            dim_size: arr.len(),
        }
    }
}

impl SetKey<&String> for Matrix {
    fn set_key(&mut self, language: &mut Language, string: &String) {
        let arr = language.string_to_vec(string);
        self.set_key(language, &arr);
    }
}
impl SetKey<&Vec<i16>> for Matrix {
    fn set_key(&mut self, _language: &mut Language, vec: &Vec<i16>) {
        let dim_size = match vec.len() {
            4 => 2,
            9 => 3,
            _ => {
                panic!("Matrix must be either 2x2 (4 letters) or 3x3 (9 letters)");
            }
        };

        self.value = {
            let mut matrix = vec![vec![0; dim_size]; dim_size];
            for i in 0..dim_size {
                for j in 0..dim_size {
                    matrix[i][j] = vec[i * dim_size + j];
                }
            }
            matrix
        };
        self.dim_size = dim_size;
    }
}
impl SetKey<&Vec<Vec<i16>>> for Matrix {
    fn set_key(&mut self, _language: &mut Language, vec: &Vec<Vec<i16>>) {
        self.value = vec.clone();
        self.dim_size = vec.len();
    }
}

impl Key for Matrix {
    fn to_string(&self, language: &mut Language) -> String {
        let mut result = String::new();
        for arr in &self.value {
            result.push_str(language.vec_to_string(&arr).as_str());
        }
        result
    }
    fn new(language: &mut Language) -> Matrix {
        assert_eq!(language.alphabet_len(), 26);
        Matrix {
            value: { vec![vec![0; 2]; 2] },
            dim_size: 2,
        }
    }
}

impl StatefulKey for Matrix {
    fn reset(&mut self, language: &mut Language) {
        assert_eq!(language.alphabet_len(), 26);
        self.value = vec![vec![0; self.dim_size]; self.dim_size];
        for i in 0..self.dim_size {
            self.value[i][i] = 1;
        }
    }
    fn randomize(&mut self, _language: &mut Language, rng: &mut impl rand::Rng) {
        loop {
            for i in 0..self.dim_size {
                for j in 0..self.dim_size {
                    self.value[i][j] = rng.gen_range(0..26);
                }
            }
            if self.is_invertible() {
                break;
            }
        }
    }
}
