pub mod bellaso_square;
pub mod classic_vig_square;
pub mod keyed_vig_square;
pub mod porta_square;

use crate::lang::Language;

pub trait VigSquare {
    fn init_squares(&mut self);

    fn encrypt(&self, x: i16, y: i16) -> i16;
    fn decrypt(&self, x: i16, y: i16) -> i16;
}

pub fn vig_square_to_string(
    language: &Language,
    square: &[Vec<i16>],
    max_y: usize,
    max_x: usize,
) -> String {
    assert_eq!(language.alphabet_len(), 26);
    let mut square_as_string = String::new();

    let is_porta = max_y == 13;

    // left padding
    square_as_string.push_str(if is_porta { "    | " } else { "  | " });

    // print headers
    for i in 0..26 {
        square_as_string.push(language.cp_to_upper(i));
        square_as_string.push(' ');
    }
    if is_porta {
        square_as_string.push_str("\n====|====================================================\n");
    } else {
        square_as_string.push_str("\n==|====================================================\n");
    }

    // main square
    for y in 0..max_y as i16 {
        if is_porta {
            square_as_string.push(language.cp_to_upper(y / 2));
            square_as_string.push(',');
            square_as_string.push(language.cp_to_upper(y / 2 + 1));
        } else {
            square_as_string.push(language.cp_to_upper(y));
        }
        square_as_string.push_str(" | ");
        for x in 0..max_x {
            square_as_string.push(language.cp_to_upper(square[y as usize][x]));
            square_as_string.push(' ');
        }
        square_as_string.push('\n')
    }

    square_as_string
}
