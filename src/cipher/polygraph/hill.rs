use crate::{
    cipher::{Asymmetric, Keyed},
    key::{IdentityKey, IoKey, Matrix, MatrixDimSize},
    lang::Language,
    util,
};

pub struct Hill {
    pub matrix: Matrix,
}

impl Hill {
    fn run_array(language: &Language, mat: &Matrix, src: &[i16], dest: &mut [i16]) {
        let size = mat.size as usize;

        for i in (0..src.len() - (size - 1)).step_by(size) {
            for j in 0..size {
                dest[i + j] = util::modulo(
                    match mat.size {
                        MatrixDimSize::Two => mat.at(j, 0) * src[i] + mat.at(j, 1) * src[i + 1],
                        MatrixDimSize::Three => {
                            mat.at(j, 0) * src[i]
                                + mat.at(j, 1) * src[i + 1]
                                + mat.at(j, 2) * src[i + 2]
                        }
                    },
                    language.cp_count(),
                );
            }
        }
    }
    fn pad_txt(txt: &mut Vec<i16>, multiple: usize) {
        if !txt.is_empty() {
            let extra_len =
                util::modulo(multiple as i16 - txt.len() as i16, multiple as i16) as usize;
            txt.resize(txt.len() + extra_len, 0);
        }
    }
    fn arr_to_str(language: &Language, msg: &str, dest: &[i16]) -> String {
        let mut i = 0;
        let mut result = msg
            .chars()
            .map(|ch| {
                if language.is_letter(&ch) {
                    let new_letter = language.update_cp(&ch, dest[i]);
                    i += 1;
                    new_letter
                } else {
                    ch
                }
            })
            .collect::<String>();

        while i < dest.len() {
            result.push(language.cp_to_upper(dest[i]));
            i += 1;
        }

        result
    }
}

impl Asymmetric for Hill {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let mut txt = language.string_to_vec(msg);
        let mut dest = vec![0; txt.len()];

        if !txt.is_empty() {
            Hill::pad_txt(&mut txt, self.matrix.size as usize);
            dest.resize(txt.len(), 0);

            Hill::run_array(language, &self.matrix, &txt, &mut dest);
        }

        Hill::arr_to_str(language, msg, &dest)
    }
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        let mut txt = language.string_to_vec(msg);
        let mut dest = vec![0; txt.len()];

        let inv = self.matrix.invert(language);

        if !txt.is_empty() {
            Hill::pad_txt(&mut txt, inv.size as usize);
            dest.resize(txt.len(), 0);

            Hill::run_array(language, &inv, &txt, &mut dest);
        }

        Hill::arr_to_str(language, msg, &dest)
    }
}

impl Keyed for Hill {
    fn new(language: &mut Language) -> Self {
        let mut result = Hill {
            matrix: Matrix::identity(language),
        };

        result
            .matrix
            .key_info_mut()
            .set("Matrix", "<4 or 9 letter string>", "mat");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.matrix]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.matrix]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::Key;

    #[test]
    fn encrypt() {
        let mut language = Language::from_file("examples/data/english.bin").unwrap();
        let mut cipher = Hill::new(&mut language);
        cipher.matrix.set(&mut language, "alphabeta").unwrap();

        let plain = "SECRETMESSAGE";
        let ciphertext = cipher.encrypt(&mut language, plain);

        assert_eq!(&ciphertext, "WYSRIOCYUMCUACQ");
    }

    #[test]
    fn decrypt() {
        let mut language = Language::from_file("examples/data/english.bin").unwrap();
        let mut cipher = Hill::new(&mut language);
        cipher.matrix.set(&mut language, "alphabeta").unwrap();

        let plain = "WYSRIOCYUMCUACQ";
        let ciphertext = cipher.decrypt(&mut language, plain);

        assert_eq!(&ciphertext, "SECRETMESSAGEAA");
    }
}
