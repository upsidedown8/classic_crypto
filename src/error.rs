use std::path::PathBuf;

use bincode::ErrorKind;

/// An alias of std::result::Result<T, E>, where E is the library [`Error`] type
///
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The Error type for the library
///
#[derive(Debug)]
pub enum Error {
    /// The path to a language file does not exist
    FileNotFound { path: PathBuf },

    /// Couldn't read from a language file
    CouldntReadFile {
        path: PathBuf,
        reason: std::io::Error,
    },

    /// Failed to deserialize a binary language file
    CouldntDeserializeFile {
        path: PathBuf,
        reason: Box<ErrorKind>,
    },

    /// The Upper/Lower alphabets for an alphabet have different lengths in chars
    /// but would expect equal length
    AlphabetLenDifference { upper_len: usize, lower_len: usize },

    /// The minimum number of characters defined in upper/lower alphabets (4) were not found
    /// in the corpus supplied to the Language
    InsufficientCorpusLen { len: usize },

    /// The expected alphabet len (given as the length of the primary alphabet of the language)
    /// was not found in any of the LangAlphabet s supplied to the Language
    AlphabetLenUnmatched { expected: usize },

    /// Expect an alphabet to have equal length to its substitution table (used for scoring)
    ScoringSubTableLen {
        alphabet_len: usize,
        table_len: usize,
    },

    /// The maximum alphabet length is 32
    MaxAlphabetLenExceeded { alphabet_len: usize },

    /// The uppercase alphabet has at least one character that is repeated
    RepeatCharUpperAlph { upper: String },

    /// The lowercase alphabet has at least one character that is repeated
    RepeatCharLowerAlph { lower: String },

    /// The substitution table expects a Vec of letters in pairs
    SubstitutionsNotPairs { subs: Vec<String> },

    /// No letter should be repeated in the substitutions
    SubstitutionsNotUnique { subs: Vec<String> },

    /// Only letters defined in the respective alphabet should appear in the substitutions
    InvalidCharsInSubstitutions { subs: Vec<String> },

    /// Couldn't Write To Stdout
    CouldntWriteToStdout,

    /// Occurs when not enough text was input to a function
    InsufficientInputLen { expected: usize, actual: usize },
}
