use crate::lang::Language;

pub trait VigSquare {
    fn init_squares(&mut self);

    fn encrypt(&self, x: i16, y: i16) -> i16;
    fn decrypt(&self, x: i16, y: i16) -> i16;
}

pub fn vig_square_to_string(_language: &Language, square: &Vec<Vec<i16>>, max_y: usize, max_x: usize) -> String {
    let mut square_as_string = String::new();

    // left padding
    square_as_string.push_str("  | ");

    // print headers
    for i in 0..26 {
        square_as_string.push((i as u8+65) as char);
        square_as_string.push(' ');
    }
    square_as_string.push_str("\n==|====================================================\n");

    // main square
    for y in 0..max_y {
        square_as_string.push((y as u8+65) as char);
        square_as_string.push_str(" | ");
        for x in 0..max_x {
            square_as_string.push((square[y][x] as u8+65) as char);
            square_as_string.push(' ');
        }
        square_as_string.push('\n')
    }

    square_as_string
}
