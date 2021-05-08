use classic_crypto::{Asymmetric, Bellaso, Keyed, Solve, Symmetric, key::SetKey, lang::Language};

fn main() {
    let mut language = Language::from_file("examples/data/english.bin").unwrap();
    let mut cipher = Bellaso::new(&mut language);

    let ciphertext = r#"ojpkiajwibvfhdxoxsfbiblwcxxxvnmqtjgbbxswrvnclzpavjrrzlugwljiubmfgpsthsxwvrbkuivkityljwrfmyfgpxscpskrjcttlagbidjjeanuultpbkljlnpvhilsityqikbhdjjwjlsmzgdhmwsularnwyepnopsfvsxschqpnowxlkjrywlzphecytmlucwgxnbtvrklfpoigbkrpgcyfhdtvqblvhqgitwxscowifnsutzkutuzbsnnakyfjblwxpdhnjrvxxegcfnilrshiljdgalzpgbsbuhakbugiinkitmtfqnnfydsnzjrvuisckudzgcyelnpbsuaiubnuvzttsiknzvhnseztfkisaibnzjejpiinpjxrrshikavcqzsorpalyubotnxsivsisygcqfiysfhkudvzimpwfupgsqjtdxoiigelubbgijavcepwcglwnfgcagwzbtamabykspbsualvingyujtfyzaeesdfjzfvajjrygkjymwxonepjiubgavzpjfblzplcjpfzyplwqmxrbbfzotdpwviousesyxxvnsxbiubrruqyknzzapcfyjuljdgrvbpffmjmwcmnjxjxzzifkvsmfiyudbpvfzvsmrmgoppnacyxxpbsuaqbpkavlfbkkvbtamjytpjbscfbglfmjmljdgugiiektrtqyknjvqdpfqkiajfskuwcgsxcyxafkhvbtbtsyjqypbmfsivsiabqzdnngiyackuyuxpkiochrsxrlmypzygdpetsykzavcrzrzrpagehciiwujhgdjbamyinnzalrlsqigrrpctwhntvhyajrtnfyzbcgazdxvbuviubkiazywdpbqutcfbjefbuobqxcbnscbgiuzmlwyzosyjhuujwxbhozbfhdxoxsfbyaztrzlrbsrzryvhskjjjqonlzyfnjmizartwyxtacjrtpxzkigcgzbrmijtjkkkwngkjykgbeziqbaykidu"#;

    let key = language.string_to_vec("fortification");
    cipher.keyword.set_key(&mut language, &key);

    println!("{}", cipher.to_string(&mut language));

    let plaintext = cipher.run(&mut language, &ciphertext);
    println!("plaintext {}", plaintext);
}
