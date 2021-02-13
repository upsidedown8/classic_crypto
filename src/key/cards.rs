use key::{Key, SetKey};

use crate::convert;
use crate::util;
use crate::key::key;

pub struct Cards {
    value: Vec<u16>
}

const CARDS_PER_SUITE: u16 = 13;
const A_JOKER: u16 = 52;
const B_JOKER: u16 = 53;

const SUITES: &str = "CDHS";

impl Cards {
    fn shift_joker(&mut self, joker: u16) {
        let joker_pos = self.value
            .iter()
            .position(|&x| {
                x == joker
            })
            .unwrap();
        match joker_pos {
            53 => {
                let mut i = 53;
                while i > 1 {
                    self.value[i] = self.value[i-1];
                    i -= 1;
                }
                self.value[1] = joker;
            },
            _ => {
                util::swap(&mut self.value, joker_pos, joker_pos+1);
            }
        }
    }
    fn triple_cut(&mut self) {
        let mut tmp = vec![0; 54];

        let a_joker_pos = self.value.iter().position(|&x| x == A_JOKER).unwrap();
        let b_joker_pos = self.value.iter().position(|&x| x == B_JOKER).unwrap();

        let min = std::cmp::min(a_joker_pos, b_joker_pos);
        let max = std::cmp::max(a_joker_pos, b_joker_pos);

        let mut idx = 0;
        for i in max+1..54 {
            tmp[idx] = self.value[i];
            idx += 1;
        }
        for i in min..max+1 {
            tmp[idx] = self.value[i];
            idx += 1;
        }
        for i in 0..min {
            tmp[idx] = self.value[i];
            idx += 1;
        }

        self.value = tmp;
    }
    fn count_cut(&mut self, length: u16) {
        if self.value[53] < 52 {
            let mut tmp = vec![0; 54];
            let mut idx = 0;
            for i in length..53 { // leave last card
                tmp[idx] = self.value[i as usize];
                idx += 1;
            }
            for i in 0..length {
                tmp[idx] = self.value[i as usize];
                idx += 1;
            }
            for i in 0..53 { // leave last card
                self.value[i] = tmp[i];
            }
        }
    }

    fn output_card(&self) -> u16 {
        if self.value[0] < 52 {
            let x = 1 + self.value[(self.value[0] + 1) as usize];
            return std::cmp::min(x, 53)
        }
        // return last card if it is a joker
        std::cmp::min(self.value[53]+1, 53)
    }
    pub fn key_stream(&mut self, stream_len: usize) -> Vec<u16> {
        let mut stream = vec![0; stream_len];

        let mut idx = 0;
        for _ in 0..stream_len {
            self.shift_joker(A_JOKER);
            self.shift_joker(B_JOKER);
            self.shift_joker(B_JOKER);
            self.triple_cut();
            if self.value[53] < 52 { // not a joker
                self.count_cut( self.value[53] + 1);
            }
            let output_card = self.output_card();
            if output_card < 53 {
                stream[idx] = output_card;
                idx += 1;
            }
        }
        return stream;
    }
}

impl From<&str> for Cards {
    fn from(string: &str) -> Cards {
        let string = String::from(string);
        Cards::from(&string)
    }
}
impl From<&String> for Cards {
    fn from(string: &String) -> Cards {
        let mut cards = Cards::new();
        cards.set(string);
        cards
    }
}
impl From<&Vec<u16>> for Cards {
    fn from(cards: &Vec<u16>) -> Cards {
        Cards {
            value: cards.clone()
        }
    }
}

impl SetKey<&String> for Cards {
    fn set(&mut self, string: &String) {
        let vec = convert::from_str(&string);
        self.set(&vec);
    }
}
impl SetKey<&Vec<u16>> for Cards {
    fn set(&mut self, vec: &Vec<u16>) {
        for i in 0..vec.len() {
            self.shift_joker(A_JOKER);
            self.shift_joker(B_JOKER);
            self.shift_joker(B_JOKER);
            self.triple_cut();
            self.count_cut(self.value[53] + 1);
            self.count_cut(vec[i] + 1);
        }
    }
}

impl Key for Cards {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for i in 0..54 {
            match self.value[i] {
                A_JOKER => {
                    result.push_str("JokerA");
                },
                B_JOKER => {
                    result.push_str("JokerB");
                },
                _ => {
                    let pos: usize = (self.value[i]/CARDS_PER_SUITE).into();
                    result.push(SUITES.as_bytes()[pos] as char);

                    let num: u16 = (self.value[i]%CARDS_PER_SUITE)+1;
                    result.push_str(&num.to_string());
                }
            };
            result.push(' ');
        }

        result
    }

    fn reset(&mut self) {
        util::fill_consecutive_vec(&mut self.value, 0, 54);
    }
    fn randomize(&mut self, rnd: &mut impl rand::Rng) {
        util::shuffle(&mut self.value, rnd);
    }

    fn new() -> Cards {
        let mut cards = vec![0; 54];
        util::fill_consecutive_vec(&mut cards, 0, 54);
        Cards {
            value: cards
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cards_from_string() {
        let cards_as_string = "H12 H13 S1 S2 C2 D1 S4 S9 S10 S11 S12 C3 C4 C5 C6 S13 C9 C10 C11 C12 C1 D2 D3 C8 JokerA C7 C13 S5 S6 S7 S8 JokerB D4 D5 D6 D7 D8 D9 D10 D11 D12 D13 H1 H2 H3 H4 H5 H6 H7 H8 H9 H10 H11 S3 ";
        assert_eq!(cards_as_string, Cards::from("Keyword").to_string());
    }
}