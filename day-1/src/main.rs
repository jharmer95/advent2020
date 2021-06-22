use inputs::get_input;

fn part1(inputs: &[u64]) -> Result<u64, String> {
    let calculated: Vec<u64> = inputs.iter().map(|i| 2020 - i).collect();

    for num in inputs {
        if calculated.iter().any(|i| i == num) {
            return Ok(num * (2020 - num));
        }
    }

    Err(String::from("Could not find match in inputs!"))
}

fn part2(inputs: &[u64]) -> Result<u64, String> {
    let calculated: Vec<u64> = inputs.iter().map(|i| 2020 - i).collect();

    for num in inputs {
        for num2 in inputs {
            if num == num2 {
                continue;
            }

            if calculated.iter().any(|i| *i == num + num2) {
                return Ok(num * num2 * (2020 - num - num2));
            }
        }
    }

    Err(String::from("Could not find match in inputs!"))
}

fn main() {
    let numbers = get_input::<u64>("inputs/day-1.txt").expect("Could not parse path!");

    println!("Part 1 solution: {:?}", part1(&numbers));
    println!("Part 1 solution: {:?}", part2(&numbers));
}

#[test]
fn check() {
    let numbers = get_input::<u64>("../inputs/day-1.txt").expect("Could not parse path!");
    
    assert_eq!(part1(&numbers).unwrap(), 290784u64);
    assert_eq!(part2(&numbers).unwrap(), 177337980u64);
}
