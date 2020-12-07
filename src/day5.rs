use std::cmp::max;
use std::collections::HashSet;

pub struct Seat {
    id: u32,
}

impl Seat {
    fn new(row: u32, col: u32) -> Self {
        Self { id: row * 8 + col }
    }
}

fn calculate(data: &str, row: u32) -> u32 {
    let mut start = 0;
    let mut end = row - 1;

    for c in data.chars() {
        match c {
            'B' | 'R' => start = ((start + end) / 2) + 1,
            'F' | 'L' => end = (start + end) / 2,
            _ => (),
        }
    }

    if start == end {
        start
    } else {
        panic!("Not found.")
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|line| {
            let row_info = &line[..7];
            let col_info = &line[7..];
            Seat::new(calculate(row_info, 128), calculate(col_info, 8))
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &Vec<Seat>) -> u32 {
    let mut max_id = 0;
    for seat in input {
        max_id = max(max_id, seat.id);
    }
    max_id
}

#[aoc(day5, part2)]
pub fn part2(input: &Vec<Seat>) -> u32 {
    let mut hash_set: HashSet<u32> = HashSet::new();
    for seat in input {
        hash_set.insert(seat.id);
    }

    // 991 is the max from part 1
    for i in 1..991 {
        if hash_set.contains(&(i as u32 - 1))
            && hash_set.contains(&(i as u32 + 1))
            && !hash_set.contains(&(i as u32))
        {
            return i;
        }
    }
    panic!("Seat not found.")
}

#[cfg(test)]
mod tests {
    use super::{calculate, input_generator, part1};

    #[test]
    fn part1_sample1() {
        let text = concat!("BFFFBBFRRR\n", "FFFBBBFRRR\n", "BBFFBBFRLL");
        let input = input_generator(text);
        assert_eq!(part1(&input), 820);
    }

    #[test]
    fn test_calculate_row() {
        let data = "FBFBBFF";
        let row = calculate(data, 128);
        assert_eq!(row, 44);
    }
    #[test]
    fn test_calculate_col() {
        let data = "RLR";
        let row = calculate(data, 8);
        assert_eq!(row, 5);
    }
}
