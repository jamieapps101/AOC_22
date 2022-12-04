use std::{convert::TryFrom, env, io};

#[cfg(not(tarpaulin_include))]
fn main() {
    let mode_str = env::args().into_iter().nth(1).unwrap();
    let mode = mode_str
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("mode = {mode_str}"));
    let input_lines = io::stdin().lines().filter_map(|l| l.ok());

    let total = match mode {
        1 => input_lines
            .map(|l| line_to_range_group(l).unwrap())
            .filter(Range::either_contains)
            .count(),
        2 => input_lines
            .map(|l| line_to_range_group(l).unwrap())
            .filter(Range::overlaps)
            .count(),
        _ => panic!("I don't have a mode {mode}"),
    };

    println!("total: {total}");
}

#[cfg_attr(test, derive(Debug, PartialEq, Copy, Clone))]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    #[cfg(not(tarpaulin_include))]
    fn either_contains(group: &(Range, Range)) -> bool {
        group.0.contains(&group.1) || group.1.contains(&group.0)
    }

    fn overlaps(group: &(Range, Range)) -> bool {
        !((group.0.start > group.1.start && group.0.start > group.1.end)
            || (group.0.end < group.1.start && group.0.end < group.1.end))
    }
}

fn line_to_range_group<S: AsRef<str>>(line: S) -> Option<(Range, Range)> {
    let mut line_split = line.as_ref().split(',');
    let range_1 = Range::try_from(line_split.next()?).ok()?;
    let range_2 = Range::try_from(line_split.next()?).ok()?;
    Some((range_1, range_2))
}

impl TryFrom<&str> for Range {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut range = s.split('-');
        dbg!(&range);
        let start = if let Some(v_str) = range.next() {
            dbg!(&v_str);
            if let Ok(v) = v_str.parse::<u32>() {
                v
            } else {
                return Err(format!("Could not parse a start value from {s}"));
            }
        } else {
            // Unreachable as split iterator "range" will always yield one item,
            // even for empty string
            unreachable!()
        };
        let end = if let Some(v_str) = range.next() {
            if let Ok(v) = v_str.parse::<u32>() {
                v
            } else {
                return Err(format!("Could not parse an end value from {s}"));
            }
        } else {
            // Unreachable as split iterator "range" will always yield one item,
            // even for empty string
            unreachable!()
        };
        Ok(Range { start, end })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl From<(u32, u32)> for Range {
        fn from(r: (u32, u32)) -> Range {
            assert!(r.0 <= r.1);
            Range {
                start: r.0,
                end: r.1,
            }
        }
    }
    #[test]
    fn test_line_to_range_group() {
        let result = line_to_range_group("17-99,18-24");
        let ref_result = Some((Range { start: 17, end: 99 }, Range { start: 18, end: 24 }));
        assert_eq!(result, ref_result);
    }

    #[test]
    fn test_contains() {
        //    0  1  2  3  4  5  6  7  8
        //          |========|          Base Range, 2-5
        //    |==|                      A, 0-1
        //       |=====|                B, 1-3
        //             |==|             C, 3-4
        //                |====|        D, 4-6
        //                     |==|     E, 6-7
        let base: Range = (2, 5).into();
        let a = (0, 1).into();
        let b = (1, 3).into();
        let c = (3, 4).into();
        let d = (4, 6).into();
        let e = (6, 7).into();
        assert_eq!(base.contains(&a), false);
        assert_eq!(base.contains(&b), false);
        assert_eq!(base.contains(&c), true);
        assert_eq!(base.contains(&d), false);
        assert_eq!(base.contains(&e), false);
    }

    #[test]
    fn test_overlaps() {
        //    0  1  2  3  4  5  6  7  8
        //          |========|          Base Range, 2-5
        //    |==|                      A, 0-1
        //       |=====|                B, 1-3
        //             |==|             C, 3-4
        //                |====|        D, 4-6
        //                     |==|     E, 6-7
        let base: Range = (2, 5).into();
        let a = (0, 1).into();
        let b = (1, 3).into();
        let c = (3, 4).into();
        let d = (4, 6).into();
        let e = (6, 7).into();
        assert_eq!(Range::overlaps(&(base, a)), false);
        assert_eq!(Range::overlaps(&(base, b)), true);
        assert_eq!(Range::overlaps(&(base, c)), true);
        assert_eq!(Range::overlaps(&(base, d)), true);
        assert_eq!(Range::overlaps(&(base, e)), false);
    }

    #[test]
    fn test_range_try_from() {
        assert_eq!(Range::try_from("5-7"), Ok(Range { start: 5, end: 7 }));

        assert_eq!(
            Range::try_from("-4"),
            Err("Could not parse a start value from -4".to_owned())
        );

        assert_eq!(
            Range::try_from("a-4"),
            Err("Could not parse a start value from a-4".to_owned())
        );

        assert_eq!(
            Range::try_from("a-"),
            Err("Could not parse a start value from a-".to_owned())
        );
    }
}
