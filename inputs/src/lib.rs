use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Opens a file and returns a vector containing the representation of each line of the file
pub fn get_input<T>(path: &str) -> Result<Vec<T>, io::Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(path).unwrap();
    let file_reader = BufReader::new(f);
    let mut vals: Vec<T> = vec![];

    for line in file_reader.lines() {
        vals.push(line.unwrap().trim().parse::<T>().unwrap());
    }

    Ok(vals)
}

/// Opens a file and returns a vector containing the representation of each line of the file,
/// also separating based on a delimiter string
pub fn get_input_delim<T>(path: &str, delim: &str) -> Result<Vec<T>, io::Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let f = File::open(path).unwrap();
    let file_reader = BufReader::new(f);
    let mut vals: Vec<T> = vec![];

    for line in file_reader.lines() {
        for val_str in line.unwrap().split(delim) {
            let val: T = val_str.trim().parse().unwrap();
            vals.push(val);
        }
    }

    Ok(vals)
}
