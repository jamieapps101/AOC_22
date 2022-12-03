use std::io;

fn main() {
    let input_lines = io::stdin().lines().filter_map(|l| l.ok());
    let score = find_score(input_lines);
    println!("Score: {score}");
}

fn find_score<I: AsRef<str>, S: Iterator<Item = I>>(source: S) -> u32 {
    source
        .filter_map(|l| find_common_item(l.as_ref()))
        .map(|c| item_to_score(c))
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
    let compartment_a_chars: Vec<char> = compartment_a.chars().collect();
    let compartment_b_chars: Vec<char> = compartment_b.chars().collect();
    for cac in compartment_a_chars {
        if compartment_b_chars.contains(&cac) {
            return Some(cac);
        }
    }
    None
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
