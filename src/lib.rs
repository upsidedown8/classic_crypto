//!
//! Implementations of numerous classical ciphers. Supports configuration for
//! an arbitrary `Language` defined in `JSON` config files.
//!

mod cipher;
pub mod key;
pub mod lang;
pub mod util;

pub use cipher::{Asymmetric,Symmetric,Keyed};

pub use cipher::electromechanical::enigma;

pub use cipher::monoalph::affine::Affine;
pub use cipher::monoalph::atbash::Atbash;
pub use cipher::monoalph::baconian::Baconian;
pub use cipher::monoalph::caesar::Caesar;
pub use cipher::monoalph::morse::Morse;
pub use cipher::monoalph::rot13::ROT13;
pub use cipher::monoalph::simple_sub::SimpleSubstitution;

pub use cipher::polyalph::autokey::Autokey;
pub use cipher::polyalph::beaufort::Beaufort;
pub use cipher::polyalph::bellaso::Bellaso;
pub use cipher::polyalph::classic_vig::ClassicVigenere;
pub use cipher::polyalph::keyed_vig::KeyedVigenere;
pub use cipher::polyalph::porta::Porta;
