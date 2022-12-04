use std::{convert::TryFrom, env, io};

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

#[cfg_attr(test, derive(Debug, PartialEq))]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

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
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut range = s.split('-');
        let start = if let Some(v_str) = range.next() {
            if let Ok(v) = v_str.parse::<u32>() {
                v
            } else {
                return Err("Could not parse a start value from {s}");
            }
        } else {
            return Err("Could not get a start value from {s}");
        };
        let end = if let Some(v_str) = range.next() {
            if let Ok(v) = v_str.parse::<u32>() {
                v
            } else {
                return Err("Could not parse an end value from {s}");
            }
        } else {
            return Err("Could not get an end value from {s}");
        };
        Ok(Range { start, end })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_line_to_range_group() {
        let result = line_to_range_group("17-99,18-24");
        let ref_result = Some((Range { start: 17, end: 99 }, Range { start: 18, end: 24 }));
        assert_eq!(result, ref_result);
    }
}
