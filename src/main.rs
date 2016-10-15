extern crate rand;
extern crate clap;

fn main() {
    use clap::{Arg, App};

    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let lowercase = "abcdefghijklmnopqrstuvwxyz".chars();
    let numeric = "1234567890".chars();

    let alpha = uppercase.chain(lowercase);
    let alphanumeric = alpha.chain(numeric);

    let matches = App::new("passgen")
                          .version("0.1.0")
                          .author("Patrick Barrett <patrick@mkii.org>")
                          .about("Generates Passwords")
                          .arg(Arg::with_name("LENGTH")
                               .help("Sets the desired password length")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("SYMBOLS")
                               .help("Sets the special characters to be used")
                               .required(false)
                               .index(2))
                          .get_matches();

    let length_str = matches.value_of("LENGTH").unwrap();
    let length: usize = length_str.parse().expect("Length must be number");
    let symbols = matches.value_of("SYMBOLS").unwrap_or("");

    println!("{}", generate(alphanumeric.chain(symbols.chars()), length));
}


fn generate<I>(charset: I, length: usize) -> String where I: Iterator<Item=char> {
    use rand::{Rng, OsRng};

    let mut r = OsRng::new().expect("failed to get os rng");

    let charset_vec: Vec<char> = charset.collect();
    let mut result = String::with_capacity(length);

    for _ in 0..length {
        result.push(*r.choose(&charset_vec).unwrap());
    }

    result
}