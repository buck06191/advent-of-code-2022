use std::{collections::HashSet, ops::Div};

fn convert_to_priority(c: &char) -> u32 {
    if c.is_lowercase() {
        *c as u32 - 96
    } else {
        *c as u32 - 38
    }
}

pub fn process_part1(file: &str) -> u32 {
    file.lines()
        .map(|line| {
            let mid = line.chars().count().div(2);
            let (left, right) = line.split_at(mid);

            let left_set: HashSet<char> = left.chars().map(|c| c).collect();
            let right_set: HashSet<char> = right.chars().map(|c| c).collect();

            let value: Vec<&char> = left_set.intersection(&right_set).collect();

            convert_to_priority(value[0])
        })
        .sum()
}

pub fn process_part2(file: &str) -> u32 {
    let lines: Vec<&str> = file.lines().collect();

    let priorities = lines.chunks(3).map(|chunk| {
        let sets: Vec<HashSet<char>> = chunk.iter().map(|items| items.chars().collect()).collect();

        let intersection_1: HashSet<char> = sets[0].intersection(&sets[1]).cloned().collect();

        let intersection: Vec<&char> = intersection_1.intersection(&sets[2]).collect();

        convert_to_priority(intersection[0])
    });

    priorities.sum()
}

#[cfg(test)]

mod tests {

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part_1_works() {
        let part1 = process_part1(INPUT);
        assert_eq!(part1, 157);
    }

    #[test]
    fn part_2_works() {
        let part2 = process_part2(INPUT);
    }
}
