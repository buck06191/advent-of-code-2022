fn contained_in_range(pair_1: &Vec<u32>, pair_2: &Vec<u32>) -> bool {
    pair_1.iter().min() >= pair_2.iter().min() && pair_1.iter().max() <= pair_2.iter().max()
}

fn overlaps(pair_1: &Vec<u32>, pair_2: &Vec<u32>) -> bool {
    pair_1.iter().min() >= pair_2.iter().min() && pair_1.iter().min() <= pair_2.iter().max()
        || pair_1.iter().max() >= pair_2.iter().min() && pair_1.iter().max() <= pair_2.iter().max()
}

pub fn process_part1(file: &str) -> u32 {
    file.lines()
        .map(|line| {
            let pairs: Vec<Vec<u32>> = line
                .split(",")
                .map(|pair| {
                    pair.split("-")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect();

            if contained_in_range(&pairs[0], &pairs[1]) || contained_in_range(&pairs[1], &pairs[0])
            {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn process_part2(file: &str) -> u32 {
    file.lines()
        .map(|line| {
            let pairs: Vec<Vec<u32>> = line
                .split(",")
                .map(|pair| {
                    pair.split("-")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect();

            if overlaps(&pairs[0], &pairs[1]) || overlaps(&pairs[1], &pairs[0]) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]

mod tests {

    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1_works() {
        let part1 = process_part1(INPUT);
        assert_eq!(part1, 2);
    }

    #[test]
    #[ignore = "unimplemented"]
    fn part_2_works() {
        let part1 = process_part1(INPUT);
        assert_eq!(part1, 4);
    }
}
