use std::io;

fn main() {
    let input_lines = io::stdin().lines();
    let score: u32 = input_lines
        .filter_map(|l| l.ok())
        .map(|l| line_to_score(l.trim()))
        .sum();
    println!("score: {score}");
}

#[cfg_attr(test, derive(Debug))]
enum Item {
    Rock,
    Paper,
    Scissors,
}

impl Item {
    fn self_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
    fn outcome(self, other: Self) -> GameResult {
        let self_index: usize = self.into();
        let other_index: usize = other.into();
        let lu = [
            [GameResult::Draw, GameResult::Loose, GameResult::Win], // Rock
            [GameResult::Win, GameResult::Draw, GameResult::Loose], // Paper
            [GameResult::Loose, GameResult::Win, GameResult::Draw], // Scissors
        ];
        lu[self_index][other_index]
    }
}

impl From<Item> for usize {
    fn from(i: Item) -> usize {
        match i {
            Item::Rock => 0,
            Item::Paper => 1,
            Item::Scissors => 2,
        }
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,

            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Could not convert {c} to Item"),
        }
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
enum GameResult {
    Win,
    Loose,
    Draw,
}

impl GameResult {
    fn to_score(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loose => 0,
        }
    }
}

fn line_to_score(line: &str) -> u32 {
    if line.len() != 3 {
        panic!("Could not understand {line}");
    }
    let mut chars = line.chars();
    let other_item: Item = chars.next().unwrap().into();
    let _ = chars.next();
    let self_item: Item = chars.next().unwrap().into();

    let self_score = self_item.self_score();

    let game_score = self_item.outcome(other_item).to_score();
    self_score + game_score
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_item_outcome() {
        assert_eq!(Item::Rock.outcome(Item::Rock), GameResult::Draw);
        assert_eq!(Item::Rock.outcome(Item::Paper), GameResult::Loose);
        assert_eq!(Item::Rock.outcome(Item::Scissors), GameResult::Win);
        assert_eq!(Item::Paper.outcome(Item::Rock), GameResult::Win);
        assert_eq!(Item::Paper.outcome(Item::Paper), GameResult::Draw);
        assert_eq!(Item::Paper.outcome(Item::Scissors), GameResult::Loose);
        assert_eq!(Item::Scissors.outcome(Item::Rock), GameResult::Loose);
        assert_eq!(Item::Scissors.outcome(Item::Paper), GameResult::Win);
        assert_eq!(Item::Scissors.outcome(Item::Scissors), GameResult::Draw);
    }

    #[test]
    fn test_line_to_score() {
        assert_eq!(line_to_score("A Y"), 8);
        assert_eq!(line_to_score("B X"), 1);
        assert_eq!(line_to_score("C Z"), 15);
    }
}
