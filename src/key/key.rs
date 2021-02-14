use rand::Rng;

pub trait SetKey<T> {
    fn set(&mut self, _: T);
}

pub trait Key {
    fn to_string(&self) -> String;

    fn new() -> Self;
}

pub trait StatefulKey {
    fn reset(&mut self);
    fn randomize(&mut self, rnd: &mut impl Rng);
}