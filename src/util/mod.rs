//!
//! Contains functions that are used across the library
//!

pub mod prime_gen;

/// Returns true if the string contains any repeated characters
///
/// # Arguments
///
/// * string The string to check
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// assert!(!util::is_unique("abca"));
/// assert!(!util::is_unique("AbcA"));
/// assert!(util::is_unique("Abca"));
/// assert!(util::is_unique("abcdefghijklmn"));
/// assert!(util::is_unique(""));
/// ```
///
pub fn is_unique(string: &str) -> bool {
    !string
        .char_indices()
        .any(|(idx, ch)| string.chars().skip(idx + 1).any(|x| x == ch))
}

///
/// Returns a vector of size `alphabet_size` containing all the values in the
/// range `0..alphabet_size`, keyed by the slice `key`.  
///
/// # Detail
/// The alphabet is structured as follows:  
///     `KEY + (KEY.max()..alphabet_size) + (0..KEY.max())`  
/// but repetitions of letters are discarded.
///
/// # Arguments
///
/// * `key` The slice of `code points` with which to populate the alphabet
/// * `alphabet_size` The length of the desired alphabet
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let key = vec![7, 4, 11, 11, 14]; // 'hello' with english Language
/// let alphabet_size = 26;
/// let expected = vec![
///      7,  4, 11, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
///     24, 25,  0,  1,  2,  3,  5,  6,  8,  9, 10, 12, 13,
/// ];
/// assert_eq!(
///     util::fill_alphabet_continue(&key, alphabet_size),
///     expected
/// );
/// ```
///
pub fn fill_alphabet_continue(key: &[i16], alphabet_size: usize) -> Vec<i16> {
    let mut result = vec![0; alphabet_size];
    let mut existing = vec![false; alphabet_size];

    let mut i = 0;
    for j in 0..key.len() {
        if !existing[key[j] as usize] {
            result[i] = key[j];
            i += 1;
        }
        existing[key[j] as usize] = true;
    }

    let val_start = result[std::cmp::max(i - 1, 0)] as usize;
    let mut val = val_start;
    while val < alphabet_size {
        if !existing[val] {
            result[i] = val as i16;
            i += 1;
        }
        val += 1;
    }
    val = 0;
    while val < val_start {
        if !existing[val] {
            result[i] = val as i16;
            i += 1;
        }
        val += 1;
    }

    result
}

///
/// Returns a vector of size `alphabet_size` containing all the values in the
/// range `0..alphabet_size`, keyed by the slice `key`.  
///
/// # Detail
///
/// The alphabet is structured as follows:  
///     `KEY + (0..alphabet_size)`  
/// but repetitions of letters are discarded.
///
/// # Arguments
///
/// * `key` The slice of `code points` with which to populate the alphabet
/// * `alphabet_size` The length of the desired alphabet
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let key = vec![7, 4, 11, 11, 14]; // 'hello' with english Language
/// let alphabet_size = 26;
/// let expected = vec![
///      7,  4, 11, 14,  0,  1,  2,  3,  5,  6,  8,  9, 10,
///     12, 13, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
/// ];
/// assert_eq!(
///     util::fill_alphabet_from_start(&key, alphabet_size),
///     expected
/// );
/// ```
///
pub fn fill_alphabet_from_start(key: &[i16], alphabet_size: usize) -> Vec<i16> {
    let mut result = vec![0; alphabet_size];
    let mut existing = vec![false; alphabet_size];

    let mut i = 0;
    for j in 0..key.len() {
        if !existing[key[j] as usize] {
            result[i] = key[j];
            i += 1;
        }
        existing[key[j] as usize] = true;
    }
    for (val, item) in existing.iter().enumerate().take(alphabet_size) {
        if !(*item) {
            result[i] = val as i16;
            i += 1;
        }
    }

    result
}

///
/// Fills a Vec<i16> with values incremented by 1 each time, starting from
/// `start`. Each value is calculated as `value % modulus`.
///
/// # Arguments
///
/// * `arr` A mutable borrow of the Vec<i16> to fill
/// * `start` The value to start at
/// * `modulus` The constant value to perform `value % modulus` with
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let mut my_vec = vec![0; 10];
/// util::fill_consecutive_vec(&mut my_vec, 0, 5);
/// assert_eq!(my_vec, vec![0,1,2,3,4,0,1,2,3,4]);
/// ```
///
pub fn fill_consecutive_vec(arr: &mut Vec<i16>, start: i16, modulus: i16) {
    let mut i = start;
    for item in arr {
        *item = modulo(i, modulus);
        i += 1;
    }
}

///
/// Fills a Vec<i16> with values incremented by 1 each time, starting from
/// `start`. Each value is calculated as `value % modulus`.
///
/// # Arguments
///
/// * `arr` A mutable borrow of the Vec<i16> to fill
/// * `rng` A rand::Rng implementation to generate random numbers
/// * `max_exclusive` The maximum exclusive value to use in the rng range
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// # use rand::{thread_rng, Rng};
/// let mut my_vec = vec![0; 10];
/// let mut rng = thread_rng();
/// let max_exclusive = 10;
/// util::fill_random_array(&mut my_vec, &mut rng, max_exclusive);
/// my_vec.iter().for_each(|&x| assert!(x < max_exclusive));
/// println!("{:?}", my_vec);
/// ```
///
pub fn fill_random_array(arr: &mut Vec<i16>, rng: &mut impl rand::Rng, max_exclusive: i16) {
    for item in arr {
        *item = rng.gen_range(0..max_exclusive);
    }
}

///
/// Shuffles the elements of a vector
///
/// # Arguments
///
/// * `arr` A mutable borrow of the Vec<i16> to shuffle
/// * `rng` A rand::Rng implementation to generate random numbers
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// # use rand::{thread_rng, Rng};
/// let mut my_vec = vec![0, 1, 2, 3, 4];
/// let mut rng = thread_rng();
/// util::shuffle(&mut my_vec, &mut rng);
/// println!("{:?}", my_vec);
/// ```
///
pub fn shuffle<T>(arr: &mut Vec<T>, rng: &mut impl rand::Rng) {
    for i in 0..arr.len() {
        let pos = rng.gen_range(0..arr.len());
        arr.swap(i, pos);
    }
}

///
/// Inverts a substitution alphabet (a `&[i16]` containing each of the
/// elements `0..arr.len()`), so that each element can be indexed in O(1) time.
/// i.e. transforms `arr[idx] = value` to `arr[value] = idx`.
///
/// # Arguments
///
/// * `arr` The `&[i16]` slice to invert, which **must** contain only the elements `0..arr.len()` in some permutation
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let my_vec = vec![2, 1, 3, 0, 4];
/// let my_inv = util::invert(&my_vec);
/// assert_eq!(my_inv[0] as usize, my_vec.iter().position(|&x| x==0).unwrap());
/// assert_eq!(my_inv[1] as usize, my_vec.iter().position(|&x| x==1).unwrap());
/// assert_eq!(my_inv[2] as usize, my_vec.iter().position(|&x| x==2).unwrap());
/// assert_eq!(my_inv[3] as usize, my_vec.iter().position(|&x| x==3).unwrap());
/// assert_eq!(my_inv[4] as usize, my_vec.iter().position(|&x| x==4).unwrap());
/// ```
///
pub fn invert(arr: &[i16]) -> Vec<i16> {
    let mut dest: Vec<i16> = vec![0; arr.len()];
    for i in 0..arr.len() {
        dest[arr[i] as usize] = i as i16;
    }
    dest
}

///
/// Generates a permutation of the range `0..arr.len()` sorted in the same order
/// as `arr`, representing the current permutation in index form.
///
/// # Detail
///
/// The order array contains a `0` where the lowest element was, and `arr.len()-1`
/// where the maximum element (and furthest right) was. Equal elements in the array
/// are ordered consecutively from left to right.
///
/// # Arguments
///
/// * `arr` The `&[i16]` slice
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let key = vec![7, 4, 11, 11, 14]; // 'hello' with english Language
/// let order = util::find_order(&key);
/// assert_eq!(order, vec![1, 0, 2, 3, 4]);
/// ```
///
pub fn find_order(key: &[i16]) -> Vec<usize> {
    let mut order = vec![0; key.len()];
    let mut pos = 0;
    let mut idx = 0;

    while pos < key.len() {
        for i in 0..key.len() {
            if key[i] == idx as i16 {
                order[i] = pos;
                pos += 1;
            }
        }
        idx += 1;
    }

    order
}

///
/// Calculates the modular multiplicative inverse (`mmi`) of `a` modulo `b`.
/// `mmi` satisfies `(a * mmi) % b == 1`. Returns `None` if there is no solution.
///
/// # Arguments
///
/// * `a` The value to invert
/// * `b` The modulus
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let a = 11;
/// let b = 26;
/// let mmi = util::mmi(a, b).unwrap();
/// assert_eq!((mmi * a) % b, 1);
/// ```
///
pub fn mmi(a: i16, b: i16) -> Option<i16> {
    let (g, x, _) = extended_gcd(a as i32, b as i32);
    match g {
        1 => Some(modulo(x as i16, b)),
        _ => None,
    }
}

///
/// Performs the extended euclidean algorithm on `a` and `b`.
///
/// # Arguments
///
/// * `a` The first value
/// * `b` The second value
///
#[allow(clippy::clippy::many_single_char_names)]
pub fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    match a {
        0 => (b, 0, 1),
        _ => {
            let (g, x, y) = extended_gcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }
}

///
/// Calculates the greatest common divisor (gcd) of `a` and `b`.
///
/// # Arguments
///
/// * `a` The first value
/// * `b` The second value
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let a = 24;
/// let b = 32;
/// assert_eq!(util::gcd(a, b), 8);
/// ```
///
pub fn gcd(a: i16, b: i16) -> i16 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

///
/// Performs the modulo operation, but ensures a positive result,
/// so that it functions for negative numbers by wrapping around
/// the modulus `b`.
///
/// # Arguments
///
/// * `a` The value to perform modulus operation on
/// * `b` The modulus
///
/// # Examples
///
/// ```rust
/// # use classic_crypto::util;
/// let a = -10;
/// let b = 26;
/// assert_eq!(util::modulo(a, b), 16);
/// ```
///
pub fn modulo(a: i16, b: i16) -> i16 {
    (b + (a % b)) % b
}
