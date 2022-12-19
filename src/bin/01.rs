pub fn part_one(input: &str) -> Option<u32> {
    let mut max = 0;
    let mut curr_total = 0;
    input.split("\n").for_each(|line| {
        if line == "" {
            if curr_total > max {
                max = curr_total;
            }
            curr_total = 0;
            return;
        }

        curr_total += line.parse::<u32>().unwrap();
    });
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totals = vec![];
    let mut curr_total = 0;
    input.split("\n").for_each(|line| {
        if line == "" {
            totals.push(curr_total);
            curr_total = 0;
            return;
        }

        curr_total += line.parse::<u32>().unwrap();
    });

    totals.sort();
    totals.reverse();
    Some(totals[0] + totals[1] + totals[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
