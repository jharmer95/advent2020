use inputs::get_input;

struct PassWordRule {
    character: char,
    n1: usize,
    n2: usize,
}

impl PassWordRule {
    fn parse(in_str: &str) -> (Self, &str) {
        let chunks: Vec<&str> = in_str.split(' ').collect();
        let reps: Vec<&str> = chunks[0].split('-').collect();

        (
            Self {
                character: chunks[1].chars().next().unwrap(),
                n1: reps[0].parse::<usize>().unwrap(),
                n2: reps[1].parse::<usize>().unwrap(),
            },
            chunks[2],
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
        if validate1(&rule, passwd) {
            count += 1;
        }
    }

    count
}

fn validate2(rule: &PassWordRule, in_str: &str) -> bool {
    (in_str.chars().nth(rule.n1 - 1).unwrap() == rule.character)
        ^ (in_str.chars().nth(rule.n2 - 1).unwrap() == rule.character)
}

fn part2(list: &[String]) -> u64 {
    let mut count = 0;

    for line in list {
        let (rule, passwd) = PassWordRule::parse(line);
        if validate2(&rule, passwd) {
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

    assert_eq!(part1(&input_lines), 582);
    assert_eq!(part2(&input_lines), 729);
}
