pub fn to_string(arr: &Vec<u16>) -> String {
    arr
        .into_iter()
        .map(|i| {
            ((*i as u8) + 65) as char
        })
        .collect()
}
pub fn from_str(string: &str) -> Vec<u16> {
    string
        .chars()
        .map(|i| {
            let upper = i.to_uppercase().next().unwrap();
            (upper as u16) - 65
        })
        .collect()
}
pub fn from_string(string: &String) -> Vec<u16> {
    string
        .chars()
        .map(|i| {
            let upper = i.to_uppercase().next().unwrap();
            (upper as u16) - 65
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_to_string() {
        let msg = String::from("");
        let vector = from_string(&msg);
        assert_eq!(vector, vec![0; 0]);
    }

    #[test]
    fn empty_from_string() {
        let vector = vec![0; 0];
        let msg = to_string(&vector);
        assert_eq!(msg, String::from(""));
    }

    #[test]
    fn lower_msg_to_string() {
        let msg = String::from("convert");
        let vector = from_string(&msg);
        assert_eq!(vector, vec![2, 14, 13, 21, 4, 17, 19]);
    }

    #[test]
    fn upper_msg_to_string() {
        let msg = String::from("CONVERT");
        let vector = from_string(&msg);
        assert_eq!(vector, vec![2, 14, 13, 21, 4, 17, 19]);
    }

    #[test]
    fn msg_from_string() {
        let vector = vec![2, 14, 13, 21, 4, 17, 19];
        let msg = to_string(&vector);
        assert_eq!(msg, String::from("CONVERT"));
    }
}