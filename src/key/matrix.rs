use rand::Rng;

use crate::{
    error::{Error, Result},
    key::{IdentityKey, IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
    util,
};

/// Represents a Matrix (See Hill Cipher)
///
pub struct Matrix {
    value: Vec<Vec<i16>>,
    dim_size: usize,
    info: KeyInfo,
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
            info: KeyInfo::default(),
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
    #[inline(always)]
    pub fn at(&self, x: usize, y: usize) -> i16 {
        self.value[x][y]
    }
}

impl Key<&[i16]> for Matrix {
    fn new(language: &mut Language, arg: &[i16]) -> Result<Box<Self>> {
        let mut result = Matrix::identity(language);
        result.set(language, arg)?;
        Ok(Box::new(result))
    }
    fn set(&mut self, _language: &mut Language, arg: &[i16]) -> Result<()> {
        if arg.len() != 4 && arg.len() != 9 {
            Err(Error::InvalidKeyFmt {
                expected: "Expected 4 or 9 values".to_string(),
                actual: format!("{} values, data: {:?}", arg.len(), arg),
            })
        } else {
            let dim_size = match arg.len() {
                4 => 2,
                9 => 3,
                _ => unreachable!(),
            };

            self.value = {
                let mut matrix = vec![vec![0; dim_size]; dim_size];
                for i in 0..dim_size {
                    for j in 0..dim_size {
                        matrix[i][j] = arg[i * dim_size + j];
                    }
                }
                matrix
            };
            self.dim_size = dim_size;

            Ok(())
        }
    }
}
impl Key<&Vec<Vec<i16>>> for Matrix {
    fn new(language: &mut Language, arg: &Vec<Vec<i16>>) -> Result<Box<Self>> {
        let mut result = Matrix::identity(language);
        result.set(language, arg)?;
        Ok(Box::new(result))
    }
    fn set(&mut self, _language: &mut Language, arg: &Vec<Vec<i16>>) -> Result<()> {
        if arg.is_empty() {
            Err(Error::InvalidKeyFmt {
                expected: "Not empty".to_string(),
                actual: "Empty vec".to_string(),
            })
        } else if arg.len() != arg[0].len() {
            Err(Error::InvalidKeyFmt {
                expected: "Square matrix".to_string(),
                actual: format!("rows ({}) != cols ({})", arg.len(), arg[0].len()),
            })
        } else if !(2..=3).contains(&arg.len()) {
            Err(Error::InvalidKeyFmt {
                expected: "Matrix must be 2x2 or 3x3".to_string(),
                actual: format!("rows: {} cols: {}", arg.len(), arg[0].len()),
            })
        } else {
            self.value = arg.clone();
            self.dim_size = arg.len();
            Ok(())
        }
    }
}
impl Key<&str> for Matrix {
    fn new(language: &mut Language, arg: &str) -> Result<Box<Self>> {
        let arr = language.string_to_vec(arg);
        Matrix::new(language, arr.as_slice())
    }
    fn set(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        let arr = language.string_to_vec(arg);
        self.set(language, arr.as_slice())?;
        Ok(())
    }
}

impl IdentityKey for Matrix {
    fn identity(language: &mut Language) -> Self {
        language.set_alph_len(26);
        Self {
            value: { vec![vec![0; 2]; 2] },
            dim_size: 2,
            info: KeyInfo::default(),
        }
    }
}

impl StatefulKey for Matrix {
    fn reset(&mut self, language: &mut Language) {
        language.set_alph_len(26);
        self.value = vec![vec![0; self.dim_size]; self.dim_size];
        for i in 0..self.dim_size {
            self.value[i][i] = 1;
        }
    }
    fn to_string(&self, language: &mut Language) -> String {
        self.value
            .iter()
            .map(|row| language.vec_to_string(row))
            .fold(String::new(), |acc, row| row + &acc)
    }
    fn randomize(&mut self, _language: &mut Language) {
        loop {
            for i in 0..self.dim_size {
                for j in 0..self.dim_size {
                    self.value[i][j] = rand::thread_rng().gen_range(0..26);
                }
            }
            if self.is_invertible() {
                break;
            }
        }
    }
}

impl IoKey for Matrix {
    fn set_key_str(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        self.set(language, arg)
    }
    fn key_info(&self) -> &KeyInfo {
        &self.info
    }
    fn key_info_mut(&mut self) -> &mut KeyInfo {
        &mut self.info
    }
}
