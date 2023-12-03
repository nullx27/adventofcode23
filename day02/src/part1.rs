use crate::game::Game;

pub fn process(input: &str) -> anyhow::Result<String> {
    let games: Vec<Game> = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect();
    let mut result = 0u32;

    for game in games {
        let mut possible = true;
        for dice in game.dice {
            if dice.red > 12 || dice.green > 13 || dice.blue > 14 {
                possible = false;
            }
        }

        if possible {
            result += game.id;
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!("8", process(input)?);

        Ok(())
    }
}