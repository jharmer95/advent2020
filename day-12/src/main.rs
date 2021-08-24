use core::mem::swap;
use inputs::get_input;

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&mut self, rotation: Rotation, degrees: i32) {
        let conv_rotation = match rotation {
            Rotation::Left => 4 - ((degrees / 90) % 4),
            Rotation::Right => (degrees / 90) % 4,
        };

        let direction_discriminant = (*self as i32 + conv_rotation) % 4;

        *self = match direction_discriminant {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Impossible number given!"),
        };
    }
}

#[derive(Clone, Copy)]
enum Rotation {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    const fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

struct Ship {
    position: Position,
    direction: Direction,
}

impl Ship {
    const fn new(dir: Direction) -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            direction: dir,
        }
    }

    fn translate(&mut self, dir: Direction, distance: i32) {
        match dir {
            Direction::North => self.position.y += distance,
            Direction::South => self.position.y -= distance,
            Direction::East => self.position.x += distance,
            Direction::West => self.position.x -= distance,
        };
    }

    fn go_forward(&mut self, distance: i32) {
        match self.direction {
            Direction::North => self.position.y += distance,
            Direction::South => self.position.y -= distance,
            Direction::East => self.position.x += distance,
            Direction::West => self.position.x -= distance,
        };
    }

    fn rotate(&mut self, dir: Rotation, degrees: i32) {
        self.direction.rotate(dir, degrees);
    }
}

fn part1(inputs: &[String]) -> i32 {
    let mut ship = Ship::new(Direction::East);

    for input in inputs {
        let amount: i32 = input[1..]
            .parse()
            .expect("Error parsing instruction as i32!");

        match input.chars().next().unwrap() {
            'F' => ship.go_forward(amount),
            'L' => ship.rotate(Rotation::Left, amount),
            'R' => ship.rotate(Rotation::Right, amount),
            'N' => ship.translate(Direction::North, amount),
            'E' => ship.translate(Direction::East, amount),
            'S' => ship.translate(Direction::South, amount),
            'W' => ship.translate(Direction::West, amount),
            _ => panic!("Invalid instruction character!"),
        }
    }

    ship.position.manhattan_distance()
}

fn rotate_waypoint(waypoint: &mut Position, rotation: Rotation, degrees: i32) {
    let conv_rotation = match rotation {
        Rotation::Left => 4 - ((degrees / 90) % 4),
        Rotation::Right => (degrees / 90) % 4,
    };

    match conv_rotation {
        0 => (),
        1 => {
            swap(&mut waypoint.x, &mut waypoint.y);
            waypoint.y *= -1;
        }
        2 => {
            waypoint.x *= -1;
            waypoint.y *= -1;
        }
        3 => {
            swap(&mut waypoint.x, &mut waypoint.y);
            waypoint.x *= -1;
        }
        _ => panic!("Invalid computation of rotation!"),
    }
}

fn part2(inputs: &[String]) -> i32 {
    let mut ship = Ship::new(Direction::East);
    let mut waypoint = Position { x: 10, y: 1 };

    for input in inputs {
        let amount: i32 = input[1..]
            .parse()
            .expect("Error parsing instruction as i32!");

        match input.chars().next().unwrap() {
            'F' => {
                ship.translate(Direction::North, waypoint.y * amount);
                ship.translate(Direction::East, waypoint.x * amount);
            }
            'L' => rotate_waypoint(&mut waypoint, Rotation::Left, amount),
            'R' => rotate_waypoint(&mut waypoint, Rotation::Right, amount),
            'N' => waypoint.y += amount,
            'E' => waypoint.x += amount,
            'S' => waypoint.y -= amount,
            'W' => waypoint.x -= amount,
            _ => panic!("Invalid instruction character!"),
        }
    }

    ship.position.manhattan_distance()
}

fn main() {
    let inputs = get_input::<String>("inputs/day-12.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<String>("../inputs/day-12.txt").expect("Could not parse path!");
    assert_eq!(part1(&inputs), 1_457);
    assert_eq!(part2(&inputs), 106_860);
}
