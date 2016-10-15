extern crate rand;
use std::io::{Error, ErrorKind};

pub fn generate<I>(charset: I, length: usize) -> Result<String, Error>
        where I: Iterator<Item=char> {
    use rand::{Rng, OsRng};

    let mut r = try!(OsRng::new());

    let charset_vec: Vec<char> = charset.collect();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        result.push(*try!(
            r.choose(&charset_vec)
                .ok_or(Error::new(ErrorKind::InvalidInput,
                                  "no charset specified"))
        ));
    }

    Ok(result)
}
