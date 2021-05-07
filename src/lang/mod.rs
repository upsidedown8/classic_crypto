//!
//! Module containing structs for generalizing cipher algorithms, so that individual
//! ciphers are only aware of the requirements of the cipher alphabet, such as the
//! number of letters needed.
//!

mod lang_alphabet;
mod language;

pub use lang_alphabet::LangAlphabet;
pub use language::Language;
