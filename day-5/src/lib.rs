use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Pile {
    string_position: usize,
    crates: Vec<char>,
}

fn convert_cargo_to_hash_map(cargo: &str) -> HashMap<usize, Pile> {
    let mut reversed_lines = cargo.lines().rev();
    let last_line = reversed_lines.next().unwrap();

    lazy_static! {
        static ref PILE_RE: Regex = Regex::new(r"(\d)").unwrap();
    }

    lazy_static! {
        static ref CRATE_RE: Regex = Regex::new(r"([A-Z])").unwrap();
    }

    let mut piles: HashMap<usize, Pile> = PILE_RE
        .find_iter(last_line)
        .map(|mat| mat.as_str())
        .map(|label| {
            let pile = Pile {
                crates: vec![],
                string_position: last_line.find(label).unwrap(),
            };
            (label.parse::<usize>().unwrap(), pile)
        })
        .collect();

    for line in reversed_lines {
        CRATE_RE
            .find_iter(line)
            .map(|mat| mat.as_str())
            .for_each(|c| {
                let pos = line.find(c).unwrap();

                let (_, pile) = piles
                    .iter_mut()
                    .find(|(_, pile)| pile.string_position == pos)
                    .unwrap();

                // let new_pile = (
                //     label,
                //     (*pile).clone().crates.push(c.chars().next().unwrap()),
                // );

                pile.crates.push(c.chars().next().unwrap());
            });
    }
    piles
}

fn process_instructions(instructions: &str, mut piles: HashMap<usize, Pile>) -> String {
    instructions.lines().for_each(|line| {
        let splits: Vec<usize> = line
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        let (number, from, to) = (splits[0], splits[1], splits[2]);

        for _i in [0..number] {
            let from_p = piles.get(&from).unwrap();
            let off = from_p.crates.pop().unwrap();

            piles.get(&to).unwrap().crates.push(off);
        }
    });

    let mut sorted: Vec<_> = piles.into_iter().collect();
    sorted.sort_by_key(|a| a.0);

    sorted
        .iter()
        .map(|pile| *(pile.1.crates.last().unwrap()))
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>()
}

pub fn process_part1(file: &str) -> String {
    let splits: Vec<&str> = file.split("\n\n").collect();

    let cargo = splits[0];

    let instructions = splits[1];

    let piles = convert_cargo_to_hash_map(cargo);

    process_instructions(instructions, piles)
}

pub fn process_part2(_file: &str) -> &str {
    "part_2"
}

#[cfg(test)]

mod tests {

    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_1_works() {
        let part1 = process_part1(INPUT);
        assert_eq!(part1, "CMZ");
    }

    #[test]
    #[ignore = "unimplemented"]
    fn part_2_works() {
        let part2 = process_part2(INPUT);
        assert_eq!(part2, "");
    }
}
