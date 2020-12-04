type Map = Vec<Vec<bool>>;

fn count_tree(map: &Map, right: usize, down: usize) -> usize {
    let height = map.len();
    let width = map[0].len();

    let mut tree_count = 0;
    let mut x = 0;
    let mut y = 0;
    while x < height {
        let index_y = y % width; // if y > width, we need to repeat the map
        if map[x][index_y] {
            tree_count = tree_count + 1;
        }
        x = x + down;
        y = y + right;
    }
    tree_count
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut row: Vec<bool> = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    _ => (),
                }
            }
            row
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Map) -> usize {
    count_tree(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &Map) -> usize {
    let a = count_tree(input, 1, 1);
    let b = count_tree(input, 3, 1);
    let c = count_tree(input, 5, 1);
    let d = count_tree(input, 7, 1);
    let e = count_tree(input, 1, 2);
    a * b * c * d * e
}

#[cfg(test)]
mod tests {
    use super::{input_generator, part1, part2};

    #[test]
    fn part1_sample1() {
        let text = concat!(
            "..##.......\n",
            "#...#...#..\n",
            ".#....#..#.\n",
            "..#.#...#.#\n",
            ".#...##..#.\n",
            "..#.##.....\n",
            ".#.#.#....#\n",
            ".#........#\n",
            "#.##...#...\n",
            "#...##....#\n",
            ".#..#...#.#",
        );
        let input = input_generator(text);
        assert_eq!(part1(&input), 7);
    }
    #[test]
    fn part2_sample1() {
        let text = concat!(
            "..##.......\n",
            "#...#...#..\n",
            ".#....#..#.\n",
            "..#.#...#.#\n",
            ".#...##..#.\n",
            "..#.##.....\n",
            ".#.#.#....#\n",
            ".#........#\n",
            "#.##...#...\n",
            "#...##....#\n",
            ".#..#...#.#",
        );
        let input = input_generator(text);
        assert_eq!(part2(&input), 336);
    }
}
