extern crate regex;
use regex::Regex;
// use std::collections::HashMap;

pub struct Password {
    required_char: char,
    // min and max may be confused in part 2,
    // since it's position index
    min: usize,
    max: usize,
    content: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    // input.parse().unwrap()
    // input.trim().lines().map(|x| x.parse().unwrap()).collect()
    let re = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)").unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            // TODO: this looks a little complicated
            let caps = re.captures(line).unwrap();
            let min = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let max = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
            let required_char = caps
                .get(3)
                .map_or("", |m| m.as_str())
                .chars()
                .next()
                .unwrap();
            let content = caps.get(4).map_or("", |m| m.as_str()).to_string();
            Password {
                required_char,
                min,
                max,
                content,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Password>) -> i32 {
    let mut valid_count = 0;
    for password in input {
        let mut char_count = 0;
        for c in password.content.chars() {
            if c == password.required_char {
                char_count = char_count + 1;
            }
            if char_count > password.max {
                break;
            }
        }
        if char_count >= password.min && char_count <= password.max {
            valid_count += 1;
        }
    }
    valid_count
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Password>) -> i32 {
    let mut valid_count = 0;
    for password in input {
        let mut char_count = 0;
        for (i, c) in password.content.chars().enumerate() {
            // no zero index!
            if i == password.min - 1 && c == password.required_char {
                char_count = char_count + 1;
            } else if i == password.max - 1 && c == password.required_char {
                char_count = char_count + 1;
                break;
            }
        }
        if char_count == 1 {
            valid_count += 1;
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn part1_sample1() {
        let input = input_generator("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(part1(&input), 2);
    }
    #[test]
    fn part2_sample1() {
        let input = input_generator("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc");
        assert_eq!(part2(&input), 1);
    }
}
