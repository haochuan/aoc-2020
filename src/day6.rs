use std::collections::{HashMap, HashSet};

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut set: HashSet<char> = HashSet::new();
        for line in group.lines() {
            for c in line.chars() {
                set.insert(c);
            }
        }
        sum = sum + set.len();
    }
    sum
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum = 0;
    for group in input.split("\n\n") {
        let mut map: HashMap<char, usize> = HashMap::new();
        let num_of_people = group.lines().count();
        for line in group.lines() {
            for c in line.chars() {
                let count = map.entry(c).or_insert(0);
                *count += 1;
            }
        }
        sum = sum
            + map
                .values()
                .filter(|x| *x == &num_of_people)
                .collect::<Vec<&usize>>()
                .len();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn part1_sample1() {
        let text = concat!(
            "abc\n", "\n", "a\n", "b\n", "c\n", "\n", "ab\n", "ac\n", "\n", "a\n", "a\n", "a\n",
            "a\n", "\n", "b",
        );
        assert_eq!(part1(&text), 11);
    }
    #[test]
    fn part2_sample1() {
        let text = concat!(
            "abc\n", "\n", "a\n", "b\n", "c\n", "\n", "ab\n", "ac\n", "\n", "a\n", "a\n", "a\n",
            "a\n", "\n", "b",
        );
        assert_eq!(part2(&text), 6);
    }
}
