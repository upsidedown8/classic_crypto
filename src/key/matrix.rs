use crate::{
    error::{Error, Result},
    key::{IdentityKey, IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
    util,
};

/// The dimension size of a matrix (2x2 or 3x3)
///
#[derive(Clone, Copy)]
pub enum MatrixDimSize {
    Two = 2,
    Three = 3,
}

/// Represents a Matrix (See Hill Cipher)
///
pub struct Matrix {
    pub size: MatrixDimSize,
    value: Vec<i16>,
    info: KeyInfo,
}

impl Matrix {
    /// Determinant of a 2x2 matrix
    ///
    /// # Arguments
    ///
    /// * `matrix` The matrix to find det from
    ///
    fn x2_det(matrix: &[i16]) -> i16 {
        matrix[0] * matrix[3] - matrix[1] * matrix[2]
    }

    /// Calculate the determinant of the matrix
    ///
    /// # Arguments
    ///
    /// `matrix` The matrix to calculate determinant from
    ///
    fn det(matrix: &[i16], language: &Language) -> i16 {
        match matrix.len() {
            4 => Matrix::x2_det(matrix),
            9 => {
                let mut det = 0;

                // calculate determinant from top row
                let mut x2_mat = vec![0; 4];
                for col in 0..3 {
                    // for each position, find 2x2 determinant
                    let mut i = 0;
                    for r in 1..3 {
                        for c in (0..3).filter(|x| *x != col) {
                            x2_mat[i] = util::modulo(matrix[r * 3 + c], language.cp_count());
                            i += 1;
                        }
                    }

                    det += Matrix::x2_det(&x2_mat) * if col == 1 { -1 } else { 1 } * matrix[col];
                }

                det
            }
            _ => unreachable!("Matrix must be 2x2 or 3x3"),
        }
    }

    /// Inverts the matrix, if possible to do so
    ///
    pub fn invert(&self, language: &Language) -> Matrix {
        Matrix {
            info: KeyInfo::default(),
            size: self.size,
            value: {
                let mut adj = vec![0; self.size as usize * self.size as usize];

                match self.size {
                    MatrixDimSize::Two => {
                        adj[0] = self.value[3];
                        adj[1] = util::modulo(-self.value[1], 26);
                        adj[2] = util::modulo(-self.value[2], 26);
                        adj[3] = self.value[0];
                    }
                    MatrixDimSize::Three => {
                        let mut x2_mat = vec![0; 4];
                        // iterate through all matrix positions
                        for row in 0..3 {
                            for col in 0..3 {
                                // for each position, find 2x2 determinant
                                let mut i = 0;
                                for r in (0..3).filter(|x| *x != row) {
                                    for c in (0..3).filter(|x| *x != col) {
                                        x2_mat[i] = self.value[r * 3 + c];
                                        i += 1;
                                    }
                                }

                                let pos = col * 3 + row;
                                adj[pos] = Matrix::x2_det(&x2_mat);
                                adj[pos] *= if (row * 3 + col) % 2 == 0 { 1 } else { -1 };
                            }
                        }
                    }
                }

                let det = util::modulo(Matrix::det(&self.value, language), language.cp_count());
                let inv = util::mmi(det, language.cp_count())
                    .expect("Failed to calculate matrix inverse");

                (0..self.size as usize * self.size as usize)
                    .map(|i| util::modulo(adj[i] * inv, language.cp_count()))
                    .collect()
            },
        }
    }

    /// Is this matrix invertible?
    ///
    /// # Arguments
    ///
    /// `language` A [`Language`] instance
    ///
    pub fn is_invertible(&self, language: &Language) -> bool {
        util::mmi(Matrix::det(&self.value, language), language.cp_count()).is_some()
    }

    /// Gets the element at (row, col) in the matrix
    ///
    /// # Arguments
    ///
    /// * `row` The row
    /// * `col` The column
    ///
    #[inline(always)]
    pub fn at(&self, row: usize, col: usize) -> i16 {
        self.value[row * self.size as usize + col]
    }
}

impl Key<&[i16]> for Matrix {
    fn new(language: &mut Language, arg: &[i16]) -> Result<Box<Self>> {
        let mut result = Matrix::identity(language);
        result.set(language, arg)?;
        Ok(Box::new(result))
    }
    fn set(&mut self, language: &mut Language, arg: &[i16]) -> Result<()> {
        if arg.len() != 4 && arg.len() != 9 {
            Err(Error::InvalidKeyFmt {
                expected: "Expected 4 or 9 values".to_string(),
                actual: format!("{} values, data: {:?}", arg.len(), arg),
            })
        } else {
            let val = Vec::from(arg);

            if util::mmi(Matrix::det(&val, &language), language.cp_count()).is_some() {
                Err(Error::InvalidKeyFmt {
                    expected: "Matrix to have an inverse".to_string(),
                    actual: format!("{:?}", arg),
                })
            } else {
                self.value = val;
                self.size = match arg.len() {
                    4 => MatrixDimSize::Two,
                    9 => MatrixDimSize::Three,
                    _ => unreachable!(),
                };

                Ok(())
            }
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
    fn identity(_language: &mut Language) -> Self {
        Self {
            value: { vec![0; 4] },
            size: MatrixDimSize::Two,
            info: KeyInfo::default(),
        }
    }
}

impl StatefulKey for Matrix {
    fn reset(&mut self, _language: &mut Language) {
        self.value = vec![0; self.size as usize * self.size as usize];
        for i in 0..self.size as usize {
            self.value[(self.size as usize + 1) * i] = 1;
        }
    }
    fn to_string(&self, language: &mut Language) -> String {
        self.value
            .iter()
            .map(|&item| language.cp_to_upper(item))
            .fold(
                String::with_capacity(self.size as usize * self.size as usize),
                |mut acc, ch| {
                    acc.push(ch);
                    acc
                },
            )
    }
    fn randomize(&mut self, language: &mut Language) {
        loop {
            for i in 0..self.size as usize * self.size as usize {
                self.value[i] = fastrand::i16(0..language.cp_count());
            }
            if self.is_invertible(language) {
                break;
            }
        }
    }
}

impl IoKey for Matrix {
    fn set_key_str(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        self.set(language, arg)
    }
    fn info(&self) -> &KeyInfo {
        &self.info
    }
    fn info_mut(&mut self) -> &mut KeyInfo {
        &mut self.info
    }
    fn desc(&self) -> String {
        "<4 or 9 letter string>".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x2_inv() {
        let mut language = Language::from_file("examples/data/english.bin").unwrap();
        let mat = *Matrix::new(&mut language, "hill").unwrap();

        assert_eq!(vec![25, 22, 1, 23], mat.invert(&language).value);
    }

    #[test]
    fn x3_inv() {
        let mut language = Language::from_file("examples/data/english.bin").unwrap();
        let mat = *Matrix::new(&mut language, "AlphaBeta").unwrap();

        assert_eq!(
            vec![3, 7, 1, 24, 4, 19, 5, 4, 19],
            mat.invert(&language).value
        );
    }
}
