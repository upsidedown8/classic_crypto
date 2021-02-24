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
