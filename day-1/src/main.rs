use inputs::get_input;

fn part1(inputs: &Vec<i32>) -> Result<i32, String> {
    let calculated: Vec<i32> = inputs.into_iter().map(|i| 2020 - i).collect();

    for num in inputs {
        if calculated.iter().any(|i| i == num) {
            return Ok(num * (2020 - num));
        }
    }

    Err(String::from("Could not find match in inputs!"))
}

fn part2(inputs: &Vec<i32>) -> Result<i32, String> {
    let calculated: Vec<i32> = inputs.into_iter().map(|i| 2020 - i).collect();

    for num in inputs {
        for num2 in inputs {
            if num == num2 {
                continue;
            } else if calculated.iter().any(|i| *i == num + num2) {
                return Ok(num * num2 * (2020 - num - num2));
            }
        }
    }

    Err(String::from("Could not find match in inputs!"))
}

fn main() {
    let numbers = get_input::<i32>("inputs/day-1.txt").expect("Could not parse path!");

    println!("Part 1 solution: {:?}", part1(&numbers));
    println!("Part 1 solution: {:?}", part2(&numbers));
}
