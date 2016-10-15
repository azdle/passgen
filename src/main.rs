extern crate clap;
extern crate passgen;

fn main() {
    use clap::{Arg, App};

    let uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let lowercase = "abcdefghijklmnopqrstuvwxyz".chars();
    let numeric = "1234567890".chars();

    let alpha = uppercase.chain(lowercase);
    let alphanumeric = alpha.chain(numeric);

    let matches = App::new("passgen")
                          .version("0.1.0")
                          .author("Patrick Barrett <patrick@psbarrett.com>")
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

    println!("{}", passgen::generate(
      alphanumeric.chain(symbols.chars()),
      length
    ).expect("failed to generate password"));
}
