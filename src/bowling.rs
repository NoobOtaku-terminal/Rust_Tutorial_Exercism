#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    current_frame: usize,
    roll_in_frame: usize,
    pins_left: u16,
    bonus_rolls_left: usize,
    game_complete: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: Vec::new(),
            current_frame: 0,
            roll_in_frame: 0,
            pins_left: 10,
            bonus_rolls_left: 0,
            game_complete: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // Check if game is already complete
        if self.game_complete {
            return Err(Error::GameComplete);
        }

        // Check if there are enough pins left
        if pins > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        // Record the roll
        self.rolls.push(pins);

        // Update game state
        if self.current_frame < 9 {
            // Frames 0-8
            if self.roll_in_frame == 0 {
                if pins == 10 {
                    // Strike - move to next frame
                    self.current_frame += 1;
                    self.pins_left = 10;
                } else {
                    // First roll in frame
                    self.roll_in_frame = 1;
                    self.pins_left = 10 - pins;
                }
            } else {
                // Second roll in frame - move to next frame
                self.current_frame += 1;
                self.roll_in_frame = 0;
                self.pins_left = 10;
            }
        } else if self.current_frame == 9 && self.bonus_rolls_left == 0 {
            // 10th frame, before bonus rolls
            if self.roll_in_frame == 0 {
                if pins == 10 {
                    // Strike in 10th frame
                    self.roll_in_frame = 1;
                    self.pins_left = 10;
                    self.bonus_rolls_left = 2;
                } else {
                    // Not a strike in 10th
                    self.roll_in_frame = 1;
                    self.pins_left = 10 - pins;
                }
            } else {
                // Second roll in 10th frame
                if pins + (10 - self.pins_left) == 10 {
                    // Spare in 10th frame
                    self.bonus_rolls_left = 1;
                    self.pins_left = 10;
                } else {
                    // Open 10th frame - game is done
                    self.game_complete = true;
                }
            }
        } else {
            // Bonus rolls after 10th frame
            self.bonus_rolls_left -= 1;
            
            if self.bonus_rolls_left == 1 {
                // After first bonus roll, set pins for second bonus
                if self.roll_in_frame == 1 && pins == 10 {
                    // Strike as first bonus after strike in 10th
                    self.pins_left = 10;
                } else {
                    // First bonus not a strike
                    self.pins_left = 10 - pins;
                }
            } else if self.bonus_rolls_left == 0 {
                // No more bonus rolls, game is complete
                self.game_complete = true;
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // Game must be complete and have enough rolls to score
        if !self.can_be_scored() {
            return None;
        }

        let mut score = 0;
        let mut roll_idx = 0;

        // Calculate score for each of the 10 frames
        for _ in 0..10 {
            if self.rolls[roll_idx] == 10 {
                // Strike
                score += 10 + self.rolls[roll_idx + 1] + self.rolls[roll_idx + 2];
                roll_idx += 1;
            } else {
                let frame_score = self.rolls[roll_idx] + self.rolls[roll_idx + 1];
                if frame_score == 10 {
                    // Spare
                    score += 10 + self.rolls[roll_idx + 2];
                } else {
                    // Open frame
                    score += frame_score;
                }
                roll_idx += 2;
            }
        }

        Some(score)
    }

    fn can_be_scored(&self) -> bool {
        // Game is complete and we have all necessary rolls
        if !self.game_complete && self.bonus_rolls_left > 0 {
            return false;
        }

        // Check that we have enough rolls for scoring
        let mut frames = 0;
        let mut roll_idx = 0;
        let mut rolls_needed = 0;

        while frames < 10 && roll_idx < self.rolls.len() {
            if self.rolls[roll_idx] == 10 {
                // Strike needs two more rolls to score
                rolls_needed = roll_idx + 3;
                roll_idx += 1;
            } else {
                if roll_idx + 1 >= self.rolls.len() {
                    return false; // Need second roll of frame
                }

                if self.rolls[roll_idx] + self.rolls[roll_idx + 1] == 10 {
                    // Spare needs one more roll to score
                    rolls_needed = roll_idx + 3;
                } else {
                    // Open frame just needs its two rolls
                    rolls_needed = roll_idx + 2;
                }
                roll_idx += 2;
            }
            frames += 1;
        }

        frames == 10 && self.rolls.len() >= rolls_needed
    }
}