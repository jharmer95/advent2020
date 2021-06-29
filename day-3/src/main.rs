use inputs::get_input;

fn check_slope(inputs: &[String], right: usize, down: usize) -> u64 {
    let num_col = inputs[0].len();
    let mut cur_col = 0;
    let mut cur_row = 0;
    let mut tree_count = 0;

    for row in inputs {
        if cur_row % down != 0 {
            cur_row += 1;
            continue;
        }

        if row.chars().nth(cur_col % num_col).unwrap() == '#' {
            tree_count += 1;
        }

        cur_row += 1;
        cur_col += right;
    }

    tree_count
}

fn part1(inputs: &[String]) -> u64 {
    check_slope(inputs, 3, 1)
}

fn part2(inputs: &[String]) -> u64 {
    check_slope(inputs, 1, 1)
        * check_slope(inputs, 3, 1)
        * check_slope(inputs, 5, 1)
        * check_slope(inputs, 7, 1)
        * check_slope(inputs, 1, 2)
}

fn main() {
    let inputs = get_input::<String>("inputs/day-3.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<String>("../inputs/day-3.txt").expect("Could not parse path!");

    assert_eq!(part1(&inputs), 289);
    assert_eq!(part2(&inputs), 5_522_401_584);
}
