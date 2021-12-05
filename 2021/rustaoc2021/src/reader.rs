use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Lines};
use std::str::FromStr;

pub fn parse_lines_to_vec<T>(filename: &str) -> Result<Vec<T>, Error>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut vec: Vec<T> = Vec::new();
    for line in get_lines(filename) {
        if let Ok(text) = line {
            vec.push(text.trim().parse().expect("parse error"));
        } else {
            return Err(Error::new(ErrorKind::Other, "failed to load all lines"));
        }
    }
    Ok(vec)
}

pub fn get_lines(filename: &str) -> Lines<BufReader<File>> {
    let f = File::open(filename).expect("Something went wrong loading file");
    BufReader::new(f).lines()
}
