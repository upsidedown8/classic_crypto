# **classic_crypto**
[![Build Status](https://github.com/upsidedown8/classic_crypto/actions/workflows/rust.yml/badge.svg)](https://github.com/upsidedown8/classic_crypto/actions/workflows/rust.yml/)

### A number of classical ciphers implemented in Rust, with the capability to supply a character set for any language. 

## **CLI**

### Capabilities
Perform encrypt, decrypt, analytics and solve operations from the command line.

## **Library**

### Usage

Add this line to your `Cargo.toml`

```toml
classic_crypto = "0.1.0"
```

### Example

```rust
use classic_crypto::{Asymmetric, Caesar, Keyed, Solve, lang::Language};

fn main() {
    let mut language = Language::from_file("examples/data/english.bin").unwrap();
    let mut caesar = Caesar::new(&mut language);

    caesar.randomize(&mut language, &mut rand::thread_rng());

    let plaintext = "Secret message!";
    let ciphertext = caesar.encrypt(&mut language, plaintext);

    caesar.randomize(&mut language, &mut rand::thread_rng());

    caesar.solve(&mut language, &ciphertext);

    let plaintext = caesar.decrypt(&mut language, &ciphertext);
    println!("plaintext {}", plaintext);
}
```

## **Supported Ciphers**

|      Name                         | Encrypt/Decrypt | Solve |
| --------------------------------- | --------------- | ----- |
| ADFGVX                            | ⬜️              | ⬜️   |
| ADFGX                             | ⬜️              | ⬜️   |
| Affine                            | ✅              | ✅   |
| Atbash                            | ✅              | N/A  |
| Autokey                           | ✅              | ✅   |
| Baconian                          | ✅              | N/A  |
| Beaufort                          | ✅              | ✅   |
| Bellaso                           | ✅              | ✅   |
| Bifid                             | ⬜️              | ⬜️   |
| Block Transposition               | ✅              | ✅   |
| Caesar                            | ✅              | ✅   |
| Chaocipher                        | ⬜️              | ⬜️   |
| Classic Vigenère                  | ✅              | ✅   |
| Clock                             | ✅              | ✅   |
| Chase                             | ✅              | ✅   |
| Column Transposition              | ✅              | ✅   |
| Enigma M3/M4                      | ✅              | ⬜️   |
| Fialka                            | ⬜️              | ⬜️   |
| Four Square                       | ⬜️              | ⬜️   |
| Fractionated Morse                | ⬜️              | ⬜️   |
| Hill (2x2 and 3x3 matrices)       | ⬜️              | ⬜️   |
| Homophonic Substitution           | ⬜️              | ⬜️   |
| Keyed Vigenère                    | ✅              | ⬜️   |
| Lorenz                            | ⬜️              | ⬜️   |
| Morse                             | ✅              | N/A  |
| Myszkowski Transposition          | ⬜️              | ⬜️   |
| Playfair                          | ⬜️              | ⬜️   |
| Polybius Square                   | ⬜️              | ⬜️   |
| Porta                             | ✅              | ✅   |
| Purple                            | ⬜️              | ⬜️   |
| Railfence                         | ✅              | ✅   |
| Rot13                             | ✅              | N/A  |
| Scytale                           | ✅              | ✅   |
| Simple Substitution               | ✅              | ✅   |
| Solitaire                         | ⬜️              | ⬜️   |
| Straddle Checkerboard             | ⬜️              | ⬜️   |
| Trifid                            | ⬜️              | ⬜️   |
| Two Square                        | ⬜️              | ⬜️   |
| Typex                             | ⬜️              | ⬜️   |
| VIC                               | ⬜️              | ⬜️   |
