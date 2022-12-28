use std::io::stdin;

#[cfg(not(tarpaulin_include))]
fn main() {
    let line_iter = stdin().lines().filter_map(|l| l.ok());
    let instr_iter = line_iter.map(|l| Instruction::try_from(l).unwrap());
    let mut computer = Computer::from(instr_iter);
    let mut screen = Crt::<40, 6>::default();
    // let mut sum = 0;
    loop {
        // let signal_strength = computer.register * cycle;
        screen.update(Some(computer.register));
        if computer.clock().is_err() {
            println!("Ran out of instructions");
            break;
        }
        // if vec![20, 60, 100, 140, 180, 220].contains(&cycle) {
        //     sum += signal_strength
        // }
    }
    println!("{screen}");
    // println!("signal strength sum = {}", sum);
}

#[cfg_attr(test, derive(Debug, PartialEq))]
enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug)]
enum InstructionParseErr {
    Unparseable,
    NoAddxVal,
    ValParseErr(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for InstructionParseErr {
    fn from(e: std::num::ParseIntError) -> Self {
        InstructionParseErr::ValParseErr(e)
    }
}

impl TryFrom<String> for Instruction {
    type Error = InstructionParseErr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(InstructionParseErr::Unparseable);
        }
        match value.split_at(4).0 {
            "noop" => Ok(Instruction::Noop),
            "addx" => {
                let value_str = value
                    .split_whitespace()
                    .nth(1)
                    .ok_or(InstructionParseErr::NoAddxVal)?;
                let value = value_str.parse::<i32>()?;
                Ok(Instruction::Addx(value))
            }
            _ => Err(InstructionParseErr::Unparseable),
        }
    }
}

enum ComputerState {
    Add(i32),
    Ready,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
enum ComputerErr {
    NoInstructions,
}

struct Computer<I: Iterator<Item = Instruction>> {
    instruction_source: I,
    register: i32,
    state: ComputerState,
}

impl<I: Iterator<Item = Instruction>> From<I> for Computer<I> {
    fn from(source: I) -> Self {
        Self {
            instruction_source: source,
            register: 1,
            state: ComputerState::Ready,
        }
    }
}

impl<I: Iterator<Item = Instruction>> Computer<I> {
    fn clock(&mut self) -> Result<(), ComputerErr> {
        match self.state {
            ComputerState::Ready => {
                if let Some(instr) = self.instruction_source.next() {
                    match instr {
                        Instruction::Noop => Ok(()),
                        Instruction::Addx(val) => {
                            self.state = ComputerState::Add(val);
                            Ok(())
                        }
                    }
                } else {
                    Err(ComputerErr::NoInstructions)
                }
            }
            ComputerState::Add(val) => {
                self.register += val;
                self.state = ComputerState::Ready;
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy)]
enum PixelState {
    Lit,
    Dark,
}

impl Default for PixelState {
    fn default() -> Self {
        Self::Dark
    }
}

impl std::fmt::Display for PixelState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PixelState::Dark => write!(f, "."),
            PixelState::Lit => write!(f, "#"),
        }
    }
}

struct Crt<const W: usize, const H: usize> {
    next_pixel: usize,
    current_row: usize,
    screen: [[PixelState; W]; H],
}

impl<const W: usize, const H: usize> Default for Crt<W, H> {
    fn default() -> Self {
        Self {
            next_pixel: 0,
            current_row: 0,
            screen: [[PixelState::default(); W]; H],
        }
    }
}

impl<const W: usize, const H: usize> Crt<W, H> {
    fn update(&mut self, sprite_position: Option<i32>) {
        // update the screen state to reflect the new
        let current_pixel = &mut self.screen[self.current_row][self.next_pixel];

        if let Some(position) = sprite_position {
            *current_pixel = if (self.next_pixel == position as usize + 1)
                || (self.next_pixel == position as usize)
                || (position > 1 && self.next_pixel == position as usize - 1)
            {
                PixelState::Lit
            } else {
                PixelState::Dark
            };
        }

        self.next_pixel = (self.next_pixel + 1) % W;
        if self.next_pixel == 0 {
            self.current_row = (self.current_row + 1) % H;
        }
    }

    fn render(&self) -> Vec<String> {
        self.screen
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| format!("{pixel}"))
                    .collect::<String>()
            })
            .collect()
    }
}

impl<const W: usize, const H: usize> std::fmt::Display for Crt<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.render() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{self, BufRead};

    use super::*;

    #[test]
    fn small_program() {
        let program = vec!["noop", "addx 3", "addx -5"];

        let instructions: Vec<Instruction> = program
            .into_iter()
            .filter_map(|l| Instruction::try_from(l.to_owned()).ok())
            .collect();

        assert_eq!(
            instructions,
            vec![
                Instruction::Noop,
                Instruction::Addx(3),
                Instruction::Addx(-5)
            ]
        );

        let mut comp = Computer::from(instructions.into_iter());

        assert_eq!(comp.register, 1); // pre-cycle
        assert_eq!(comp.clock(), Ok(())); // cycle 1
        assert_eq!(comp.register, 1);
        assert_eq!(comp.clock(), Ok(())); // cycle 2
        assert_eq!(comp.register, 1);
        assert_eq!(comp.clock(), Ok(())); // cycle 3
        assert_eq!(comp.register, 4);
        assert_eq!(comp.clock(), Ok(())); // cycle 4
        assert_eq!(comp.register, 4);
        assert_eq!(comp.clock(), Ok(())); // cycle 5
        assert_eq!(comp.register, -1);
        assert_eq!(comp.clock(), Err(ComputerErr::NoInstructions));
    }

    #[test]
    fn signal_strength_example() {
        let fp = File::open("test_input.txt").unwrap();
        let buf_read = io::BufReader::new(fp);
        let instructions: Vec<Instruction> = buf_read
            .lines()
            .into_iter()
            .map(|l| l.unwrap())
            .filter_map(|l| Instruction::try_from(l).ok())
            .collect();

        let mut comp = Computer::from(instructions.into_iter());

        for i in 1..221 {
            let signal_strength = comp.register * i;
            if i == 20 {
                assert_eq!(signal_strength, 420)
            }
            if i == 60 {
                assert_eq!(signal_strength, 1140)
            }
            if i == 100 {
                assert_eq!(signal_strength, 1800)
            }
            if i == 140 {
                assert_eq!(signal_strength, 2940)
            }
            if i == 180 {
                assert_eq!(signal_strength, 2880)
            }
            if i == 220 {
                assert_eq!(signal_strength, 3960)
            }
            assert_eq!(comp.clock(), Ok(()));
        }
    }
}
