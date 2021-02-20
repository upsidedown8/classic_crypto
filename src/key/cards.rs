use std::usize;

use key::{Key, SetKey, StatefulKey, KeyFrom};

use crate::lang::Language;
use crate::util;
use crate::key;

pub struct Cards {
    value: Vec<i16>
}

const CARDS_PER_SUITE: i16 = 13;
const A_JOKER: i16 = 52;
const B_JOKER: i16 = 53;

const SUITES: &str = "CDHS";

impl Cards {
    fn shift_joker(&mut self, joker: i16) {
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
    fn count_cut(&mut self, length: i16) {
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

    fn output_card(&self) -> i16 {
        if self.value[0] < 52 {
            let x = 1 + self.value[(self.value[0] + 1) as usize];
            return std::cmp::min(x, 53)
        }
        // return last card if it is a joker
        std::cmp::min(self.value[53]+1, 53)
    }
    pub fn key_stream(&mut self, stream_len: usize) -> Vec<i16> {
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

impl KeyFrom<&String> for Cards {
    fn create_from(language: &Language, string: &String) -> Cards {
        let mut cards = Cards::new(language);
        cards.set_key(language, string);
        cards
    }
}
impl KeyFrom<&Vec<i16>> for Cards {
    fn create_from(_language: &Language, cards: &Vec<i16>) -> Cards {
        Cards {
            value: cards.clone()
        }
    }
}

impl SetKey<&String> for Cards {
    fn set_key(&mut self, language: &Language, string: &String) {
        let vec = language.string_to_vec(&string);
        self.set_key(language, &vec);
    }
}
impl SetKey<&Vec<i16>> for Cards {
    fn set_key(&mut self, _language: &Language, vec: &Vec<i16>) {
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
    fn to_string(&self, _language: &Language) -> String {
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
                    let pos: usize = (self.value[i]/CARDS_PER_SUITE) as usize;
                    result.push(SUITES.as_bytes()[pos] as char);

                    let num: i16 = (self.value[i]%CARDS_PER_SUITE)+1;
                    result.push_str(&num.to_string());
                }
            };
            result.push(' ');
        }

        result
    }
    fn new(_language: &Language) -> Cards {
        let mut cards = vec![0; 54];
        util::fill_consecutive_vec(&mut cards, 0, 54);
        Cards {
            value: cards
        }
    }
}

impl StatefulKey for Cards {
    fn reset(&mut self, _language: &Language) {
        util::fill_consecutive_vec(&mut self.value, 0, 54);
    }
    fn randomize(&mut self, _language: &Language, rng: &mut impl rand::Rng) {
        util::shuffle(&mut self.value, rng);
    }
}
