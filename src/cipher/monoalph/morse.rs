use crate::cipher::Asymmetric;
use crate::lang::Language;

const LETTERS: [char; 37] = [
    ' ', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
];
const MORSE: [&str; 37] = [
    "/", ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
    "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
    "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.",
    "-----",
];

pub struct Morse {}

impl Asymmetric for Morse {
    fn encrypt(&self, _language: &Language, msg: &String) -> String {
        msg.to_lowercase()
            .chars()
            .filter(|c| LETTERS.contains(c))
            .map(|c| MORSE[LETTERS.iter().position(|x| *x == c).unwrap()])
            .fold(String::new(), |mut acc, x| {
                acc.push_str(x);
                acc.push(' ');
                acc
            })
            .trim()
            .to_string()
    }
    fn decrypt(&self, _language: &Language, msg: &String) -> String {
        msg.split('/')
            .map(|word| {
                word.replace('_', "-")
                    .trim()
                    .split(' ')
                    .map(|letter| MORSE.iter().position(|&x| x == letter))
                    .filter(|&x| x != None)
                    .map(|pos| LETTERS[pos.unwrap()])
                    .fold(String::new(), |mut acc, x| {
                        acc.push(x);
                        acc
                    })
            })
            .fold(String::new(), |mut acc, x| {
                acc.push_str(&x);
                acc.push(' ');
                acc
            })
            .trim()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
        let msg = String::from("morse code");
        let language = Language::new();
        let morse = Morse {};
        let encrypted = morse.encrypt(&language, &msg);
        assert_eq!(encrypted, "-- --- .-. ... . / -.-. --- -.. .");
    }

    #[test]
    fn decrypt() {
        let msg = String::from("-- --- .-. ... . / -.-. --- -.. .");
        let language = Language::new();
        let morse = Morse {};
        let decrypted = morse.decrypt(&language, &msg);
        assert_eq!(decrypted, "morse code");
    }
}
