use std::fs::{self, read_to_string, File};
use std::io::{self, Read};

macro_rules! try_ {
    ($expr:expr) => {
        match $expr {
            Ok(result) => result,
            Err(e) => return Err(e),
        }
    };
}

fn main() -> Result<(), io::Error> {
    try_!(all_your_quotes_are_belong_to_us());
    Err(io::Error::new(io::ErrorKind::Other, "oh no!"))
}

fn all_your_quotes_are_belong_to_us() -> Result<String, io::Error> {
    let mut deep = try_!(File::open("deep_quotes.txt"));
    let mut wide = try_!(File::open("wide_quotes.txt"));



    let mut contents = String::new();
    try_!(deep.read_to_string(&mut contents));
    try_!(wide.read_to_string(&mut contents));
    Ok(contents)
}

fn all_your_quotes_are_belong_to_us_without_try_macro() -> Result<String, io::Error> {
    let mut quotes = String::new();

    quotes.push_str(&fs::read_to_string("deep_quotes.txt")?);
    quotes.push_str(&fs::read_to_string("wide_quotes.txt")?);


    
    Ok(quotes)
}