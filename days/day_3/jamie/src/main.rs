use std::env;
use std::io;

fn main() {
    let mode_str = env::args().into_iter().nth(1).unwrap();
    let mode = mode_str
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("mode = {mode_str}"));

    let input_lines = io::stdin().lines().filter_map(|l| l.ok());
    let score: u32 = match mode {
        1 => find_score(input_lines),
        2 => input_lines
            .chonk::<3>()
            .filter_map(|group| {
                let c = find_common_items_between(&group[0], &group[1]);
                let mut c =
                    find_common_items_between(&group[2], c.iter().collect::<String>().as_ref());
                c.dedup();
                c.pop()
            })
            .map(item_to_score)
            .sum(),
        _ => panic!("I only work with mode 1 or 2"),
    };
    println!("Score: {score}");
}

fn find_score<I: AsRef<str>, S: Iterator<Item = I>>(source: S) -> u32 {
    source
        .filter_map(|l| find_common_item(l.as_ref()))
        .map(item_to_score)
        .sum()
}

fn item_to_score(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => panic!("Did not expect a {item}"),
    }
}

/// Given a string representing unique items in both compartments, find the
/// common item.
fn find_common_item(contents: &str) -> Option<char> {
    let length = contents.len();
    let (compartment_a, compartment_b) = contents.split_at(length / 2);
    find_common_item_between(compartment_a, compartment_b)
}

fn find_common_item_between(a: &str, b: &str) -> Option<char> {
    let b_chars: Vec<char> = b.chars().collect();
    a.chars().into_iter().find(|&ac| b_chars.contains(&ac))
}

fn find_common_items_between(a: &str, b: &str) -> Vec<char> {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    a_chars
        .iter()
        .filter(|ac| b_chars.contains(ac))
        .copied()
        .collect()
}

trait Chonk<I, S: Iterator<Item = I>> {
    fn chonk<const N: usize>(self) -> Chonker<N, I, S>;
}

struct Chonker<const N: usize, I, S: Iterator<Item = I>> {
    source: S,
}

impl<I, S: Iterator<Item = I>> Chonk<I, S> for S {
    fn chonk<const N: usize>(self) -> Chonker<N, I, S> {
        Chonker { source: self }
    }
}

impl<const N: usize, I, S: Iterator<Item = I>> Iterator for Chonker<N, I, S> {
    type Item = Vec<I>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut t = Vec::with_capacity(N);
        for _ in 0..N {
            t.push(self.source.next()?);
        }
        Some(t)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_item_to_score_ok() {
        assert_eq!(item_to_score('a'), 1);
        assert_eq!(item_to_score('z'), 26);
        assert_eq!(item_to_score('A'), 27);
        assert_eq!(item_to_score('Z'), 52);
    }

    #[test]
    #[should_panic]
    fn test_item_to_score_not_ok() {
        let res = item_to_score('!');
        dbg!(res);
    }

    #[test]
    fn test_find_common_item() {
        assert_eq!(find_common_item("vJrwpWtwJgWrhcsFMMfFFhFp"), Some('p'));
        assert_eq!(
            find_common_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Some('L')
        );
        assert_eq!(find_common_item("PmmdzqPrVvPwwTWBwg"), Some('P'));
        assert_eq!(
            find_common_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Some('v')
        );
        assert_eq!(find_common_item("ttgJtRGJQctTZtZT"), Some('t'));
        assert_eq!(find_common_item("CrZsJsPPZsGzwwsLwLmpwMDw"), Some('s'));
    }

    #[test]
    fn test_find_score() {
        let ref_data = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_owned(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_owned(),
            "PmmdzqPrVvPwwTWBwg".to_owned(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_owned(),
            "ttgJtRGJQctTZtZT".to_owned(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_owned(),
        ];
        let score = find_score(ref_data.iter());
        assert_eq!(score, 157);
    }
}
