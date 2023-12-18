use aocd::*;

fn parse_line(input: &str) -> u32 {
    // Can safely unwrap as the Scratchcard format will always have a : & |
    let (_, input) = input.split_once(':').unwrap();
    let (left, right) = input.split_once('|').unwrap();

    let winning_numbers: Vec<u32> = left
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    // Fold over the right hand values and calculate how many winning numbers there are
    right
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .fold(0_u32, |acc, num| {
            if winning_numbers.contains(&num) {
                return acc + 1;
            }
            acc
        })
}

fn part_a(input: String) -> u32 {
    // For each line calculate the score by working out 2 ^ (no_wins - 1), return sum
    input.lines().map(parse_line).fold(0, |acc, count| {
        if count != 0 {
            return acc + 2_u32.pow(count - 1);
        }
        acc
    })
}

fn part_b(input: String) -> u32 {
    let mut res = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
        // For each score, increment the result vec by the number of scorecards at this index
        for x in 1..=parse_line(line) {
            res[index + x as usize] += res[index];
        }
    }

    // Sum the resulting number of scorecards
    res.iter().sum::<u32>()
}

#[aocd(2023, 4)]
fn main() {
    submit!(1, part_a(input!()));
    submit!(2, part_b(input!()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let result = part_a(include_str!("test.txt").to_string());
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(include_str!("test.txt").to_string());
        assert_eq!(result, 30);
    }
}
