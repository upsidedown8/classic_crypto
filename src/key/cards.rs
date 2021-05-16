use crate::{
    error::Result,
    key::{IdentityKey, IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
    util,
};

/// Represents a deck of cards (See Solitaire cipher)
///
pub struct Cards {
    value: Vec<i16>,
    info: KeyInfo,
}

const CARDS_PER_SUITE: i16 = 13;
const A_JOKER: i16 = 52;
const B_JOKER: i16 = 53;

const SUITES: &str = "CDHS";

impl Cards {
    /// Moves the joker one place
    ///
    /// # Arguments
    ///
    /// * `joker` The joker to move (A_JOKER or B_JOKER)
    ///
    fn shift_joker(&mut self, joker: i16) {
        let joker_pos = self.value.iter().position(|&x| x == joker).unwrap();
        match joker_pos {
            53 => {
                let mut i = 53;
                while i > 1 {
                    self.value[i] = self.value[i - 1];
                    i -= 1;
                }
                self.value[1] = joker;
            }
            _ => {
                self.value.swap(joker_pos, joker_pos + 1);
            }
        }
    }

    /// Performs a triple cut on the deck
    ///
    fn triple_cut(&mut self) {
        let mut tmp = vec![0; 54];

        let a_joker_pos = self.value.iter().position(|&x| x == A_JOKER).unwrap();
        let b_joker_pos = self.value.iter().position(|&x| x == B_JOKER).unwrap();

        let min = std::cmp::min(a_joker_pos, b_joker_pos);
        let max = std::cmp::max(a_joker_pos, b_joker_pos);

        let mut idx = 0;
        for i in max + 1..54 {
            tmp[idx] = self.value[i];
            idx += 1;
        }
        for i in min..max + 1 {
            tmp[idx] = self.value[i];
            idx += 1;
        }
        for i in 0..min {
            tmp[idx] = self.value[i];
            idx += 1;
        }

        self.value = tmp;
    }

    /// Performs a count cut on the deck
    ///
    /// # Arguments
    ///
    /// * `length` The length of the count cut
    ///
    ///
    fn count_cut(&mut self, length: i16) {
        if self.value[53] < 52 {
            let mut tmp = vec![0; 54];
            let mut idx = 0;
            for i in length..53 {
                // leave last card
                tmp[idx] = self.value[i as usize];
                idx += 1;
            }
            for i in 0..length {
                tmp[idx] = self.value[i as usize];
                idx += 1;
            }
            self.value.copy_from_slice(&tmp[0..53]);
        }
    }

    /// Calculates the output card of the deck (used for the key stream)
    ///
    fn output_card(&self) -> i16 {
        if self.value[0] < 52 {
            let x = 1 + self.value[(self.value[0] + 1) as usize];
            return std::cmp::min(x, 53);
        }
        // return last card if it is a joker
        std::cmp::min(self.value[53] + 1, 53)
    }

    /// Calculates the key stream of the deck, which is then used to shift
    /// the plaintext letters
    ///
    /// # Arguments
    ///
    /// * `stream_len` The required length of the key stream
    ///
    pub fn key_stream(&mut self, stream_len: usize) -> Vec<i16> {
        let mut stream = vec![0; stream_len];

        let mut idx = 0;
        for _ in 0..stream_len {
            self.shift_joker(A_JOKER);
            self.shift_joker(B_JOKER);
            self.shift_joker(B_JOKER);
            self.triple_cut();
            if self.value[53] < 52 {
                // not a joker
                self.count_cut(self.value[53] + 1);
            }
            let output_card = self.output_card();
            if output_card < 53 {
                stream[idx] = output_card;
                idx += 1;
            }
        }
        stream
    }
}

impl Key<&[i16]> for Cards {
    fn new(_language: &mut Language, arg: &[i16]) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            value: Vec::from(arg),
            info: KeyInfo::default(),
        }))
    }
    fn set(&mut self, _language: &mut Language, arg: &[i16]) -> Result<()> {
        for &card in arg {
            self.shift_joker(A_JOKER);
            self.shift_joker(B_JOKER);
            self.shift_joker(B_JOKER);
            self.triple_cut();
            self.count_cut(self.value[53] + 1);
            self.count_cut(card + 1);
        }
        Ok(())
    }
}
impl Key<&str> for Cards {
    fn new(language: &mut Language, arg: &str) -> Result<Box<Self>> {
        let mut cards = Cards::identity(language);
        cards.set(language, arg)?;
        Ok(Box::new(cards))
    }
    fn set(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        let vec = language.string_to_vec(arg);
        self.set(language, vec.as_slice())
    }
}

impl IdentityKey for Cards {
    fn identity(_language: &mut Language) -> Self {
        Self {
            value: (0..54).collect(),
            info: KeyInfo::default(),
        }
    }
}

impl StatefulKey for Cards {
    fn reset(&mut self, _language: &mut Language) {
        util::fill_consecutive_vec(&mut self.value, 0, 54);
    }
    fn to_string(&self, _language: &mut Language) -> String {
        let mut result = String::new();

        for i in 0..54 {
            match self.value[i] {
                A_JOKER => {
                    result.push_str("JokerA");
                }
                B_JOKER => {
                    result.push_str("JokerB");
                }
                _ => {
                    let pos: usize = (self.value[i] / CARDS_PER_SUITE) as usize;
                    result.push(SUITES.as_bytes()[pos] as char);

                    let num: i16 = (self.value[i] % CARDS_PER_SUITE) + 1;
                    result.push_str(&num.to_string());
                }
            };
            result.push(' ');
        }

        result
    }
    fn randomize(&mut self, _language: &mut Language) {
        util::shuffle(&mut self.value);
    }
}

impl IoKey for Cards {
    fn set_key_str(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        self.set(language, arg)
    }
    fn key_info(&self) -> &KeyInfo {
        &self.info
    }
    fn key_info_mut(&mut self) -> &mut KeyInfo {
        &mut self.info
    }
}
