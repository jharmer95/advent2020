use inputs::get_input;

fn validate_passport_string1(pass_str: &str) -> bool {
    pass_str.contains("byr:")
        && pass_str.contains("iyr:")
        && pass_str.contains("eyr:")
        && pass_str.contains("hgt:")
        && pass_str.contains("hcl:")
        && pass_str.contains("ecl:")
        && pass_str.contains("pid:")
}

fn validate_passport_string2(pass_str: &str) -> bool {
    let keys: Vec<&str> = pass_str.split(' ').collect();

    match keys.iter().find(|&&s| s.starts_with("byr:")) {
        Some(s) => match s[4..].parse::<i32>() {
            Ok(num) => {
                if !(1920..=2002).contains(&num) {
                    return false;
                }
            }
            _ => {
                return false;
            }
        },
        None => {
            return false;
        }
    }

    match keys.iter().find(|&&s| s.starts_with("iyr:")) {
        Some(s) => match s[4..].parse::<i32>() {
            Ok(num) => {
                if !(2010..=2020).contains(&num) {
                    return false;
                }
            }
            _ => {
                return false;
            }
        },
        None => {
            return false;
        }
    }

    match keys.iter().find(|&&s| s.starts_with("eyr:")) {
        Some(s) => match s[4..].parse::<i32>() {
            Ok(num) => {
                if !(2020..=2030).contains(&num) {
                    return false;
                }
            }
            _ => {
                return false;
            }
        },
        None => {
            return false;
        }
    }

    match keys.iter().find(|&&s| s.starts_with("hgt:")) {
        Some(s) => {
            let hgt_str = &s[4..];

            match hgt_str.find("cm") {
                Some(idx2) => match hgt_str[..idx2].parse::<i32>() {
                    Ok(hgt) => {
                        if !(150..=193).contains(&hgt) {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                },
                None => match hgt_str.find("in") {
                    Some(idx2) => match hgt_str[..idx2].parse::<i32>() {
                        Ok(hgt) => {
                            if !(59..=76).contains(&hgt) {
                                return false;
                            }
                        }
                        _ => {
                            return false;
                        }
                    },
                    None => {
                        return false;
                    }
                },
            }
        }
        None => {
            return false;
        }
    }

    match keys.iter().find(|&&s| s.starts_with("hcl:")) {
        Some(s) => {
            let mut iter = s.chars().skip(4);

            if iter.next().unwrap() != '#' {
                return false;
            }

            for c in iter {
                if !c.is_digit(16) {
                    return false;
                }
            }
        }
        None => {
            return false;
        }
    }

    match keys.iter().find(|&&s| s.starts_with("ecl:")) {
        Some(s) => {
            let ecl_str = &s[4..];

            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl_str) {
                return false;
            }
        }
        None => {
            return false;
        }
    }

    match keys.iter().find(|&&s| s.starts_with("pid:")) {
        Some(s) => {
            if s.len() != (4 + 9) {
                return false;
            }

            matches!(&s[4..].parse::<u64>(), Ok(_))
        }
        None => false,
    }
}

fn part1(inputs: &[String]) -> u64 {
    let mut str_buf = String::new();
    let mut valid_count = 0;

    for line in inputs {
        if line.is_empty() {
            if validate_passport_string1(&str_buf) {
                valid_count += 1;
            }

            str_buf.clear();
            continue;
        }

        str_buf.push_str(line);
        str_buf.push(' ');
    }

    // One more time assuming there is no final empty line
    if validate_passport_string1(&str_buf) {
        valid_count += 1;
    }

    valid_count
}

fn part2(inputs: &[String]) -> u64 {
    let mut str_buf = String::new();
    let mut valid_count = 0;

    for line in inputs {
        if line.is_empty() {
            if validate_passport_string2(&str_buf) {
                valid_count += 1;
            }

            str_buf.clear();
            continue;
        }

        str_buf.push_str(line);
        str_buf.push(' ');
    }

    // One more time assuming there is no final empty line
    if validate_passport_string2(&str_buf) {
        valid_count += 1;
    }

    valid_count
}

fn main() {
    let inputs = get_input::<String>("inputs/day-4.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<String>("../inputs/day-4.txt").expect("Could not parse path!");

    assert_eq!(part1(&inputs), 250u64);
    assert_eq!(part2(&inputs), 158u64);
}
