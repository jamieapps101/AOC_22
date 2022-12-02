use std::io;

fn main() {
    let lines = io::stdin().lines();
    let mut numbers = lines_to_numbers(lines);
    if let Some(top_value) = sum_top::<1, _>(&mut numbers) {
        println!("Top Value: {}", top_value);
    }
    if let Some(top_3) = sum_top::<3, _>(&mut numbers) {
        println!("Summed Top 3 Values: {}", top_3);
    }
}

fn lines_to_numbers<E: std::fmt::Debug, S: Iterator<Item = Result<String, E>>>(s: S) -> Vec<u32> {
    let mut sum = 0;
    let mut numbers: Vec<u32> = s
        .filter_map(|l| l.ok())
        .map(|l| l.trim().parse::<u32>().ok())
        .filter_map(|n_opt| match n_opt {
            Some(n) => {
                sum += n;
                None
            }
            None => Some(std::mem::take(&mut sum)),
        })
        .collect();
    // handle case where last line of input is a number not a new line
    if sum != u32::default() {
        numbers.push(sum)
    }
    numbers
}

fn sum_top<'a, const C: usize, T: std::iter::Sum<&'a T> + Ord>(s: &'a mut [T]) -> Option<T> {
    if s.len() >= C {
        s.sort_unstable();
        return Some(s[s.len() - C..s.len()].iter().sum::<T>());
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_lines_to_numbers_0() {
        let example = vec![
            Ok("1\n".to_owned()),
            Err("No number..."),
            Ok("2\n".to_owned()),
            Ok("3\n".to_owned()),
            Ok("\n".to_owned()),
            Ok("4\n".to_owned()),
            Ok("5\n".to_owned()),
            Ok("6\n".to_owned()),
            Ok("\n".to_owned()),
        ];
        let out = lines_to_numbers(example.into_iter());
        assert_eq!(out, vec![6, 15]);
    }

    #[test]
    fn test_lines_to_numbers_1() {
        let example = vec![
            Ok("1\n".to_owned()),
            Err("No number..."),
            Ok("2\n".to_owned()),
            Ok("3\n".to_owned()),
            Ok("\n".to_owned()),
            Ok("4\n".to_owned()),
            Ok("5\n".to_owned()),
            Ok("6\n".to_owned()),
        ];
        let out = lines_to_numbers(example.into_iter());
        assert_eq!(out, vec![6, 15]);
    }

    #[test]
    fn test_lines_to_numbers_2() {
        let example = vec![
            Err("No number..."),
            Err("No number..."),
            Err("No number..."),
            Err("No number..."),
            Err("No number..."),
        ];
        let out = lines_to_numbers(example.into_iter());
        assert_eq!(out, vec![]);
    }

    #[test]
    fn test_sum_top_0() {
        let mut v = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        let res = sum_top::<0, u32>(&mut v);
        assert_eq!(res, Some(0));
    }

    #[test]
    fn test_sum_top_1() {
        let mut v = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        let res = sum_top::<1, u32>(&mut v);
        assert_eq!(res, Some(5));
    }

    #[test]
    fn test_sum_top_3() {
        let mut v = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        let res = sum_top::<3, u32>(&mut v);
        assert_eq!(res, Some(12));
    }

    #[test]
    fn test_sum_top_6() {
        let mut v = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        let res = sum_top::<6, u32>(&mut v);
        assert_eq!(res, Some(15));
    }

    #[test]
    fn test_sum_top_7() {
        let mut v = [0u32, 1u32, 2u32, 3u32, 4u32, 5u32];
        let res = sum_top::<7, u32>(&mut v);
        assert_eq!(res, None);
    }
}
