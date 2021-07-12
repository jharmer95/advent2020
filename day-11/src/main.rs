use inputs::get_input;

enum Seat {
    None,
    Empty,
    Occupied,
}

impl Clone for Seat {
    fn clone(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Empty => Self::Empty,
            Self::Occupied => Self::Occupied,
        }
    }
}

impl Seat {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::None,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!("Invalid character parsed!"),
        }
    }

    fn parse_row(s: &str) -> Vec<Self> {
        let mut vec = vec![];
        vec.reserve(s.len());

        for c in s.chars() {
            vec.push(Self::parse(c));
        }

        vec
    }

    fn parse_grid(grid: &[String]) -> Vec<Vec<Self>> {
        let mut vec = vec![];
        vec.reserve(grid.len());

        for s in grid {
            vec.push(Self::parse_row(s));
        }

        vec
    }
}

fn check_neighbors(current: (usize, usize), seats: &[Vec<Seat>]) -> i32 {
    let mut count = 0;

    if current.0 != 0 && matches!(seats[current.0 - 1][current.1], Seat::Occupied) {
        count += 1;
    }

    if current.1 != 0 && matches!(seats[current.0][current.1 - 1], Seat::Occupied) {
        count += 1;
    }

    if current.0 != seats.len() - 1 && matches!(seats[current.0 + 1][current.1], Seat::Occupied) {
        count += 1;
    }

    if current.1 != seats[current.0].len() - 1
        && matches!(seats[current.0][current.1 + 1], Seat::Occupied)
    {
        count += 1;
    }

    if current.0 != 0
        && current.1 != 0
        && matches!(seats[current.0 - 1][current.1 - 1], Seat::Occupied)
    {
        count += 1;
    }

    if current.0 != 0
        && current.1 != seats[current.0].len() - 1
        && matches!(seats[current.0 - 1][current.1 + 1], Seat::Occupied)
    {
        count += 1;
    }

    if current.0 != seats.len() - 1
        && current.1 != 0
        && matches!(seats[current.0 + 1][current.1 - 1], Seat::Occupied)
    {
        count += 1;
    }

    if current.0 != seats.len() - 1
        && current.1 != seats[current.0].len() - 1
        && matches!(seats[current.0 + 1][current.1 + 1], Seat::Occupied)
    {
        count += 1;
    }

    count
}

fn part1(inputs: &[String]) -> i32 {
    let mut seats = Seat::parse_grid(inputs);
    let mut changes: Vec<(usize, usize)> = vec![];

    loop {
        changes.clear();

        for i in 0..seats.len() {
            for j in 0..seats[i].len() {
                match seats[i][j] {
                    Seat::None => continue,
                    Seat::Empty => {
                        if check_neighbors((i, j), &seats) == 0 {
                            changes.push((i, j));
                        }
                    }

                    Seat::Occupied => {
                        if check_neighbors((i, j), &seats) >= 4 {
                            changes.push((i, j));
                        }
                    }
                }
            }
        }

        if changes.is_empty() {
            break;
        }

        for change in &changes {
            match seats[change.0][change.1] {
                Seat::None => continue,
                Seat::Occupied => {
                    seats[change.0][change.1] = Seat::Empty;
                }
                Seat::Empty => {
                    seats[change.0][change.1] = Seat::Occupied;
                }
            }
        }
    }

    let mut occ_count = 0;

    for row in &seats {
        for seat in row {
            if matches!(seat, Seat::Occupied) {
                occ_count += 1;
            }
        }
    }

    occ_count
}

fn check_los((y, x): (usize, usize), seats: &[Vec<Seat>]) -> i32 {
    let mut count = 0;

    let mut current_y = y;

    // Look North
    loop {
        if current_y == 0 {
            break;
        }

        current_y -= 1;

        match seats[current_y][x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    current_y = y;
    let mut current_x = x;

    // Look NE
    loop {
        if current_y == 0 || current_x == seats[y].len() - 1 {
            break;
        }

        current_y -= 1;
        current_x += 1;

        match seats[current_y][current_x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    current_x = x;

    // Look E
    loop {
        if current_x == seats[y].len() - 1 {
            break;
        }

        current_x += 1;

        match seats[y][current_x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    current_y = y;
    current_x = x;

    // Look SE
    loop {
        if current_y == seats.len() - 1 || current_x == seats[y].len() - 1 {
            break;
        }

        current_y += 1;
        current_x += 1;

        match seats[current_y][current_x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    current_y = y;

    // Look S
    loop {
        if current_y == seats.len() - 1 {
            break;
        }

        current_y += 1;

        match seats[current_y][x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    current_y = y;
    current_x = x;

    // Look SW
    loop {
        if current_y == seats.len() - 1 || current_x == 0 {
            break;
        }

        current_y += 1;
        current_x -= 1;

        match seats[current_y][current_x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    current_x = x;

    // Look W
    loop {
        if current_x == 0 {
            break;
        }

        current_x -= 1;

        match seats[y][current_x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    current_y = y;
    current_x = x;

    // Look NW
    loop {
        if current_y == 0 || current_x == 0 {
            break;
        }

        current_y -= 1;
        current_x -= 1;

        match seats[current_y][current_x] {
            Seat::Occupied => {
                count += 1;
                break;
            }
            Seat::Empty => {
                break;
            }
            Seat::None => continue,
        }
    }

    count
}

fn part2(inputs: &[String]) -> i32 {
    let mut seats = Seat::parse_grid(inputs);
    let mut changes: Vec<(usize, usize)> = vec![];

    loop {
        changes.clear();

        for i in 0..seats.len() {
            for j in 0..seats[i].len() {
                match seats[i][j] {
                    Seat::None => continue,
                    Seat::Empty => {
                        if check_los((i, j), &seats) == 0 {
                            changes.push((i, j));
                        }
                    }

                    Seat::Occupied => {
                        if check_los((i, j), &seats) >= 5 {
                            changes.push((i, j));
                        }
                    }
                }
            }
        }

        if changes.is_empty() {
            break;
        }

        for change in &changes {
            match seats[change.0][change.1] {
                Seat::None => continue,
                Seat::Occupied => {
                    seats[change.0][change.1] = Seat::Empty;
                }
                Seat::Empty => {
                    seats[change.0][change.1] = Seat::Occupied;
                }
            }
        }
    }

    let mut occ_count = 0;

    for row in &seats {
        for seat in row {
            if matches!(seat, Seat::Occupied) {
                occ_count += 1;
            }
        }
    }

    occ_count
}

fn main() {
    let inputs = get_input::<String>("inputs/day-11.txt").expect("Could not parse path!");

    println!("Part 1 solution: {}", part1(&inputs));
    println!("Part 2 solution: {}", part2(&inputs));
}

#[test]
fn check() {
    let inputs = get_input::<String>("../inputs/day-11.txt").expect("Could not parse path!");
    assert_eq!(part1(&inputs), 2494);
    assert_eq!(part2(&inputs), 2306);
}
