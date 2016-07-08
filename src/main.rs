extern crate rand;
extern crate clap;

fn main() {
    use clap::{Arg, App};
    use rand::Rng;
    use std::ascii::AsciiExt;

    let matches = App::new("passgen")
                          .version("0.1.0")
                          .author("Patrick Barrett <patrick@mkii.org>")
                          .about("Generates Passwords")
                          .arg(Arg::with_name("LENGTH")
                               .help("Sets the desired password length")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("CHARSET")
                               .help("Sets the special characters to be used")
                               .required(false)
                               .index(2))
                          .get_matches();

    let length_str = matches.value_of("LENGTH").unwrap();
    let length: u16 = length_str.parse().expect("Length must be number");
    let charset = matches.value_of("CHARSET").unwrap_or("");

    let result = rand::thread_rng()
        .gen_iter::<char>()
        .filter(|x| (x.is_ascii() && x.is_alphanumeric()) || (charset.chars().filter(|y| y == x).count() > 0))
        .take(length as usize)
        .collect::<String>();

    println!("{}", result);
}
