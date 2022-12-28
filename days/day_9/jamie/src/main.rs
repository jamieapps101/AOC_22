use std::io::stdin;

fn main() {
    let commands = stdin()
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| Command::try_from(l).unwrap());

    let mut rope = Rope::init(10);

    let coord_log: Vec<Coord> = rope
        .apply_multiple(commands)
        .into_iter()
        .flatten()
        .collect();

    // coord_log.sort_by(|a, b| {
    //     a.x.partial_cmp(&b.x)
    //         .or_else(|| a.y.partial_cmp(&b.y))
    //         .unwrap()
    // });

    let (_count_log, position_log) = inspect_log(coord_log);
    println!("unique locations visited: {}", position_log.len());
}

fn inspect_log(coord_log: Vec<Coord>) -> (Vec<usize>, Vec<Coord>) {
    let mut position_log = vec![];
    let mut count_log = vec![];
    if !coord_log.is_empty() {
        for &c in coord_log.iter() {
            let search_result =
                position_log.iter().enumerate().find_map(
                    |(i, &c_l)| {
                        if c_l == c {
                            Some(i)
                        } else {
                            None
                        }
                    },
                );
            if let Some(index) = search_result {
                count_log[index] += 1;
            } else {
                position_log.push(c);
                count_log.push(1);
            }
        }

        let most_visited_index = count_log
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1))
            .unwrap()
            .0;

        let most_visited_coord = position_log[most_visited_index];
        println!(
            "most visited: {}, {} times",
            most_visited_coord, most_visited_index
        );
        println!("{} location visited", count_log.len());
    } else {
        println!("No coord log");
    }
    (count_log, position_log)
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
struct Command {
    dir: Direction,
    dist: i32,
}

impl Command {
    fn split(self) -> CommandSplitter {
        CommandSplitter {
            command: self,
            state: 0,
        }
    }
}

struct CommandSplitter {
    command: Command,
    state: i32,
}

impl Iterator for CommandSplitter {
    type Item = Command;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state < self.command.dist {
            self.state += 1;
            Some(Command {
                dir: self.command.dir,
                dist: 1,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum CommandParseErr {
    NoDirStr,
    NoDestStr,
    AdditionalItems(String),
    IncorrectDirSymbol(String),
    DistParseErr(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for CommandParseErr {
    fn from(e: std::num::ParseIntError) -> Self {
        CommandParseErr::DistParseErr(e)
    }
}

impl TryFrom<String> for Command {
    type Error = CommandParseErr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut spliterator = value.split_whitespace();
        let dir_str = spliterator.next().ok_or(CommandParseErr::NoDirStr)?;
        let dist_str = spliterator.next().ok_or(CommandParseErr::NoDestStr)?;
        if let Some(i) = spliterator.next() {
            return Err(CommandParseErr::AdditionalItems(i.to_owned()));
        }
        let dir = match dir_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(CommandParseErr::IncorrectDirSymbol(dir_str.to_owned())),
        };
        let dist = dist_str.parse::<i32>()?;
        Ok(Command { dir, dist })
    }
}

#[derive(Clone, PartialEq, Debug, Default, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn apply(&mut self, c: Command) {
        match c.dir {
            Direction::Up => self.y += c.dist,
            Direction::Left => self.x -= c.dist,
            Direction::Right => self.x += c.dist,
            Direction::Down => self.y -= c.dist,
        }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<[i32; 2]> for Coord {
    fn from(value: [i32; 2]) -> Self {
        Coord {
            x: value[0],
            y: value[1],
        }
    }
}

// impl Coord {
//     fn abs(&self) -> f32 {
//         ((self.x as f32).powi(2) +  (self.y as f32).powi(2)).powf(0.5)
//     }
// }

#[derive(Default)]
struct Rope {
    head: Coord,
    tail: Vec<Coord>,
}

impl Rope {
    fn init(len: usize) -> Self {
        Rope {
            head: Coord::default(),
            tail: vec![Coord::default(); len - 1],
        }
    }

    fn apply_multiple<S: Iterator<Item = Command>>(&mut self, source: S) -> Vec<Vec<Coord>> {
        let mut rtn_vec = vec![vec![*self.tail.last().unwrap()]];
        rtn_vec.extend(source.map(|c| {
            // apply command to head
            c.split()
                .map(|c_s| {
                    self.apply(c_s);
                    *self.tail.last().unwrap()
                })
                .collect::<Vec<Coord>>()
        }));
        rtn_vec
    }

    /// Apply command to head of rope, then apply reaction behaviour to tail
    fn apply(&mut self, c: Command) {
        self.head.apply(c);
        // apply reaction to tail
        for i in 0..self.tail.len() {
            let ref_knot = if i == 0 { self.head } else { self.tail[i - 1] };
            let current_knot = &mut self.tail[i];

            let x_diff = ref_knot.x - current_knot.x;
            let y_diff = ref_knot.y - current_knot.y;
            if x_diff.abs() >= 2 || y_diff.abs() >= 2 {
                if x_diff != 0 {
                    current_knot.x += x_diff / x_diff.abs()
                }
                if y_diff != 0 {
                    current_knot.y += y_diff / y_diff.abs()
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_parse() {
        let input_str = vec![
            "R 4".to_owned(),
            "U 4".to_owned(),
            "L 3".to_owned(),
            "D 1".to_owned(),
            "R 4".to_owned(),
            "D 1".to_owned(),
            "L 5".to_owned(),
            "R 2".to_owned(),
        ];

        let dut_commands = input_str
            .into_iter()
            .filter_map(|s| Command::try_from(s).ok())
            .collect::<Vec<Command>>();

        let ref_commands = vec![
            Command {
                dir: Direction::Right,
                dist: 4,
            },
            Command {
                dir: Direction::Up,
                dist: 4,
            },
            Command {
                dir: Direction::Left,
                dist: 3,
            },
            Command {
                dir: Direction::Down,
                dist: 1,
            },
            Command {
                dir: Direction::Right,
                dist: 4,
            },
            Command {
                dir: Direction::Down,
                dist: 1,
            },
            Command {
                dir: Direction::Left,
                dist: 5,
            },
            Command {
                dir: Direction::Right,
                dist: 2,
            },
        ];

        assert_eq!(dut_commands, ref_commands);
    }

    #[test]
    fn test_apply_multiple() {
        let input_str = vec![
            "R 4".to_owned(),
            "U 4".to_owned(),
            "L 3".to_owned(),
            "D 1".to_owned(),
            "R 4".to_owned(),
            "D 1".to_owned(),
            "L 5".to_owned(),
            "R 2".to_owned(),
        ];

        let commands = input_str
            .into_iter()
            .filter_map(|s| Command::try_from(s).ok())
            .collect::<Vec<Command>>();

        let mut rope = Rope::init(2);

        let ref_log: Vec<Vec<Coord>> = vec![
            // Initial state
            vec![[0, 0].into()],
            // R 4
            vec![[0, 0].into(), [1, 0].into(), [2, 0].into(), [3, 0].into()],
            // U 4
            vec![[3, 0].into(), [4, 1].into(), [4, 2].into(), [4, 3].into()],
            // L 3
            vec![[4, 3].into(), [3, 4].into(), [2, 4].into()],
            // D 1
            vec![[2, 4].into()],
        ];

        let log = rope.apply_multiple(commands.into_iter());
        println!("log:\n{log:#?}");

        for (ref_step, step) in ref_log.iter().zip(log[..6].iter()) {
            assert_eq!(ref_step, step);
        }
    }

    #[test]
    fn test_command_splitter() {
        let c = Command {
            dir: Direction::Up,
            dist: 4,
        };
        let dut_sub_c: Vec<Command> = c.split().collect();

        let ref_sub_c = vec![
            Command {
                dir: Direction::Up,
                dist: 1,
            },
            Command {
                dir: Direction::Up,
                dist: 1,
            },
            Command {
                dir: Direction::Up,
                dist: 1,
            },
            Command {
                dir: Direction::Up,
                dist: 1,
            },
        ];

        assert_eq!(dut_sub_c.len(), 4);
        assert_eq!(dut_sub_c, ref_sub_c);
    }

    #[test]
    fn test_apply_multiple_10_knot() {
        let input_res_0 = (
            vec![
                "R 4".to_owned(),
                "U 4".to_owned(),
                "L 3".to_owned(),
                "D 1".to_owned(),
                "R 4".to_owned(),
                "D 1".to_owned(),
                "L 5".to_owned(),
                "R 2".to_owned(),
            ],
            1,
        );

        let input_res_1 = (
            vec![
                "R 5".to_owned(),
                "U 8".to_owned(),
                "L 8".to_owned(),
                "D 3".to_owned(),
                "R 17".to_owned(),
                "D 10".to_owned(),
                "L 25".to_owned(),
                "U 20".to_owned(),
            ],
            36,
        );

        for (input_str, res) in [input_res_0, input_res_1] {
            let commands = input_str
                .into_iter()
                .filter_map(|s| Command::try_from(s).ok())
                .collect::<Vec<Command>>();

            let mut rope = Rope::init(10);

            let log = rope.apply_multiple(commands.into_iter());
            let flat_log = log.iter().flatten().copied().collect();
            let (_count_log, position_log) = inspect_log(flat_log);

            assert_eq!(position_log.len(), res);
        }
    }

    #[test]
    fn test_inspect_log() {
        let ref_log = vec![
            [0, 0].into(),
            [0, 0].into(),
            [0, 0].into(),
            [0, 1].into(),
            [0, 0].into(),
            [0, 0].into(),
            [0, 2].into(),
            [0, 1].into(),
            [0, 0].into(),
            [0, 3].into(),
            [0, 2].into(),
            [0, 1].into(),
        ];
        let (count_log, position_log) = inspect_log(ref_log);

        let ref_count_log = vec![6, 3, 2, 1];
        let ref_position_log: Vec<Coord> =
            vec![[0, 0].into(), [0, 1].into(), [0, 2].into(), [0, 3].into()];

        assert_eq!(count_log, ref_count_log);
        assert_eq!(position_log, ref_position_log);
    }
}
