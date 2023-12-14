use aocd::*;

fn part_a(_input: String) -> usize {
    0
}

fn part_b(_input: String) -> usize {
    0
}

#[aocd({{year}}, {{day}})]
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
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(include_str!("test.txt").to_string());
        assert_eq!(result, 0);
    }
}
