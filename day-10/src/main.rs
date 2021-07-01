use std::collections::BTreeMap;

use inputs::get_input;

fn part1(inputs: &[i32]) -> i32 {
    let mut vec: Vec<i32> = inputs.to_vec();
    vec.sort_unstable();

    let mut diffs = (0, 0, 1);
    let mut last = 0;

    for num in vec {
        match num - last {
            1 => diffs.0 += 1,
            2 => diffs.1 += 1,
            3 => diffs.2 += 1,
            _ => panic!("Gap exceeded 3 jolts!"),
        }

        last = num;
    }

    diffs.0 * diffs.2
}

fn part2(inputs: &[i32]) -> usize {
    let mut vec = inputs.to_vec();
    let max_val = inputs.iter().max().unwrap() + 3;
    vec.push(max_val);
    vec.sort_unstable();

    let mut map = BTreeMap::new();
    map.insert(0, 1);

    for num in vec {
        map.insert(num, 0);

        if map.contains_key(&(num - 1)) {
            map.insert(num, map[&num] + map[&(num - 1)]);
        }

        if map.contains_key(&(num - 2)) {
            map.insert(num, map[&num] + map[&(num - 2)]);
        }

        if map.contains_key(&(num - 3)) {
            map.insert(num, map[&num] + map[&(num - 3)]);
        }
    }

    map[&max_val]
}

fn main() {
    let inputs = get_input::<i32>("inputs/day-10.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<i32>("../inputs/day-10.txt").expect("Could not parse path!");
    assert_eq!(part1(&inputs), 2_482);
    assert_eq!(part2(&inputs), 96_717_311_574_016);
}
