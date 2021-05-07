use classic_crypto::lang::Language;

fn main() {
    // let mut enigma = Enigma::new(&language)

    // let mut cipher = Affine::new(&language);
    // cipher.randomize(&language, &mut rand::thread_rng());

    // let plaintext = "Some plaintext";
    // let ciphertext = cipher.encrypt(&language, plaintext);

    // println!("{}", ciphertext);

    let contents = std::fs::read_to_string("examples/data/english.json")
        .expect("Something went wrong reading the file");
    let lang: Language = Language::from_json(contents).unwrap();

    let json = serde_json::to_string_pretty(&lang).unwrap();

    println!("{}", json);
}
