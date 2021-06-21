use inputs::get_input;

struct PassWordRule {
    character: char,
    n1: i32,
    n2: i32,
}

impl PassWordRule {
    fn parse(in_str: &String) -> (PassWordRule, String) {
        let chunks: Vec<&str> = in_str.split(" ").collect();
        let reps: Vec<&str> = chunks[0].split("-").collect();

        (
            PassWordRule {
                character: chunks[1].chars().nth(0).unwrap(),
                n1: String::from(reps[0]).parse::<i32>().unwrap(),
                n2: String::from(reps[1]).parse::<i32>().unwrap(),
            },
            String::from(chunks[2]),
        )
    }
}

fn validate1(rule: &PassWordRule, in_str: &String) -> bool {
    let mut count = 0;

    for c in in_str.chars() {
        if c == rule.character {
            count += 1;
        }
    }

    count <= rule.n2 && count >= rule.n1
}

fn part1(list: &Vec<String>) -> i32 {
    let mut count = 0;

    for line in list {
        let (rule, passwd) = PassWordRule::parse(line);
        if validate1(&rule, &passwd) {
            count += 1;
        }
    }

    count
}

fn validate2(rule: &PassWordRule, in_str: &String) -> bool {
    let chars: Vec<char> = in_str.chars().collect();

    (chars[(rule.n1 - 1) as usize] == rule.character)
        ^ (chars[(rule.n2 - 1) as usize] == rule.character)
}

fn part2(list: &Vec<String>) -> i32 {
    let mut count = 0;

    for line in list {
        let (rule, passwd) = PassWordRule::parse(line);
        if validate2(&rule, &passwd) {
            count += 1;
        }
    }

    count
}

fn main() {
    let input_lines = get_input::<String>("inputs/day-2.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&input_lines));
    println!("Part 2 solution: {}", part2(&input_lines));
}
