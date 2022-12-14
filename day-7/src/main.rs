use inputs::get_input;
use std::collections::HashMap;

fn tokenize(inputs: &[String]) -> HashMap<&str, Vec<(String, usize)>> {
    let mut ret = HashMap::new();

    for line in inputs {
        let split: Vec<&str> = line.split(" bags contain ").collect();
        let container = split[0];
        let contents = split[1];

        let mut str1 = contents.replace(" bags", "");
        str1 = str1.replace(" bag", "");
        str1 = str1.replace('.', "");
        let mut split2: Vec<&str> = str1.split(", ").collect();
        split2.retain(|&x| x != "no other");

        let mut vec = Vec::new();

        for part in split2 {
            let (num, color) = part.split_at(1);
            let num = num.parse::<usize>().unwrap();
            vec.push((color.trim().to_string(), num));
        }

        ret.insert(container, vec);
    }

    ret
}

fn count_bags1(map: &HashMap<&str, Vec<(String, usize)>>, color: &str, bag_list: &mut Vec<String>) {
    for (&k, v) in map {
        for entry in v {
            if entry.0 == color {
                bag_list.push(k.to_string());
                count_bags1(map, k, bag_list);
            }
        }
    }
}

fn count_bags2(map: &HashMap<&str, Vec<(String, usize)>>, color: &str) -> usize {
    let mut count = 0;

    for (&k, v) in map {
        if k == color {
            for entry in v {
                count += entry.1 + entry.1 * count_bags2(map, entry.0.as_str());
            }
        }
    }

    count
}

fn part1(inputs: &[String], bag_name: &str) -> usize {
    let map = tokenize(inputs);

    let mut bag_list = vec![];
    count_bags1(&map, bag_name, &mut bag_list);

    bag_list.sort_unstable();
    bag_list.dedup();
    bag_list.len()
}

fn part2(inputs: &[String], bag_name: &str) -> usize {
    let map = tokenize(inputs);

    count_bags2(&map, bag_name)
}

fn main() {
    let inputs = get_input::<String>("inputs/day-7.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs, "shiny gold"));
    println!("Part 2 solution: {}", part2(&inputs, "shiny gold"));
}

#[test]
fn check() {
    let inputs = get_input::<String>("../inputs/day-7.txt").expect("Could not parse path!");
    assert_eq!(part1(&inputs, "shiny gold"), 300);
    assert_eq!(part2(&inputs, "shiny gold"), 8030);
}
