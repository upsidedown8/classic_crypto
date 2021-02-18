use crate::lang::Language;

trait Cipher {
    fn to_string(&self, language: &Language) -> String;
}

trait Encrypt {
    fn encrypt(&self, language: &Language, msg: &String) -> String;
}

trait Decrypt {
    fn decrypt(&self, language: &Language, msg: &String) -> String;
}

trait Solve {
    fn solve(&self, language: &Language, msg: &String);
}

trait KeyedCipher {
    fn reset(&self, language: &Language);
    fn to_string(&self, language: &Language) -> String;
}