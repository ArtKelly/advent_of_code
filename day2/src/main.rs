use aocd::*;
use nom::{
    bytes::complete::{tag, take, take_till},
    character::complete::{alpha1, space1, u32},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Default)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    // Implement a function to determine if a valid nuber of cubes were used within the game
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    // Separate and parse all the different rounds within the game
    let (input, data) = separated_list0(tag(", "), separated_pair(u32, space1, alpha1))(input)?;

    // Todo: Maybe use map_res? this also works though
    let mut game = Game::default();
    for (count, colour) in data {
        match colour {
            "red" => game.red += count,
            "green" => game.green += count,
            "blue" => game.blue += count,
            _ => (),
        }
    }
    Ok((input, game))
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Game>> {
    // Split of the start of the input as we don't need it
    let (input, _) = take_till(|c| c == ':')(input)?;
    let (input, _) = take(2_u8)(input)?;

    // Parse the rest as a list of games
    separated_list0(tag("; "), parse_game)(input)
}

fn part_a(input: String) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, data) = parse_lines(line).unwrap();
            // Parse the input and return a boolean defined by whether all the rounds were valid
            data.iter().all(|game| game.is_valid())
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
            let (_, data) = parse_lines(line).unwrap();
            // Fold over the rounds to find the max values for red, green and blue
            let (max_red, max_green, max_blue) = data.iter().fold((0, 0, 0), |(r, g, b), game| {
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
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_b() {
        let result = part_b(include_str!("test.txt").to_string());
        assert_eq!(result, 0);
    }
}
