use rand::Rng;
use crate::lang::Language;

pub trait KeyFrom<T> {
    fn from(language: &Language, _: T) -> Self;
}

pub trait SetKey<T> {
    fn set_key(&mut self, language: &Language, _: T);
}

pub trait Key {
    fn to_string(&self, language: &Language) -> String;

    fn new() -> Self;
}

pub trait StatefulKey {
    fn reset(&mut self);
    fn randomize(&mut self, rnd: &mut impl Rng);
}