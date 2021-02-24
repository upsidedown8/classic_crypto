# Classic Crypto
![Build Status](https://github.com/upsidedown8/classic_crypto/actions/workflows/rust.yml/badge.svg)
### Implementations of numerous classical cryptographic algorithms in rust, with a focus on configurable support for any language/charset. Currently only 26 letter alphabets are fully supported, but this should change in later releases.

---

## Usage

Add this line to your `Cargo.toml`

```toml
classic_crypto = "0.1.0"
```
---

## Example

```rust
use classic_crypto::{lang::Language, Affine, Asymmetric, Keyed};

fn main() {
    let contents = std::fs::read_to_string("examples/data/english.json")
        .expect("Something went wrong reading the file");
    let language: Language = serde_json::from_str(&contents).unwrap();

    let mut cipher = Affine::new(&language);
    cipher.randomize(&language, &mut rand::thread_rng());

    let plaintext = "Some plaintext";
    let ciphertext = cipher.encrypt(&language, plaintext);

    println!("{}", ciphertext);
}
```

---


## Supported Ciphers

### Electromechanical
 - [x] Enigma M3/M4
 - [ ] Fialka
 - [ ] Lorenz
 - [ ] Purple

### Monoalphabetic
 - [x] Affine
 - [x] Atbash
 - [x] Baconian
 - [x] Caesar
 - [ ] Homophonic Substitution
 - [x] Morse (Could do with more encodings)
 - [ ] Playfair
 - [ ] Polybius Square
 - [x] Rot13
 - [x] Simple Substitution
 - [ ] Straddle Checkerboard

### Polyalphabetic
 - [x] Autokey
 - [x] Beaufort
 - [x] Bellaso
 - [ ] Chaocipher
 - [x] Classic Vigenere
 - [x] Keyed Vigenere
 - [x] Porta

 ### Polygraphic
 - [ ] ADFGVX
 - [ ] ADFGX
 - [ ] Bifid
 - [ ] Four Square
 - [ ] Fractionated Morse
 - [ ] Hill (2x2 and 3x3 matrices)
 - [ ] Trifid
 - [ ] Two Square
 - [ ] VIC

 ### Stream
 - [ ] Solitaire

 ### Transposition
  - [ ] Block Transposition
  - [ ] Column Transposition
  - [ ] Myszkowski Transposition
  - [ ] Railfence
  - [ ] Scytale
