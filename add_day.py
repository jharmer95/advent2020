import subprocess
import sys
import webbrowser

num: int = sys.argv[1]

with open("Cargo.toml", "r+") as f:
    lines = f.readlines()
    lines.insert(-1, f'    "day-{num}",\n')
    f.seek(0)
    f.truncate(0)
    f.writelines(lines)

subprocess.run(["cargo", "new", f"day-{num}"])
open(f"inputs/day-{num}.txt", "w").close()

with open(f"day-{num}/Cargo.toml", "a") as f:
    f.write('inputs = { path = "../inputs" }')

with open(f"day-{num}/src/main.rs", "w") as f:
    f.write(
f'''use inputs::get_input;

fn part1(inputs: &[String]) -> i32 {{
    todo!("Implement Part 1!");
    0
}}

fn part2(inputs: &[String]) -> i32 {{
    todo!("Implement Part 2!");
    0
}}

fn main() {{
    let inputs = get_input::<String>("inputs/day-{num}.txt").expect("Could not parse path!");

    println!("Part 1 solution: {{}}", part1(&inputs));
    println!("Part 2 solution: {{}}", part2(&inputs));
}}

#[test]
fn check() {{
    let inputs = get_input::<String>("../inputs/day-{num}.txt").expect("Could not parse path!");
    assert_eq!(part1(&inputs), 0);
    assert_eq!(part2(&inputs), 0);
}}
'''
)

webbrowser.open(f"https://adventofcode.com/2020/day/{num}")
