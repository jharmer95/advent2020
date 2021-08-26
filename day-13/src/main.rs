use std::collections::BTreeMap;

use inputs::get_input_delim;

fn part1(inputs: &[String]) -> i32 {
    let timestamp: i32 = inputs[0].parse().expect("Error parsing timestamp!");
    let mut min_diff: i32 = std::i32::MAX;
    let mut next_bus = 0;

    for input in &inputs[1..] {
        if input == "x" {
            continue;
        }

        let bus_num: i32 = input.parse().expect("Error parsing bus number!");

        let diff = if (timestamp % bus_num) == 0 {
            0
        } else {
            bus_num - (timestamp % bus_num)
        };

        if diff < min_diff {
            min_diff = diff;
            next_bus = bus_num;
        }
    }

    min_diff * next_bus
}

struct SieveInput {
    modulo: i64,
    offset: i64,
}

fn part2_naive(inputs: &[String]) -> i64 {
    let mut offset: i64 = 0;
    let mut vec = Vec::new();

    for input in &inputs[1..] {
        if input == "x" {
            offset += 1;
            continue;
        }

        let bus_num: i64 = input.parse().expect("Error parsing bus number!");
        vec.push(SieveInput {
            modulo: bus_num,
            offset,
        });

        offset += 1;
    }

    // TODO: Try using a bezout identity to calculate (https://www.wikiwand.com/en/Chinese_remainder_theorem#/Using_the_existence_construction)

    0
}

fn part2(inputs: &[String]) -> i64 {
    // TODO: Find a more algorithmic way to solve this
    part2_naive(inputs)
}

fn main() {
    let inputs =
        get_input_delim::<String>("inputs/day-13.txt", ",").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs =
        get_input_delim::<String>("../inputs/day-13.txt", ",").expect("Could not parse path!");

    // TODO: Re-enable tests once complete!
    //assert_eq!(part1(&inputs), 2_045);
    //assert_eq!(part2(&inputs), 1_068_788);
}
