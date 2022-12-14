use std::io;

fn main() {
    let mut lines = io::stdin().lines().filter_map(|l| l.ok());
    let mut columns = Columns::from(&mut lines);
    // println!("Before\n{}", columns);
    let action_lines = lines.skip(1);
    let mut swap_buffer = vec![0u8; 100_000_000];
    action_lines
        .filter_map(|l| Action::try_from(l).ok())
        .enumerate()
        .for_each(|(index, action)| {
            if index % 1000 == 0 {
                println!("index: {}", index)
            };
            let action_from_1 = action.from - 1;
            let len_to_swap = columns.columns[action_from_1].items.len();
            let swap_buffer_l = &mut swap_buffer[0..action.quantity];
            swap_buffer_l.copy_from_slice(
                &columns.columns[action_from_1].items[len_to_swap - action.quantity..len_to_swap],
            );
            columns.columns[action_from_1]
                .items
                .truncate(len_to_swap - action.quantity);
            columns.columns[action.to - 1]
                .items
                .extend(swap_buffer_l.iter());
        });
    // println!("After\n{}", columns);
    let tops = String::from_iter(columns.tops().into_iter().map(|u| u as char));
    println!("Tops: {tops}");
}

#[cfg_attr(test, derive(PartialEq, Debug))]
struct Columns<I> {
    columns: Vec<Column<I>>,
}

#[cfg_attr(test, derive(PartialEq, Debug))]
struct Column<I> {
    items: Vec<I>,
}

impl<I> From<Vec<I>> for Column<I> {
    fn from(items: Vec<I>) -> Self {
        Self { items }
    }
}

impl<I: AsRef<str>, S: Iterator<Item = I>> From<&mut S> for Columns<u8> {
    fn from(source: &mut S) -> Self {
        let mut data: Vec<Vec<Option<char>>> = Vec::new();
        for s in source {
            let mut space_count = 1;
            let mut line_data = Vec::new();
            let line = s.as_ref();
            if !line.contains('[') {
                break;
            }
            for c in line.chars() {
                match c {
                    ' ' => {
                        space_count += 1;
                        if space_count == 4 {
                            space_count = 0;
                            line_data.push(None);
                        }
                    }
                    '[' | ']' | '\n' => space_count = 0, // do nothing
                    _ => {
                        space_count = 0;
                        line_data.push(Some(c))
                    }
                }
            }
            data.push(line_data);
        }

        let col_count = data[..].iter().map(|l| l.len()).max().unwrap();
        let mut columns = Vec::new();

        for i in 0..col_count {
            let mut col_data = Vec::new();
            for line_index in (0..data.len()).rev() {
                let line = &data[line_index];
                if line.len() > i {
                    if let Some(c) = line[i] {
                        col_data.push(c as u8);
                    } else {
                        break;
                    }
                }
            }
            columns.push(col_data.into());
        }

        Self { columns }
    }
}

impl<I: Copy + Clone> Columns<I> {
    fn tops(&self) -> Vec<I> {
        self.columns
            .iter()
            .filter_map(|c| c.items.last())
            .copied()
            .collect()
    }
}

use std::fmt;
impl<I: std::fmt::Display> fmt::Display for Columns<I> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // determine highest col
        let high_point = self
            .columns
            .iter()
            .enumerate()
            .max_by(|x, y| x.1.items.len().cmp(&y.1.items.len()))
            .unwrap();
        for d in (0..high_point.1.items.len()).rev() {
            for c in &self.columns {
                if c.items.len() > d {
                    write!(f, " [{}]", c.items[d])?;
                } else {
                    write!(f, "    ")?;
                };
            }
            writeln!(f)?;
        }
        for c_index in 0..self.columns.len() {
            write!(f, "  {} ", c_index)?;
        }
        Ok(())
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
struct Action {
    quantity: usize,
    from: usize,
    to: usize,
}

use std::convert::TryFrom;
impl TryFrom<String> for Action {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut split = value.split(' ');
        let quantity = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();
        Ok(Action { quantity, from, to })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_columns_tops() {
        let columns = Columns {
            columns: vec![
                Column {
                    items: vec!['Z', 'N'],
                },
                Column {
                    items: vec!['M', 'C', 'D'],
                },
                Column { items: vec!['P'] },
            ],
        };

        assert_eq!(columns.tops(), vec!['N', 'D', 'P']);
    }

    #[test]
    fn test_action_try_from() {
        let a = Action::try_from("move 1 from 2 to 1".to_owned());
        let ref_a = Ok(Action {
            quantity: 1,
            from: 2,
            to: 1,
        });
        assert_eq!(a, ref_a)
    }
}
