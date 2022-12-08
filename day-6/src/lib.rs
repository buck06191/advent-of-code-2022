use std::collections::HashSet;

fn find_stop_point(message: &str, unique_length: usize) -> usize {
    let mut stop_point: usize = 0;

    let interval = unique_length - 1;

    message
        .chars()
        .enumerate()
        .skip(interval)
        .try_for_each(|(idx, _)| {
            stop_point = idx;

            let sub = &message[idx - interval..=idx];
            let is_unique = sub.chars().collect::<HashSet<char>>().len() == unique_length;

            if is_unique {
                None
            } else {
                Some(())
            }
        });

    stop_point + 1
}
pub fn process_part1(file: &str) -> usize {
    find_stop_point(file, 4)
}

pub fn process_part2(file: &str) -> usize {
    find_stop_point(file, 14)
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn part_1_works() {
        assert_eq!(process_part1("mjqjpqmgbldphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_2_works() {
        // assert_eq!(process_part2("mjqjpqmgbldphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
