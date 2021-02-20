use crate::cipher::Asymmetric;
use crate::lang::Language;

pub struct Baconian { }

impl Asymmetric for Baconian {
    fn encrypt(&self, language: &Language, msg: &String) -> String {
        let mut result = String::new();
        msg
            .chars()
            .filter(|c| language.is_letter(c))
            .for_each(|c| {
                let cp = language.get_cp(&c);
                for i in 0..5 {
                    let mask = 1 << (4-i);
                    let value = (cp & mask) >> (4-i);
                    result.push(language.cp_to_upper(value));
                }
            });
        result
    }
    fn decrypt(&self, language: &Language, msg: &String) -> String {
        let mut result = String::new();
        let mut value = 0;
        let mut count = 0;
        msg
            .chars()
            .filter(|c| language.is_letter(c))
            .map(|c| language.get_cp(&c))
            .filter(|cp| *cp < 2)
            .for_each(|cp| {
                value = (value<<1) | cp;
                count += 1;
                if count >= 5 {
                    result.push(language.cp_to_upper(value));
                    count = 0;
                    value = 0;
                }
            });
        result
    }
}