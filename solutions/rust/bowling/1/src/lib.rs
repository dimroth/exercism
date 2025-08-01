#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    bonus_rolls: Vec<u16>,
    is_game_over: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::with_capacity(21),
            bonus_rolls: Vec::with_capacity(2),
            is_game_over: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_over {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_tenth_frame() {
            return self.roll_tenth_frame(pins);
        }
        
        let frame_start_index = self.current_frame_start_index();
        let frame_rolls = &self.rolls[frame_start_index..];

        if !frame_rolls.is_empty() && frame_rolls[0] < 10 && frame_rolls[0] + pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        
        self.rolls.push(pins);

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_over {
            return None;
        }
        
        let mut total_score = 0;
        let mut roll_index = 0;
        let all_rolls = [self.rolls.as_slice(), self.bonus_rolls.as_slice()].concat();
        
        for _ in 0..10 {
            if roll_index >= self.rolls.len() {
                break;
            }

            if self.rolls[roll_index] == 10 { // Strike
                total_score += 10 + all_rolls.get(roll_index + 1).unwrap_or(&0) + all_rolls.get(roll_index + 2).unwrap_or(&0);
                roll_index += 1;
            } else if roll_index + 1 < self.rolls.len() && self.rolls[roll_index] + self.rolls[roll_index + 1] == 10 { // Spare
                total_score += 10 + all_rolls.get(roll_index + 2).unwrap_or(&0);
                roll_index += 2;
            } else { // Open frame
                total_score += self.rolls.get(roll_index).unwrap_or(&0) + self.rolls.get(roll_index + 1).unwrap_or(&0);
                roll_index += 2;
            }
        }
        Some(total_score)
    }

    fn is_tenth_frame(&self) -> bool {
        let mut frames = 0;
        let mut i = 0;
        while i < self.rolls.len() {
            frames += 1;
            if frames == 10 {
                return true;
            }
            if self.rolls[i] == 10 { // Strike
                i += 1;
            } else {
                if i + 1 >= self.rolls.len() {
                    return false;
                }
                i += 2;
            }
        }
        false
    }

    fn roll_tenth_frame(&mut self, pins: u16) -> Result<(), Error> {
        if !self.is_bonus_time() {
            let frame_start_index = self.current_frame_start_index();
            let frame_rolls = &self.rolls[frame_start_index..];
            if !frame_rolls.is_empty() && frame_rolls[0] < 10 && frame_rolls[0] + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            self.rolls.push(pins);
            
            let frame_rolls_after = &self.rolls[frame_start_index..];
            if !self.is_bonus_time() && frame_rolls_after.len() == 2 {
                self.is_game_over = true;
            }
        } else {
            let tenth_frame_rolls = &self.rolls[self.current_frame_start_index()..];

            if self.bonus_rolls.is_empty() {
                if tenth_frame_rolls.iter().sum::<u16>() >= 10 && tenth_frame_rolls[0] != 10 && pins > 10 { // spare bonus
                     return Err(Error::NotEnoughPinsLeft);
                }
            } else if !self.bonus_rolls.is_empty() { // Second bonus roll
                if self.bonus_rolls[0] != 10 && self.bonus_rolls[0] + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
            self.bonus_rolls.push(pins);
            if self.is_bonus_time_complete() {
                self.is_game_over = true;
            }
        }
        Ok(())
    }

    fn is_bonus_time(&self) -> bool {
        if !self.is_tenth_frame() {
            return false;
        }
        let frame_start_index = self.current_frame_start_index();
        let frame_rolls = &self.rolls[frame_start_index..];
        let frame_sum: u16 = frame_rolls.iter().sum();
        
        (frame_rolls.len() == 1 && frame_sum == 10) || (frame_rolls.len() == 2 && frame_sum >= 10)
    }

    fn is_bonus_time_complete(&self) -> bool {
        if !self.is_bonus_time() { return false; }
        
        let frame_start_index = self.current_frame_start_index();
        let frame_rolls = &self.rolls[frame_start_index..];

        if frame_rolls[0] == 10 { // strike in 10th
            self.bonus_rolls.len() == 2
        } else { // spare in 10th
            self.bonus_rolls.len() == 1
        }
    }
    
    fn current_frame_start_index(&self) -> usize {
        let mut i = 0;
        let mut frames = 0;
        while i < self.rolls.len() {
            if frames == 9 {
                break;
            }
            frames += 1;
            if self.rolls[i] == 10 {
                i += 1;
            } else {
                if i + 1 >= self.rolls.len() {
                    break;
                }
                i += 2;
            }
        }
        i
    }
}
