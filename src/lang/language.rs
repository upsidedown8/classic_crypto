use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};
use crate::lang::LangAlphabet;

use serde::{Deserialize, Serialize};

pub const MAX_ALPHABET_LEN: usize = 32;

const UNIGRAM_LEN: usize = 32;
const BIGRAM_LEN: usize = 32 * 32;
const TRIGRAM_LEN: usize = 32 * 32 * 32;
const QUADGRAM_LEN: usize = 32 * 32 * 32 * 32;

const UNIGRAM_MASK: usize = 0b0000_0000_0000_0001_1111;
const BIGRAM_MASK: usize = 0b0000_0000_0011_1111_1111;
const TRIGRAM_MASK: usize = 0b0000_0111_1111_1111_1111;
const QUADGRAM_MASK: usize = 0b1111_1111_1111_1111_1111;

pub enum ScoreSize {
    Unigrams,
    Bigrams,
    Trigrams,
    Quadgrams,
}

impl ScoreSize {
    /// Returns the length of plaintext substrings that each [`ScoreSize`] uses
    ///
    pub fn length(&self) -> usize {
        match *self {
            ScoreSize::Unigrams => 1,
            ScoreSize::Bigrams => 2,
            ScoreSize::Trigrams => 3,
            ScoreSize::Quadgrams => 4,
        }
    }

    /// Returns the mask to use over 32 bit indexes for this scoring statistic
    ///
    pub fn mask(&self) -> usize {
        match *self {
            ScoreSize::Unigrams => UNIGRAM_MASK,
            ScoreSize::Bigrams => BIGRAM_MASK,
            ScoreSize::Trigrams => TRIGRAM_MASK,
            ScoreSize::Quadgrams => QUADGRAM_MASK,
        }
    }
}

/// Provides compatability with different cipher alphabet lengths, performs conversions
/// from letters to code points (from `0..alphabet_size`), scores plaintext data (Coming soon),
/// recognises letters/punctuation and can be configured from a single binary data file.
///
#[derive(Serialize, Deserialize, Default)]
pub struct Language {
    /// The name of the `Language`
    ///
    pub name: String,

    /// Length of the standard alphabet
    ///
    alphabet_len: i16,

    /// Stores single letter probabilities
    ///
    pub unigrams: Vec<f64>,

    /// Stores double letter probabilities
    ///
    pub bigrams: Vec<f64>,

    /// Stores triple letter probabilities
    ///
    pub trigrams: Vec<f64>,

    /// Stores quadruple letter probabilities
    ///
    pub quadgrams: Vec<f64>,

    /// Stores all supported alphabets
    ///
    alphabets: Vec<LangAlphabet>,

    /// Maps diacritics -> standard letter(s)
    ///
    substitution_table: HashMap<char, String>,

    /// Stores the probability of each code point occuring
    ///
    #[serde(skip)]
    pub unigram_probabilities: Vec<f64>,

    /// The index of [`LangAlphabet`] that is currently selected.
    ///
    #[serde(skip)]
    selected_alph_idx: usize,
}

impl Language {
    /// Creates a new [`Language`] with parameters
    ///
    /// # Arguments
    ///
    /// * `name` The name of the language
    /// * `alphabet_len` The primary alphabet length for the language (english = 26)
    /// * `alphabets` A Vec of [`LangAlphabet`]s, which should contain one with length alphabet_len.
    /// * `corpus` The text corpus to base frequency statistics on.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // read in the text corpus
    /// # use classic_crypto::lang::{Language,LangAlphabet};
    /// let corpus = std::fs::read_to_string("examples/data/corpus.txt").unwrap();
    ///
    /// let lang = Language::new(
    ///     "English".to_string(),
    ///     26,
    ///     vec![
    ///         LangAlphabet::new(
    ///             "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
    ///             "abcdefghijklmnopqrstuvwxyz".to_string(),
    ///             vec![],
    ///             vec![],
    ///             (0..26).collect(),
    ///         ).unwrap(),
    ///         LangAlphabet::new(
    ///             "ABCDEFGHIKLMNOPQRSTUVWXYZ".to_string(),
    ///             "abcdefghiklmnopqrstuvwxyz".to_string(),
    ///             vec!["JI".to_string()],
    ///             vec!["ji".to_string()],
    ///             // since J is missed out
    ///             (0..25).map(|x| if x > 8 {x+1} else {x}).collect(),
    ///         ).unwrap(),
    ///     ],
    ///     corpus
    /// ).unwrap();
    ///
    /// // write to file
    /// let encoded: Vec<u8> = bincode::serialize(&lang).unwrap();
    /// std::fs::write("examples/data/english.bin", encoded).unwrap();
    ///
    /// ```
    ///
    pub fn new(
        name: String,
        alphabet_len: usize,
        alphabets: Vec<LangAlphabet>,
        corpus: String,
    ) -> Result<Language> {
        if let Some(selected_alph_idx) = alphabets.iter().position(|x| x.length() == alphabet_len) {
            let mut lang = Language {
                name,
                alphabets,
                alphabet_len: alphabet_len as i16,
                substitution_table: HashMap::new(),
                unigrams: vec![0.0; UNIGRAM_LEN],
                bigrams: vec![0.0; BIGRAM_LEN],
                trigrams: vec![0.0; TRIGRAM_LEN],
                quadgrams: vec![0.0; QUADGRAM_LEN],
                unigram_probabilities: vec![0.0; UNIGRAM_LEN],
                selected_alph_idx,
            };

            // filter out punctuation from corpus
            let corpus = lang.string_to_vec(&corpus);

            if corpus.len() < 4 {
                return Err(Error::InsufficientCorpusLen { len: corpus.len() });
            }

            // load corpus
            if corpus.len() >= 4 {
                // calculate initial value of idx for first 3 letters
                let mut idx: usize = 0;
                for &cp in corpus.iter().take(3) {
                    idx = (idx << 5) | (cp as usize);
                }
                for &cp in corpus.iter().skip(3) {
                    idx = (idx << 5) | (cp as usize);

                    // increment counts for all 4
                    lang.unigrams[idx & UNIGRAM_MASK] += 1.0;
                    lang.bigrams[idx & BIGRAM_MASK] += 1.0;
                    lang.trigrams[idx & TRIGRAM_MASK] += 1.0;
                    lang.quadgrams[idx & QUADGRAM_MASK] += 1.0;
                }

                // now have frequencies stored. find probabilities
                let uni_sum = lang.unigrams.iter().sum::<f64>();
                let bg_sum = lang.bigrams.iter().sum::<f64>();
                let tg_sum = lang.trigrams.iter().sum::<f64>();
                let qg_sum = lang.quadgrams.iter().sum::<f64>();

                // take the log to exploit
                //      log(a*b)      =     log(a) + log(b)
                //  log(P(A) * P(B))  =  log(P(A)) + log(P(B))
                for i in 0..lang.unigrams.len() {
                    // store probability
                    lang.unigram_probabilities[i] = (lang.unigrams[i] + 1.0) / uni_sum;
                    // store log(probability)
                    lang.unigrams[i] = lang.unigram_probabilities[i].ln();
                }
                for i in 0..lang.bigrams.len() {
                    lang.bigrams[i] = ((lang.bigrams[i] + 1.0) / bg_sum).ln();
                }
                for i in 0..lang.trigrams.len() {
                    lang.trigrams[i] = ((lang.trigrams[i] + 1.0) / tg_sum).ln();
                }
                for i in 0..lang.quadgrams.len() {
                    lang.quadgrams[i] = ((lang.quadgrams[i] + 1.0) / qg_sum).ln();
                }
            }

            // calculate ioc for each alphabet
            for i in 0..lang.alphabets.len() {
                lang.set_alph_len(lang.alphabets[i].length());
                lang.alphabets[i].expected_ioc = lang.index_of_coincedence(&corpus);
            }

            // reset the length afterwards
            lang.set_alph_len(alphabet_len);

            Ok(lang)
        } else {
            Err(Error::AlphabetLenUnmatched {
                expected: alphabet_len,
            })
        }
    }

    /// Creates a new [`Language`] from a binary file
    ///
    /// # Arguments
    ///
    /// * `filename` The path to the language file
    ///
    pub fn from_file(filename: &str) -> Result<Language> {
        Self::from_pathbuf(&PathBuf::from(filename))
    }

    /// Creates a new [`Language`] from a binary file
    ///
    /// # Arguments
    ///
    /// * `path` The PathBuf to the language file
    ///  
    pub fn from_pathbuf(path: &Path) -> Result<Language> {
        if !path.exists() {
            Err(Error::FileNotFound {
                path: path.to_path_buf(),
            })
        } else {
            // read bytes then deserialize
            match File::open(&path) {
                Ok(mut file) => {
                    let mut bytes = Vec::new();

                    match file.read_to_end(&mut bytes) {
                        Ok(..) => {
                            match bincode::deserialize::<Language>(&bytes) {
                                Ok(mut lang) => {
                                    for i in 0..lang.alphabets.len() {
                                        // init all alphabets
                                        lang.alphabets[i].init()?;
                                    }

                                    // init unigram probs
                                    for i in 0..lang.alphabet_len() {
                                        lang.unigram_probabilities.push(lang.unigrams[i].exp());
                                    }

                                    Ok(lang)
                                }
                                Err(err) => Err(Error::CouldntDeserializeFile {
                                    path: path.to_path_buf(),
                                    reason: err,
                                }),
                            }
                        }
                        Err(reason) => Err(Error::CouldntReadFile {
                            path: path.to_path_buf(),
                            reason,
                        }),
                    }
                }
                Err(reason) => Err(Error::CouldntReadFile {
                    path: path.to_path_buf(),
                    reason,
                }),
            }
        }
    }

    /// Gets the currently selected alphabet
    ///
    fn alph(&self) -> &LangAlphabet {
        &self.alphabets[self.selected_alph_idx]
    }

    /// Sets the current length of the alphabet. Returns true if the operation suceeded.
    ///
    /// # Arguments
    ///
    /// * `len` The desired length of the alphabet
    ///
    pub fn set_alph_len(&mut self, len: usize) -> bool {
        if let Some(idx) = self.alphabets.iter().position(|x| x.length() == len) {
            self.selected_alph_idx = idx;
            true
        } else {
            false
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                 Scoring                                    */
    /* -------------------------------------------------------------------------- */
    /// Calculates the Index of Coincedence of a given slice of code points, using the
    /// currently selected alphabet.
    ///
    /// # Arguments
    ///
    /// * `data` The slice to analyse
    ///
    pub fn index_of_coincedence(&self, data: &[i16]) -> f64 {
        let mut counts = [0; MAX_ALPHABET_LEN];

        // count each letter
        for &cp in data {
            counts[cp as usize] += 1;
        }

        let mut total = 0;
        // total f(f-1) where f is the frequency of a particular letter
        for &f in counts.iter().take(self.alphabet_len()) {
            total += f * (f - 1);
        }

        (total as f64) / ((data.len() * (data.len() - 1)) as f64)
    }

    /// Calculates the Index of Coincedence of a given slice of code points given
    /// a particular cipher period, by averaging the IOC for each column. IOC is
    /// calculated using the currently selected alphabet.
    ///
    /// # Arguments
    ///
    /// * `data` The slice to analyse
    /// * `period_length` The length of the cipher period
    ///
    pub fn periodic_ioc(&self, data: &[i16], period_length: usize) -> f64 {
        let mut total = 0.0;

        let mut temp = Vec::new();

        for i in 0..period_length {
            temp.clear();
            for j in (i..data.len()).step_by(period_length) {
                temp.push(data[j]);
            }
            total += self.index_of_coincedence(&temp);
        }

        total / (period_length as f64)
    }

    /// Calculates the chi-squared value for a slice of code points,
    /// comparing it to generated frequency percentages from the corpus.
    ///
    /// # Arguments
    ///
    /// * `data` The slice to analyse
    ///
    pub fn chi_squared(&self, data: &[i16]) -> f64 {
        let mut total = 0.0;
        let mut counts = [0; MAX_ALPHABET_LEN];
        for &cp in data {
            counts[cp as usize] += 1;
        }
        for cp in 0..self.alphabet_len() {
            // convert cp to standard alphabet index
            let i = self.alph().scoring_sub_table[cp] as usize;

            // calculate values
            let expected = self.unigram_probabilities[i] * (data.len() as f64);
            let diff = counts[i] as f64 - expected;
            total += (diff * diff) / expected;
        }
        total
    }

    /// Calculates the unigram, bigram, trigram or quadgram score, based on score_size, of data
    ///
    /// # Arguments
    ///
    /// * `data` The data to score
    /// * `score_size` Which variety of statistic to use
    ///
    pub fn score(&self, data: &[i16], score_size: ScoreSize) -> f64 {
        self.score_iter(data.iter().copied(), score_size)
    }

    /// Calculates the unigram, bigram, trigram or quadgram score, based on score_size, given
    /// an iterator over code points. (Removes the need to use memory to store decryptions).
    ///
    /// # Arguments
    ///
    /// * `iter` An iterator over code points
    /// * `score_size` Which variety of statistic to use
    ///
    pub fn score_iter(&self, mut iter: impl Iterator<Item = i16>, score_size: ScoreSize) -> f64 {
        // initial value of idx
        let mut idx = 0;
        for _ in 0..score_size.length() {
            if let Some(cp) = iter.next() {
                idx = (idx << 5) | (cp as usize);
            }
        }
        // calculate score
        let mut score = 0.0;
        let stats_vec = match score_size {
            ScoreSize::Unigrams => &self.unigrams,
            ScoreSize::Bigrams => &self.bigrams,
            ScoreSize::Trigrams => &self.trigrams,
            ScoreSize::Quadgrams => &self.quadgrams,
        };
        let mask = score_size.mask();
        for cp in iter {
            score += stats_vec[idx & mask];
            idx = (idx << 5) | (self.alph().scoring_sub_table[cp as usize] as usize);
        }

        score
    }

    /* -------------------------------------------------------------------------- */
    /*                             Current Alphabet                               */
    /* -------------------------------------------------------------------------- */
    /// Converts a letter to its code point
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to convert
    ///
    /// # Panics
    ///
    /// If the char provided does not satisfy `self.is_letter()`
    ///
    pub fn get_cp(&self, letter: &char) -> i16 {
        debug_assert!(self.is_letter(letter));

        self.alph().char_to_cp[letter]
    }

    /// Updates the code point of a letter, whilst keeping the correct case
    ///
    /// # Arguments
    ///
    /// * `old_letter` A letter occupying the same space in the string, in the past.
    /// * `new_cp` The new code point to change the letter to
    ///
    /// # Panics
    ///
    /// If the char provided is not a letter (`self.is_letter()`) or `new_cp` does not satisfy `self.valid_cp()`.
    ///
    pub fn update_cp(&self, old_letter: &char, new_cp: i16) -> char {
        debug_assert!(self.valid_cp(new_cp));
        debug_assert!(self.is_letter(old_letter));

        if self.is_upper(old_letter) {
            self.cp_to_upper(new_cp)
        } else {
            self.cp_to_lower(new_cp)
        }
    }

    /// Converts from a code point to its uppercase equivalant
    ///
    /// # Arguments
    ///
    /// * `cp` The code point to convert
    ///
    pub fn cp_to_upper(&self, cp: i16) -> char {
        debug_assert!(self.valid_cp(cp));

        self.alph().upper.chars().nth(cp as usize).unwrap()
    }

    /// Converts from a code point to its lowercase equivalant
    ///
    /// # Arguments
    ///
    /// * `cp` The code point to convert
    ///
    pub fn cp_to_lower(&self, cp: i16) -> char {
        debug_assert!(self.valid_cp(cp));

        self.alph().lower.chars().nth(cp as usize).unwrap()
    }

    /// Gets the expected value for the index of coincedence for the current
    /// alphabet.
    ///
    pub fn expected_ioc(&self) -> f64 {
        self.alph().expected_ioc
    }

    /// Converts a letter to lowercase
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to convert to lowercase
    ///
    pub fn to_lower(&self, letter: &char) -> char {
        if self.is_upper(letter) {
            self.cp_to_lower(self.get_cp(letter))
        } else {
            *letter
        }
    }

    /// Converts a letter to uppercase
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to convert to uppercase
    ///
    pub fn to_upper(&self, letter: &char) -> char {
        if self.is_lower(letter) {
            self.cp_to_upper(self.get_cp(letter))
        } else {
            *letter
        }
    }

    /// Is a particular char a letter?
    ///
    /// # Arguments
    ///
    /// * `letter` The char to check
    ///
    pub fn is_letter(&self, letter: &char) -> bool {
        self.alph().char_to_cp.contains_key(&letter)
    }

    /// Is a particular letter punctuation (not a letter)?
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    pub fn is_punct(&self, letter: &char) -> bool {
        !self.is_letter(letter)
    }

    /// Is a particular letter uppercaes?
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    pub fn is_upper(&self, letter: &char) -> bool {
        self.alph().upper.contains(*letter)
            || self
                .alph()
                .upper_substitutions
                .iter()
                .any(|x| x.chars().next().unwrap() == *letter)
    }

    /// Is a particular letter lowercase?
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    pub fn is_lower(&self, letter: &char) -> bool {
        self.alph().lower.contains(*letter)
            || self
                .alph()
                .lower_substitutions
                .iter()
                .any(|x| x.chars().next().unwrap() == *letter)
    }

    /// Is the code point valid?
    ///
    /// # Arguments
    ///
    /// * `cp` The code point to validate
    ///
    pub fn valid_cp(&self, cp: i16) -> bool {
        0 <= cp && cp <= self.max_cp()
    }

    /// Returns the max value of a code point
    ///
    pub fn max_cp(&self) -> i16 {
        self.cp_count() - 1
    }

    /// Returns the number of distinct code points
    ///
    pub fn cp_count(&self) -> i16 {
        self.alphabet_len() as i16
    }

    /// Returns the length of this alphabet variant
    ///
    pub fn alphabet_len(&self) -> usize {
        self.alph().length()
    }

    /* -------------------------------------------------------------------------- */
    /*                             String conversions                             */
    /* -------------------------------------------------------------------------- */
    /// Converts a borrowed slice of code points to a string. This calls `self.alph().cp_to_upper()` on each
    /// element of the slice.
    ///
    /// # Arguments
    ///
    /// * `arr` The slice to convert
    ///
    pub fn vec_to_string(&self, arr: &[i16]) -> String {
        arr.iter().map(|&i| self.cp_to_upper(i)).collect()
    }

    /// Converts a borrowed slice of a string. This calls cp_to_upper() on each
    /// element of the slice.
    ///
    /// # Arguments
    ///
    /// * `arr` The slice to convert
    ///
    pub fn string_to_vec(&self, string: &str) -> Vec<i16> {
        let mut result = Vec::with_capacity(string.len());

        for ch in string.chars() {
            if self.is_letter(&ch) {
                result.push(self.get_cp(&ch));
            }
        }

        result.truncate(result.len());

        result
    }

    /* -------------------------------------------------------------------------- */
    /*                               Alphabet setup                               */
    /* -------------------------------------------------------------------------- */
    /// Adds an alphabet to the [`Language`]. For more information on alphabets, see [`LangAlphabet`].
    /// Overwrites the existing alphabet if one with the same length already exists.
    ///
    /// # Arguments
    ///
    /// * `alphabet` The alphabet to add
    ///
    pub fn add_alphabet(&mut self, alphabet: LangAlphabet) {
        if let Some(pos) = self
            .alphabets
            .iter()
            .position(|x| x.length() == alphabet.length())
        {
            self.alphabets[pos] = alphabet;
        } else {
            self.alphabets.push(alphabet);
        }
    }

    /// Removes an alphabet from the [`Language`]. For more information on alphabets, see [`LangAlphabet`].
    /// No errors occur if there is no such alphabet in the [`Language`].
    ///
    /// # Arguments
    ///
    /// * `alphabet_len` The length of alphabet to remove
    ///
    pub fn del_alphabet(&mut self, alphabet_len: usize) {
        self.alphabets.retain(|x| x.length() != alphabet_len);
    }

    /* -------------------------------------------------------------------------- */
    /*                          Substitution table setup                          */
    /* -------------------------------------------------------------------------- */
    /// Adds a substitution to the self contained substitution table. Used optionally in preprocessing
    /// for strings before they are sent to ciphers. Use case: expanding ligatures i.e "Æ" -> "AE". Since
    /// the substitution can result in multiple letters, `char`s are mapped to `Strings`, which interfere
    /// with the single character based operations of all ciphers. Hence, substitutions are applied in
    /// a preprocessing stage.
    ///
    /// # Arguments
    ///
    /// * `from` The `char` (ligature) to substitute from
    /// * `to` The string to replace the ligature with
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// language.add_substitution(&'Æ', "AE");
    /// assert_eq!(String::from("AE"), language.substitute_string("Æ"));
    /// ```
    ///
    pub fn add_substitution(&mut self, from: &char, to: &str) {
        debug_assert!(!self.substitution_table.contains_key(from));

        self.substitution_table.insert(*from, String::from(to));
    }

    /// Removes a substitution from the internal substitution table. `from` must already
    /// be a key in the substitution table.
    ///
    /// # Arguments
    ///
    /// * `from` The character that you no longer want to substitute
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// language.add_substitution(&'Æ', "AE");
    /// language.del_substitution(&'Æ');
    /// assert_eq!(String::from("Æ"), language.substitute_string("Æ"));
    /// ```
    ///
    pub fn del_substitution(&mut self, from: &char) {
        debug_assert!(self.substitution_table.contains_key(from));

        self.substitution_table.remove(from);
    }

    /// Clears all ligature substitutions from the internal substitution table
    ///
    pub fn clear_substitutions(&mut self) {
        self.substitution_table.clear();
    }

    /* -------------------------------------------------------------------------- */
    /*                             Substitution table                             */
    /* -------------------------------------------------------------------------- */
    /// Performs the ligature substitutions stored in the internal substitution table to a string.
    /// See `Language::add_substitution()` for more details.
    ///
    /// # Arguments
    ///
    /// * `string` The string to substitute
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// language.add_substitution(&'Æ', "AE");
    /// assert_eq!(String::from("AE"), language.substitute_string("Æ"));
    /// ```
    ///
    pub fn substitute_string(&self, string: &str) -> String {
        let mut result: String = String::new();
        string.chars().for_each(|c| {
            if self.substitution_table.contains_key(&c) {
                result.push_str(&self.substitution_table[&c]);
            } else {
                result.push(c);
            }
        });
        result
    }
}
