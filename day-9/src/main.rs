use inputs::get_input;

const PREAMBLE_SZ: usize = 25;

fn check_index(index: usize, inputs: &[u64]) -> bool {
    for &num in &inputs[(index - PREAMBLE_SZ)..index] {
        for &num2 in &inputs[(index - PREAMBLE_SZ)..index] {
            if num == num2 {
                continue;
            }

            if num + num2 == inputs[index] {
                return true;
            }
        }
    }

    false
}

fn part1(inputs: &[u64]) -> Option<u64> {
    for index in PREAMBLE_SZ..=inputs.len() - PREAMBLE_SZ {
        if !check_index(index, inputs) {
            return Some(inputs[index]);
        }
    }

    eprintln!("Value not found!");
    None
}

fn part2(inputs: &[u64]) -> Option<u64> {
    for index in PREAMBLE_SZ..=inputs.len() - PREAMBLE_SZ {
        if !check_index(index, inputs) {
            let val = inputs[index];

            for idx in 0..inputs.len() {
                let mut idx2 = idx;
                let mut sum = 0;

                while sum < val {
                    sum += inputs[idx2];
                    idx2 += 1;
                }

                if sum == val {
                    return Some(
                        inputs[idx..idx2].iter().max().unwrap()
                            + inputs[idx..idx2].iter().min().unwrap(),
                    );
                }
            }
        }
    }

    eprintln!("Value not found!");
    None
}

fn main() {
    let inputs = get_input::<u64>("inputs/day-9.txt").expect("Could not parse path!");

    println!("Part 1 solution: {:?}", part1(&inputs));
    println!("Part 2 solution: {:?}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<u64>("../inputs/day-9.txt").expect("Could not parse path!");

    assert_eq!(part1(&inputs), Some(41_682_220));
    assert_eq!(part2(&inputs), Some(5_388_976));
}
