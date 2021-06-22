use inputs::get_input;

fn validate_passport_string1(pass_str: &String) -> bool {
    pass_str.contains("byr:")
        && pass_str.contains("iyr:")
        && pass_str.contains("eyr:")
        && pass_str.contains("hgt:")
        && pass_str.contains("hcl:")
        && pass_str.contains("ecl:")
        && pass_str.contains("pid:")
}

fn validate_passport_string2(pass_str: &String) -> bool {
    let keys: Vec<&str> = pass_str.split(" ").collect();

    match keys.iter().find(|&s| (*s).starts_with("byr:")) {
        Some(s) => match s.chars().skip(4).collect::<String>().parse::<i32>() {
            Ok(num) => {
                if num < 1920 || num > 2002 {
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

    match keys.iter().find(|&s| (*s).starts_with("iyr:")) {
        Some(s) => match s.chars().skip(4).collect::<String>().parse::<i32>() {
            Ok(num) => {
                if num < 2010 || num > 2020 {
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

    match keys.iter().find(|&s| (*s).starts_with("eyr:")) {
        Some(s) => match s.chars().skip(4).collect::<String>().parse::<i32>() {
            Ok(num) => {
                if num < 2020 || num > 2030 {
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

    match keys.iter().find(|&s| (*s).starts_with("hgt:")) {
        Some(s) => {
            let hgt_str = s.chars().skip(4).collect::<String>();

            match hgt_str.find("cm") {
                Some(idx2) => match hgt_str
                    .chars()
                    .take(idx2)
                    .collect::<String>()
                    .parse::<i32>()
                {
                    Ok(hgt) => {
                        if hgt < 150 || hgt > 193 {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                },
                None => match hgt_str.find("in") {
                    Some(idx2) => match hgt_str
                        .chars()
                        .take(idx2)
                        .collect::<String>()
                        .parse::<i32>()
                    {
                        Ok(hgt) => {
                            if hgt < 59 || hgt > 76 {
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

    match keys.iter().find(|&s| (*s).starts_with("hcl:")) {
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

    match keys.iter().find(|&s| (*s).starts_with("ecl:")) {
        Some(s) => {
            let ecl_str = s.chars().skip(4).collect::<String>();

            if ecl_str != "amb"
                && ecl_str != "blu"
                && ecl_str != "brn"
                && ecl_str != "gry"
                && ecl_str != "grn"
                && ecl_str != "hzl"
                && ecl_str != "oth"
            {
                return false;
            }
        }
        None => {
            return false;
        }
    }

    match keys.iter().find(|&s| (*s).starts_with("pid:")) {
        Some(s) => {
            if s.len() != (4 + 9) {
                return false;
            }

            match s.chars().skip(4).collect::<String>().parse::<u64>() {
                Ok(_) => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        None => {
            return false;
        }
    }
}

fn part1(inputs: &Vec<String>) -> u64 {
    let mut str_buf: String = String::from("");
    let mut valid_count = 0;

    for line in inputs {
        if line.is_empty() {
            if validate_passport_string1(&str_buf) {
                valid_count += 1;
            }

            str_buf.clear();
            continue;
        }

        str_buf.push_str(line.as_str());
        str_buf.push(' ');
    }

    // One more time assuming there is no final empty line
    if validate_passport_string1(&str_buf) {
        valid_count += 1;
    }

    valid_count
}

fn part2(inputs: &Vec<String>) -> u64 {
    let mut str_buf: String = String::from("");
    let mut valid_count = 0;

    for line in inputs {
        if line.is_empty() {
            if validate_passport_string2(&str_buf) {
                valid_count += 1;
            }

            str_buf.clear();
            continue;
        }

        str_buf.push_str(line.as_str());
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
