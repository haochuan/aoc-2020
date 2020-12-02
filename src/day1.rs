use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    // input.parse().unwrap()
    input.trim().lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
// same as LC1 two sum lol
pub fn part1(input: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for number in input {
        if map.contains_key(&number) {
            return number * map.get(&number).unwrap();
        } else {
            let key = 2020 - number;
            map.insert(key, *number);
        }
    }
    0
}

#[aoc(day1, part2)]
// same as LC15 3 sum, but no duplication limitation
pub fn part2(input: &Vec<i32>) -> i32 {
    let mut data = input.clone();
    data.sort();
    for i in 0..data.len() - 2 {
        let head = data[i];
        let remain = 2020 - head;
        let mut start = i + 1;
        let mut end = data.len() - 1;
        while start < end {
            if data[start] + data[end] > remain {
                end = end - 1;
            } else if data[start] + data[end] < remain {
                start = start + 1;
            } else {
                return head * data[start] * data[end];
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn part1_sample1() {
        let input = input_generator("1721\n979\n366\n299\n675\n1456");
        assert_eq!(part1(&input), 514579);
    }
    #[test]
    fn part2_sample1() {
        let input = input_generator("1721\n979\n366\n299\n675\n1456");
        assert_eq!(part2(&input), 241861950);
    }
}
