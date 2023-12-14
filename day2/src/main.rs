use aocd::*;

mod game;
use game::Game;

fn part_a(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, games) = Game::from(line).unwrap();

            // Parse the input and return a boolean defined by whether all the rounds were valid
            games.iter().all(|game| game.is_valid())
        })
        // Use enumerate to get the game number
        .enumerate()
        // Map to preserve index
        .map(|(i, e)| if e { i as u32 + 1 } else { 0 })
        .sum::<u32>()
}

fn part_b(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, games) = Game::from(line).unwrap();

            // Fold over the rounds to find the max values for red, green and blue
            let (max_red, max_green, max_blue) = games.iter().fold((0, 0, 0), |(r, g, b), game| {
                (r.max(game.red), g.max(game.green), b.max(game.blue))
            });

            max_red * max_green * max_blue
        })
        .sum::<u32>()
}

#[aocd(2023, 2)]
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
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(include_str!("test.txt").to_string());
        assert_eq!(result, 2286);
    }
}
