use classic_crypto::cli::run;
use classic_crypto::error::Result;

fn main() -> Result<()> {
    run()
}

// use classic_crypto::lang::{LangAlphabet, Language};

// fn main() {
//     let alphabets = vec![
//         LangAlphabet::new(
//             "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
//             "abcdefghijklmnopqrstuvwxyz".to_string(),
//             Vec::new(),
//             Vec::new(),
//             (0..26).collect()
//         ).unwrap(),
//         LangAlphabet::new(
//             "ABCDEFGHIKLMNOPQRSTUVWXYZ".to_string(),
//             "abcdefghiklmnopqrstuvwxyz".to_string(),
//             vec!["JI".to_string()],
//             vec!["ji".to_string()],
//             (0..25).collect()
//         ).unwrap(),
//     ];

//     let language = Language::new(
//         "English".to_string(),
//         26,
//         alphabets,
//         std::fs::read_to_string("examples/data/corpus.txt").unwrap()
//     ).unwrap();

//     std::fs::write("examples/data/english.bin", &bincode::serialize(&language).unwrap()).unwrap();
// }
