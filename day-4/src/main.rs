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

fn validate_key(list: &[&str], key: &str, f: &dyn Fn(&&str) -> bool) -> bool {
    list.iter()
        .find(|&&s| s.starts_with(key))
        .map_or(false, |s| f(s))
}

fn validate_passport_string2(pass_str: &str) -> bool {
    let keys: Vec<&str> = pass_str.split(' ').collect();

    validate_key(&keys, "byr:", &|s| {
        s[4..]
            .parse::<i32>()
            .map_or(false, |num| (1920..=2002).contains(&num))
    }) && validate_key(&keys, "iyr:", &|s| {
        s[4..]
            .parse::<i32>()
            .map_or(false, |num| (2010..=2020).contains(&num))
    }) && validate_key(&keys, "eyr:", &|s| {
        s[4..]
            .parse::<i32>()
            .map_or(false, |num| (2020..=2030).contains(&num))
    }) && validate_key(&keys, "hgt:", &|s| {
        let hgt_str = &s[4..];

        hgt_str.find("cm").map_or_else(
            || {
                hgt_str.find("in").map_or(false, |idx2| {
                    hgt_str[..idx2]
                        .parse::<i32>()
                        .map_or(false, |hgt| (59..=76).contains(&hgt))
                })
            },
            |idx2| {
                hgt_str[..idx2]
                    .parse::<i32>()
                    .map_or(false, |hgt| (150..=193).contains(&hgt))
            },
        )
    }) && validate_key(&keys, "hcl:", &|s| {
        let mut iter = s.chars().skip(4);

        iter.next().unwrap() == '#' && iter.all(|c| c.is_ascii_hexdigit())
    }) && validate_key(&keys, "ecl:", &|s| {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&s[4..])
    }) && validate_key(&keys, "pid:", &|s| {
        s.len() == 13 && matches!(&s[4..].parse::<u64>(), Ok(_))
    })
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

    assert_eq!(part1(&inputs), 250);
    assert_eq!(part2(&inputs), 158);
}
