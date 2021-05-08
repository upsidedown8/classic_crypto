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
/// * `msg` A slice of ciphertext code points
/// * `shift_inc` The amount by which key shifts should be incremented
/// * `language` The current language instance
/// * `table_lookup` A function mapping (letter, shift) -> decrypted letter.
///
pub fn vig_solve<F>(
    msg: &[i16],
    shift_inc: usize,
    language: &Language,
    decrypt_one: F,
    is_autokey: bool,
) -> Vec<i16>
where
    F: Fn(i16, i16) -> i16,
{
    let mut best_score = f64::MIN;
    let mut best_key = Vec::new();

    for len in 1..=KEY_LEN_MAX_TEST.min(msg.len()) {
        let mut key = vec![0; len];
        let mut prev_score;
        let mut curr_score = f64::MIN;

        loop {
            prev_score = curr_score;

            for col in 0..len {
                let mut best_shift = key[col];

                // try all shifts for this column
                for shift in (0..26).step_by(shift_inc) {
                    key[col] = shift;
                    let score = if is_autokey {
                        let mut plain = vec![0; len];

                        language.score_iter(
                            msg.iter().enumerate().map(|(idx, &cp)| {
                                plain[idx%len] =
                                    decrypt_one(cp, if idx < len { key[idx] } else { plain[idx%len] });
    
                                plain[idx%len]
                            }),
                            ScoreSize::Quadgrams,
                        )
                    } else {
                        language.score_iter(
                            msg.iter()
                                .enumerate()
                                .map(|(idx, &cp)| decrypt_one(cp, key[idx % len])),
                            ScoreSize::Quadgrams,
                        )
                    };

                    if score > curr_score {
                        curr_score = score;
                        best_shift = shift;
                    }
                }

                key[col] = best_shift;
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
