use nom::{
    bytes::complete::{tag, take, take_till},
    character::complete::{alpha1, space1, u32},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Default)]
pub struct Game {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    // Implement a function to determine if a valid nuber of cubes were used within the game
    pub fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    pub fn from(input: &str) -> IResult<&str, Vec<Game>> {
        // Split of the start of the input as we don't need it
        let (input, _) = take_till(|c| c == ':')(input)?;
        let (input, _) = take(2_u8)(input)?;

        // Parse the rest as a list of games
        separated_list0(tag("; "), parse_game)(input)
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
