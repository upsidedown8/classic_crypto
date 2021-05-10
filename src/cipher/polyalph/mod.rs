use crate::lang::{Language, ScoreSize};

pub mod autokey;
pub mod beaufort;
pub mod bellaso;
pub mod chaocipher;
pub mod classic_vig;
pub mod keyed_vig;
pub mod porta;

const KEY_LEN_MAX_TEST: usize = 30;

/// Solve a vigenere cipher
///
/// # Arguments
///
/// * `ciphertext` A slice of ciphertext code points
/// * `shift_inc` The amount by which key shifts should be incremented
/// * `language` The current language instance
/// * `decrypt_one` A function mapping (letter, shift) -> decrypted letter.
/// * `get_shift` A function mapping (key, idx, key_len, plaintext) -> shift
///
pub fn vig_solve<F, G>(
    ciphertext: &[i16],
    shift_inc: usize,
    language: &Language,
    decrypt_one: F,
    get_shift: G,
) -> Vec<i16>
where
    F: Fn(i16, i16) -> i16,
    G: Fn(&Vec<i16>, usize, usize, &Vec<i16>) -> i16,
{
    let mut best_score = f64::MIN;
    let mut best_key = Vec::new();
    let mut plaintext = vec![0; ciphertext.len()];

    for key_len in 1..=KEY_LEN_MAX_TEST.min(ciphertext.len()) {
        let mut key = vec![0; key_len];
        let mut prev_score;
        let mut curr_score = f64::MIN;

        // initial decrypt
        for idx in 0..ciphertext.len() {
            plaintext[idx] =
                decrypt_one(ciphertext[idx], get_shift(&key, idx, key_len, &plaintext));
        }

        loop {
            prev_score = curr_score;

            for col in 0..key_len {
                let mut best_shift = key[col];

                // try all shifts for this column
                for shift in (0..26).step_by(shift_inc) {
                    key[col] = shift;

                    // incremental decrypt
                    for idx in (col..ciphertext.len()).step_by(key_len) {
                        plaintext[idx] =
                            decrypt_one(ciphertext[idx], get_shift(&key, idx, key_len, &plaintext));
                    }

                    let score = language.score(&plaintext, ScoreSize::Quadgrams);

                    if score > curr_score {
                        curr_score = score;
                        best_shift = shift;
                    }
                }

                key[col] = best_shift;

                // decrypt using the best shift so far
                for idx in (col..ciphertext.len()).step_by(key_len) {
                    plaintext[idx] =
                        decrypt_one(ciphertext[idx], get_shift(&key, idx, key_len, &plaintext));
                }
            }

            // exit if no improvement
            if (prev_score - curr_score).abs() < 0.1 {
                break;
            }
        }

        // check whether this is better than previous keys
        if curr_score > best_score {
            best_score = curr_score;
            best_key = key;
        }
    }

    best_key
}
