use std::io;

#[cfg(not(tarpaulin_include))]
fn main() {
    let line_source = io::stdin().lines().filter_map(|l| l.ok());
    for (line_index, line) in line_source.enumerate() {
        if let Some(start_position) = find_start_position::<4, _>(&line) {
            println!("Line {line_index} start position: {start_position:?}")
        }
    }
}

fn find_start_position<const BUFF_SIZE: usize, S: AsRef<str>>(source: S) -> Option<usize> {
    let t = source
        .as_ref()
        .chars()
        .decode::<BUFF_SIZE>()
        .enumerate()
        .find(|(_index, packet)| gen_buffer_neq(packet));

    t.map(|(index, _packet)| index + 1)
}

trait PacketBuffer<T: Copy, I: Iterator<Item = T>> {
    fn decode<const BUFF_SIZE: usize>(self) -> Decoder<T, BUFF_SIZE, I>;
}

struct Decoder<T: Copy, const BUFF_SIZE: usize, I: Iterator<Item = T>> {
    source: I,
    buffer: [Option<T>; BUFF_SIZE],
    buffer_index: usize,
}

fn gen_buffer_neq<const BUFF_SIZE: usize, I: PartialEq>(buffer: &[Option<I>; BUFF_SIZE]) -> bool {
    if !buffer.iter().all(|i| i.is_some()) {
        return false;
    }
    for i in 1..BUFF_SIZE {
        for j in 0..i {
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    true
}

impl<T: Copy, I: Iterator<Item = T>> PacketBuffer<T, I> for I {
    fn decode<const BUFF_SIZE: usize>(self) -> Decoder<T, BUFF_SIZE, I> {
        Decoder {
            source: self,
            buffer: [None; BUFF_SIZE],
            buffer_index: 0,
        }
    }
}

impl<T: Copy, const BUFF_SIZE: usize, I: Iterator<Item = T>> Iterator for Decoder<T, BUFF_SIZE, I> {
    type Item = [Option<T>; BUFF_SIZE];
    fn next(&mut self) -> Option<Self::Item> {
        let next_item = self.source.next();
        next_item?;
        self.buffer[self.buffer_index] = next_item;
        self.buffer_index = (self.buffer_index + 1) % BUFF_SIZE;
        Some(self.buffer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // impl<const BUFF_SIZE: usize> From<[I]>
    #[test]
    fn test_gen_buffer_neq() {
        let buf = [Some('A'), Some('B'), Some('C'), Some('D')];
        assert_eq!(gen_buffer_neq(&buf), true);

        let buf = [Some('A'), Some('A'), Some('C'), Some('D')];
        assert_eq!(gen_buffer_neq(&buf), false);

        let buf = [Some('A'), Some('B'), Some('A'), Some('D')];
        assert_eq!(gen_buffer_neq(&buf), false);

        let buf = [Some('A'), Some('B'), Some('C'), Some('A')];
        assert_eq!(gen_buffer_neq(&buf), false);

        let buf = [Some('A'), Some('B'), Some('B'), Some('A')];
        assert_eq!(gen_buffer_neq(&buf), false);

        let buf = [Some('A'), Some('B'), Some('C'), Some('B')];
        assert_eq!(gen_buffer_neq(&buf), false);

        let buf = [Some('A'), Some('B'), Some('C'), Some('C')];
        assert_eq!(gen_buffer_neq(&buf), false);

        let buf = [None, Some('B'), Some('C'), Some('D')];
        assert_eq!(gen_buffer_neq(&buf), false);
    }

    #[test]
    fn test_find_start_position() {
        assert_eq!(
            find_start_position::<4, _>("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(5)
        );
        assert_eq!(
            find_start_position::<4, _>("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(6)
        );
        assert_eq!(
            find_start_position::<4, _>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(10)
        );
        assert_eq!(
            find_start_position::<4, _>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(11)
        );
    }
}
