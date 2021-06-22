use inputs::get_input;

struct PassWordRule {
    character: char,
    n1: usize,
    n2: usize,
}

impl PassWordRule {
    fn parse(in_str: &str) -> (Self, String) {
        let chunks: Vec<&str> = in_str.split(' ').collect();
        let reps: Vec<&str> = chunks[0].split('-').collect();

        (
            Self {
                character: chunks[1].chars().next().unwrap(),
                n1: String::from(reps[0]).parse::<usize>().unwrap(),
                n2: String::from(reps[1]).parse::<usize>().unwrap(),
            },
            String::from(chunks[2]),
        )
    }
}

fn validate1(rule: &PassWordRule, in_str: &str) -> bool {
    let mut count = 0;

    for c in in_str.chars() {
        if c == rule.character {
            count += 1;
        }
    }

    count <= rule.n2 && count >= rule.n1
}

fn part1(list: &[String]) -> u64 {
    let mut count = 0;

    for line in list {
        let (rule, passwd) = PassWordRule::parse(line);
        if validate1(&rule, &passwd) {
            count += 1;
        }
    }

    count
}

fn validate2(rule: &PassWordRule, in_str: &str) -> bool {
    let chars: Vec<char> = in_str.chars().collect();

    (chars[(rule.n1 - 1)] == rule.character) ^ (chars[(rule.n2 - 1)] == rule.character)
}

fn part2(list: &[String]) -> u64 {
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

#[test]
fn check() {
    let input_lines = get_input::<String>("../inputs/day-2.txt").expect("Could not parse path!");
    
    assert_eq!(part1(&input_lines), 582u64);
    assert_eq!(part2(&input_lines), 729u64);
}
