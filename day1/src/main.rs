use aocd::*;

fn part_a(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            // Initialise a result variable with value zero
            let mut result = 0;

            // Work forwards through the string and add the value of the first number to our result
            for c in line.chars() {
                if c.is_numeric() {
                    result += c.to_digit(10).unwrap();
                    break;
                }
            }

            // Work backwards through the string. On discovering the final digit, multiply the current result by 10 to shift the value left. Finally, add the value of the last number to our result
            for c in line.chars().rev() {
                if c.is_numeric() {
                    result *= 10;
                    result += c.to_digit(10).unwrap();
                    break;
                }
            }

            // return the result
            result
        })
        .sum()
}

fn part_b(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            //Create a blank string
            let mut result = 0;

            for (i, c) in line.chars().enumerate() {
                // If its a digit, break early.
                if c.is_numeric() {
                    result += c.to_digit(10).unwrap();
                    break;
                }

                //Match the current slice to find the first number written as a word
                let val: Option<u32> = match line[..i + 1] {
                    ref x if x.contains("one") => Some(1),
                    ref x if x.contains("two") => Some(2),
                    ref x if x.contains("three") => Some(3),
                    ref x if x.contains("four") => Some(4),
                    ref x if x.contains("five") => Some(5),
                    ref x if x.contains("six") => Some(6),
                    ref x if x.contains("seven") => Some(7),
                    ref x if x.contains("eight") => Some(8),
                    ref x if x.contains("nine") => Some(9),
                    _ => None,
                };

                // If there is one, add it to the result and return
                if let Some(val) = val {
                    result += val;
                    break;
                }
            }

            // Loop through in reverse
            for (i, c) in line.chars().rev().enumerate() {
                // If its a digit, break early.
                if c.is_numeric() {
                    result *= 10;
                    result += c.to_digit(10).unwrap();
                    break;
                }

                // Match the string in reverse
                let val: Option<u32> = match line[line.len() - i - 1..] {
                    ref x if x.contains("one") => Some(1),
                    ref x if x.contains("two") => Some(2),
                    ref x if x.contains("three") => Some(3),
                    ref x if x.contains("four") => Some(4),
                    ref x if x.contains("five") => Some(5),
                    ref x if x.contains("six") => Some(6),
                    ref x if x.contains("seven") => Some(7),
                    ref x if x.contains("eight") => Some(8),
                    ref x if x.contains("nine") => Some(9),
                    _ => None,
                };

                // Push to the result
                if let Some(val) = val {
                    result *= 10;
                    result += val;
                    break;
                }
            }

            result
        })
        .sum::<u32>()
}

#[aocd(2023, 1)]
fn main() {
    submit!(1, part_a(input!()));
    submit!(2, part_b(input!()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let result = part_a(include_str!("test_a.txt").to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(include_str!("test_b.txt").to_string());
        assert_eq!(result, 0);
    }
}
