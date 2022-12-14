use inputs::get_input;

fn calc_seat_id(input: &str) -> i32 {
    struct MinMax {
        min: i32,
        max: i32,
    }

    impl MinMax {
        const fn range(&self) -> i32 {
            self.max - self.min
        }

        fn bisect_low(&mut self) {
            self.max -= (self.range() + 1) / 2;
        }

        fn bisect_high(&mut self) {
            self.min += (self.range() + 1) / 2;
        }
    }

    let mut row = MinMax { min: 0, max: 127 };
    let mut column = MinMax { min: 0, max: 7 };

    for c in input.chars() {
        match c {
            'F' => row.bisect_low(),
            'B' => row.bisect_high(),
            'L' => column.bisect_low(),
            'R' => column.bisect_high(),
            _ => panic!("Invalid character: '{c}' detected!"),
        }
    }

    debug_assert_eq!(row.range(), 0);
    debug_assert_eq!(column.range(), 0);
    row.max * 8 + column.max
}

fn part1(inputs: &[String]) -> i32 {
    let mut max_id = i32::MIN;

    for input in inputs {
        let result = calc_seat_id(input);

        if result > max_id {
            max_id = result;
        }
    }

    max_id
}

fn part2(inputs: &[String]) -> i32 {
    let mut id_list: Vec<i32> = vec![];
    id_list.reserve_exact(inputs.len());

    for input in inputs {
        id_list.push(calc_seat_id(input));
    }

    let min_id = *id_list.iter().min().unwrap();
    let max_id = *id_list.iter().max().unwrap();

    for x in min_id..max_id {
        if !id_list.contains(&x) {
            return x;
        }
    }

    panic!("Could not find a missing seat!");
}

fn main() {
    let inputs = get_input::<String>("inputs/day-5.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<String>("../inputs/day-5.txt").expect("Could not parse path!");

    assert_eq!(part1(&inputs), 871);
    assert_eq!(part2(&inputs), 640);
}
