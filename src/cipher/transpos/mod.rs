use crate::lang::{Language, ScoreSize};

pub mod block_transpos;
pub mod column_transpos;
pub mod myszkowski;
pub mod railfence;
pub mod scytale;

/// Solves a block/column transposition depending on arguments
///
/// # Arguments
///
/// * `ciphertext` A slice containing the ciphertext code points
/// * `language` A [`Language`] instance
/// * `decrypt_indexes` A function of type: (len, key_order) -> decrypt_indexes
/// * `get_index` A function of type: (row, col, key_len, num_rows) -> index
///
pub fn transposition_solve<F, G>(
    ciphertext: &[i16],
    language: &mut Language,
    decrypt_indexes: F,
    get_index: G,
) -> Vec<i16>
where
    F: Fn(usize, Vec<usize>) -> Vec<usize>,
    G: Fn(usize, usize, usize, usize) -> usize,
{
    let len = ciphertext.len();

    // store best score seen
    let mut best_score = f64::MIN;
    let mut best_key = Vec::new();

    // try all key lengths
    for key_len in 3.min(len)..15.min(len) {
        // ignore any extra above a multiple of key_len
        let num_rows = len / key_len;

        for start_col in 0..key_len {
            let mut key = vec![start_col];

            while key.len() < key_len {
                let col1 = key.last().copied().unwrap();

                let mut max_score = f64::MIN;
                let mut max_col = 0;

                for col2 in (0..key_len).filter(|x| !key.contains(x)) {
                    let mut total_score = 0.0;

                    for row in 0..num_rows {
                        let col1_letter = ciphertext[get_index(row, col1, key_len, num_rows)];
                        let col2_letter = ciphertext[get_index(row, col2, key_len, num_rows)];

                        total_score +=
                            language.bigrams[((col1_letter << 5) | col2_letter) as usize];
                    }

                    if total_score > max_score {
                        max_score = total_score;
                        max_col = col2;
                    }
                }

                key.push(max_col);
            }

            let score = language.score_iter(
                decrypt_indexes(len, key.clone())
                    .into_iter()
                    .map(|idx| ciphertext[idx]),
                ScoreSize::Quadgrams,
            );

            if score > best_score {
                best_score = score;
                best_key = key.clone();
            }
        }
    }

    best_key.iter().map(|x| *x as i16).collect()
}
