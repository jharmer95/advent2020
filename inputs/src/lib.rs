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
    let mut nums: Vec<T> = vec![];

    for line in file_reader.lines() {
        nums.push(line.unwrap().trim().parse::<T>().unwrap());
    }

    Ok(nums)
}

/// Opens a file and returns a vector containing the representation of each line of the file,
/// also separating based on a delimiter string
pub fn get_input_delim<T>(path: &str, delim: &str) -> Result<Vec<T>, io::Error>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
    T: std::fmt::Display,
{
    let f = File::open(path).unwrap();
    let file_reader = BufReader::new(f);
    let mut nums: Vec<T> = vec![];

    for line in file_reader.lines() {
        for num in line.unwrap().split(delim) {
            let val: T = num.parse().unwrap();
            nums.push(val);
        }
    }

    Ok(nums)
}
