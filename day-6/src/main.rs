use inputs::get_input;

fn part1(inputs: &[String]) -> usize {
    let mut str_buf = String::new();
    let mut count = 0;
    let mut char_vec: Vec<char> = vec![];
    char_vec.reserve(26);

    for line in inputs {
        if line.is_empty() {
            char_vec = str_buf.chars().collect();
            char_vec.sort_unstable();
            char_vec.dedup();

            count += char_vec.len();
            str_buf.clear();
            continue;
        }

        str_buf.push_str(line);
    }

    char_vec = str_buf.chars().collect();
    char_vec.sort_unstable();
    char_vec.dedup();

    count + char_vec.len()
}

fn part2(inputs: &[String]) -> usize {
    let mut str_buf = String::new();
    let mut count = 0;
    let mut group_sz = 0;

    for line in inputs {
        if line.is_empty() {
            for c in 'a'..='z' {
                if str_buf.matches(c).count() == group_sz {
                    count += 1;
                }
            }

            str_buf.clear();
            group_sz = 0;
            continue;
        }

        str_buf.push_str(line);
        group_sz += 1;
    }

    for c in 'a'..='z' {
        if str_buf.matches(c).count() == group_sz {
            count += 1;
        }
    }

    count
}

fn main() {
    let inputs = get_input::<String>("inputs/day-6.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<String>("../inputs/day-6.txt").expect("Could not parse path!");

    assert_eq!(part1(&inputs), 6947);
    assert_eq!(part2(&inputs), 3398);
}
