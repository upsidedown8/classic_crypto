//!
//! Module containing structs for generalizing cipher algorithms, so that individual
//! ciphers are only aware of the requirements of the cipher alphabet, such as the
//! number of letters needed.
//!

mod language;
mod lang_alphabet;

pub use language::Language;
pub use lang_alphabet::LangAlphabet;
