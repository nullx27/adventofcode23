use std::fmt::Error;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct Game {
    pub id: u32,
    pub dice: Vec<Dice>,
}

impl Game {
    pub fn new(id: u32) -> Self {
        Game {
            id,
            dice: Vec::new(),
        }
    }

    pub fn total_red(&self) -> u32 {
        self.dice.iter().map(|x| x.red).sum::<u32>()
    }

    pub fn total_green(&self) -> u32 {
        self.dice.iter().map(|x| x.green).sum::<u32>()
    }

    pub fn total_blue(&self) -> u32 {
        self.dice.iter().map(|x| x.blue).sum::<u32>()
    }

    pub fn min_dice(&self) -> Dice {
        let red = self.dice.iter().max_by(|x, y| x.red.cmp(&y.red)).unwrap().red;
        let blue = self.dice.iter().max_by(|x, y| x.blue.cmp(&y.blue)).unwrap().blue;
        let green = self.dice.iter().max_by(|x, y| x.green.cmp(&y.green)).unwrap().green;

        Dice{
            red,
            green,
            blue,
        }
    }

    pub fn add_dice(&mut self, dice: Dice) {
        self.dice.push(dice);
    }
}

#[derive(Debug, Default)]
pub struct Dice {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(':').unwrap();
        let game_id = p1.split_once(' ').unwrap().1;

        let mut game: Game = Game::new(game_id.parse::<u32>().unwrap());

        p2.split(';').for_each(|x| {
            let mut d = Dice::default();
            for throw in x.trim().split(',').collect::<Vec<_>>() {
                let dice: Vec<_> = throw.trim().split(' ').collect();
                match dice[1] {
                    "red" => d.red = dice[0].parse::<u32>().unwrap(),
                    "green" => d.green = dice[0].parse::<u32>().unwrap(),
                    "blue" => d.blue = dice[0].parse::<u32>().unwrap(),
                    _ => (),
                };
            }
            game.add_dice(d);
        });

        Ok(game)
    }
}