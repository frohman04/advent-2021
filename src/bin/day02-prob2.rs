fn main() {
    let cmds = std::fs::read_to_string("src/bin/day02.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(parse_line)
                .collect::<Vec<Command>>()
        })
        .expect("Unable to open file");
    let (horizontal, depth) = calculate_distance(cmds);
    println!("{:?}", horizontal * depth);
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug, PartialEq)]
struct Command {
    dir: Direction,
    dist: u32,
}

impl Command {
    fn new(dir: Direction, dist: u32) -> Command {
        Command { dir, dist }
    }
}

fn parse_line(line: &str) -> Command {
    let parts: Vec<&str> = line.split(' ').collect();
    Command::new(
        match parts[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Unknown direction {}", parts[0]),
        },
        parts[1]
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Unable to parse number from {}", parts[1])),
    )
}

fn calculate_distance(cmds: Vec<Command>) -> (u32, u32) {
    let mut horizontal = 0u32;
    let mut depth = 0u32;
    let mut aim = 0u32;
    for cmd in cmds {
        match cmd.dir {
            Direction::Forward => {
                horizontal += cmd.dist;
                depth += cmd.dist * aim;
            }
            Direction::Up => aim -= cmd.dist,
            Direction::Down => aim += cmd.dist,
        }
    }
    (horizontal, depth)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line_up() {
        assert_eq!(parse_line("up 7"), Command::new(Direction::Up, 7))
    }

    #[test]
    fn test_parse_line_down() {
        assert_eq!(parse_line("down 2"), Command::new(Direction::Down, 2))
    }

    #[test]
    fn test_parse_line_forward() {
        assert_eq!(
            parse_line("forward 10"),
            Command::new(Direction::Forward, 10)
        )
    }

    #[test]
    fn test_calculate_distance() {
        assert_eq!(
            calculate_distance(vec![
                Command::new(Direction::Forward, 5),
                Command::new(Direction::Down, 5),
                Command::new(Direction::Forward, 8),
                Command::new(Direction::Up, 3),
                Command::new(Direction::Down, 8),
                Command::new(Direction::Forward, 2)
            ]),
            (15, 60)
        )
    }
}
