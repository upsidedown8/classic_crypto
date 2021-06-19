use std::collections::HashMap;

use crate::{
    error::Result,
    key::{IdentityKey, IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
    util,
};

/// Represents a Polybius Square
///
pub struct PolybiusSquare {
    value: Vec<i16>,
    inverse: Vec<i16>,
    row_keys: Vec<i16>,
    col_keys: Vec<i16>,
    row_lookup: HashMap<i16, usize>,
    col_lookup: HashMap<i16, usize>,
    dim_size: usize,
}

impl PolybiusSquare {
    /// Encrypts `cp` and returns a pair of row/col coordinates
    ///
    /// # Arguments
    ///
    /// * `cp` The letter to encrypt
    ///
    pub fn encrypt(&self, cp: i16) -> (i16, i16) {
        let pos = self.inverse[cp as usize] as usize;
        let row = self.value[(pos / self.dim_size) as usize] as usize;
        let col = self.value[(pos % self.dim_size) as usize] as usize;
        (self.row_keys[row], self.col_keys[col])
    }

    /// Decrypts a pair of (`row`, `col`). If the row/col are
    /// not found returns `None`.
    ///
    /// # Arguments
    ///
    /// * `row` The row coordinate of the letter
    /// * `col` The col coordinate of the letter
    ///
    pub fn decrypt(&self, row: i16, col: i16) -> Option<i16> {
        let row_idx = self.row_lookup.get(&row)?;
        let col_idx = self.col_lookup.get(&col)?;
        let pos = *row_idx * self.dim_size + *col_idx;
        Some(self.value[pos])
    }
}
