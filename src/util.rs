pub fn fill_alphabet_continue(key: &Vec<i16>, alphabet_size: usize) -> Vec<i16> {
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
pub fn fill_alphabet_from_start(key: &Vec<i16>, alphabet_size: usize) -> Vec<i16> {
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
    for val in 0..alphabet_size {
        if !existing[val] {
            result[i] = val as i16;
            i += 1;
        }
    }

    result
}

pub fn fill_consecutive_vec(arr: &mut Vec<i16>, start: i16, modulus: i16) {
    for i in 0..arr.len() {
        arr[i] = modulo(start + (i as i16), modulus);
    }
}
pub fn fill_random_array(arr: &mut Vec<i16>, rng: &mut impl rand::Rng, max_exclusive: i16) {
    for i in 0..arr.len() {
        arr[i] = rng.gen_range(0..max_exclusive) as i16;
    }
}

pub fn swap(arr: &mut Vec<i16>, a: usize, b: usize) {
    let temp: i16 = arr[a];
    arr[a] = arr[b];
    arr[b] = temp;
}
pub fn shuffle(arr: &mut Vec<i16>, rng: &mut impl rand::Rng) {
    for i in 0..arr.len() {
        swap(arr, i, rng.gen_range(0..arr.len()));
    }
}

pub fn invert(arr: &Vec<i16>) -> Vec<i16> {
    let mut dest: Vec<i16> = vec![0; arr.len()];
    for i in 0..arr.len() {
        dest[arr[i] as usize] = i as i16;
    }
    dest
}
pub fn find_order(key: &Vec<i16>) -> Vec<i16> {
    let mut order: Vec<i16> = vec![0; key.len()];
    let mut pos: usize = 0;
    let mut idx: usize = 0;

    while pos < key.len() {
        for i in 0..key.len() {
            if key[i] == idx as i16 {
                order[i] = pos as i16;
                pos += 1;
            }
        }
        idx += 1;
    }

    order
}

pub fn mmi(a: i16, b: i16) -> Option<i16> {
    let (g, x, _) = extended_gcd(a as i32, b as i32);
    match g {
        1 => Some(modulo(x as i16, b)),
        _ => None,
    }
}
pub fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    match a {
        0 => (b, 0, 1),
        _ => {
            let (g, x, y) = extended_gcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }
}
pub fn gcd(a: i16, b: i16) -> i16 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn modulo(a: i16, b: i16) -> i16 {
    (b + (a % b)) % b
}
