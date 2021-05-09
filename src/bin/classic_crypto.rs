use classic_crypto::{Asymmetric, BlockTransposition, Keyed, Solve, key::SetKey, lang::Language};

fn main() {
    let mut language = Language::from_file("examples/data/english.bin").unwrap();
    let mut cipher = BlockTransposition::new(&mut language);
    
    let key = language.string_to_vec("badc");
    cipher.keyword.set_key(&mut language, &key);
    
    let plaintext = "Abc d a BC d abcd abcd";

    let ciphertext = cipher.encrypt(&mut language, plaintext);
    let plaintext = cipher.decrypt(&mut language, &ciphertext);

    println!("ciphertext: {}", ciphertext);
    println!("plaintext: {}", plaintext);
    println!("cipher: {}", cipher.to_string(&mut language));
}
