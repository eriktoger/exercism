#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Roll {
    first: Option<u16>,
    second: Option<u16>,
    third: Option<u16>,
    pins_left: u16,
}

pub struct BowlingGame {
    rolls: Vec<Roll>,
    current_roll_index: usize,
    score: Option<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: vec![],
            current_roll_index: 0,
            score: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.score.is_some() {
            return Err(Error::GameComplete);
        }

        if self.current_roll_index == self.rolls.len() {
            self.rolls.push(Roll {
                first: None,
                second: None,
                third: None,
                pins_left: 10,
            })
        }

        let current_roll = &mut self.rolls[self.current_roll_index];

        if current_roll.pins_left < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        if current_roll.first.is_none() {
            current_roll.pins_left -= pins;
            current_roll.first = Some(pins);

            if pins == 10 && self.current_roll_index != 9 {
                self.current_roll_index += 1;
            } else if pins == 10 {
                current_roll.pins_left = 10;
            }
        } else if current_roll.second.is_none() {
            current_roll.pins_left -= pins;
            current_roll.second = Some(pins);

            if self.current_roll_index != 9 {
                self.current_roll_index += 1;
            } else if current_roll.pins_left != 0 && current_roll.first.unwrap() != 10 {
                self.current_roll_index += 1;
            } else if current_roll.pins_left == 0 {
                current_roll.pins_left = 10;
            }
        } else {
            current_roll.third = Some(pins);
            self.current_roll_index += 1;
        }

        if self.current_roll_index == 10 {
            self.calc_score();
        }

        return Ok(());
    }

    fn calc_score(&mut self) {
        let mut score = 0;
        for (idx, roll) in self.rolls.iter().enumerate() {
            let first = roll.first.unwrap_or_default();
            let second = roll.second.unwrap_or_default();
            let third = roll.third.unwrap_or_default();
            score += first + second + third;

            if first + second == 10 && idx < 9 {
                let next_roll = &self.rolls[idx + 1];
                score += next_roll.first.unwrap_or_default();
            }

            if first == 10 && idx < 9 {
                let next_roll = &self.rolls[idx + 1];
                score += next_roll.second.unwrap_or_default();
            }

            if first == 10 && idx < 8 {
                let second_next_roll = &self.rolls[idx + 2];
                score += second_next_roll.first.unwrap_or_default();
            }
        }
        self.score = Some(score);
    }

    pub fn score(&self) -> Option<u16> {
        self.score
    }
}
