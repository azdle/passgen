extern crate rand;

pub fn generate<I>(charset: I, length: usize) -> String where I: Iterator<Item=char> {
    use rand::{Rng, OsRng};

    let mut r = OsRng::new().expect("failed to get os rng");

    let charset_vec: Vec<char> = charset.collect();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        result.push(*r.choose(&charset_vec).unwrap());
    }

    result
}
