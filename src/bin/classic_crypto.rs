use classic_crypto::{lang::Language, Bellaso, Keyed, Solve, Symmetric};

fn main() {
    let mut language = Language::from_file("examples/data/english.bin").unwrap();
    let mut cipher = Bellaso::new(&mut language);

    let ciphertext = r#"ojpkiajwibvfhdxoxsfbiblwcxxxvnmqtjgbbxswrvnclzpavjrrzlugwljiubmfgpsthsxwvrbkuivkityljwrfmyfgpxscpskrjcttlagbidjjeanuultpbkljlnpvhilsityqikbhdjjwjlsmzgdhmwsularnwyepnopsfvsxschqpnowxlkjrywlzphecytmlucwgxnbtvrklfpoigbkrpgcyfhdtvqblvhqgitwxscowifnsutzkutuzbsnnakyfjblwxpdhnjrvxxegcfnilrshiljdgalzpgbsbuhakbugiinkitmtfqnnfydsnzjrvuisckudzgcyelnpbsuaiubnuvzttsiknzvhnseztfkisaibnzjejpiinpjxrrshikavcqzsorpalyubotnxsivsisygcqfiysfhkudvzimpwfupgsqjtdxoiigelubbgijavcepwcglwnfgcagwzbtamabykspbsualvingyujtfyzaeesdfjzfvajjrygkjymwxonepjiubgavzpjfblzplcjpfzyplwqmxrbbfzotdpwviousesyxxvnsxbiubrruqyknzzapcfyjuljdgrvbpffmjmwcmnjxjxzzifkvsmfiyudbpvfzvsmrmgoppnacyxxpbsuaqbpkavlfbkkvbtamjytpjbscfbglfmjmljdgugiiektrtqyknjvqdpfqkiajfskuwcgsxcyxafkhvbtbtsyjqypbmfsivsiabqzdnngiyackuyuxpkiochrsxrlmypzygdpetsykzavcrzrzrpagehciiwujhgdjbamyinnzalrlsqigrrpctwhntvhyajrtnfyzbcgazdxvbuviubkiazywdpbqutcfbjefbuobqxcbnscbgiuzmlwyzosyjhuujwxbhozbfhdxoxsfbyaztrzlrbsrzryvhskjjjqonlzyfnjmizartwyxtacjrtpxzkigcgzbrmijtjkkkwngkjykgbeziqbaykidu"#;

    cipher.solve(&mut language, ciphertext);

    println!("{}", cipher.to_string(&mut language));

    let plaintext = cipher.run(&mut language, &ciphertext);
    println!("plaintext {}", plaintext);
}
