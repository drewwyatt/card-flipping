use std::str::FromStr;

#[derive(Debug)]
pub enum Card {
    FaceUp(usize),
    FaceDown(usize),
    Removed(usize),
}

#[derive(Debug)]
pub struct Game {
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub enum GameError {
    ParseError(char),
    InvalidRemove(Card),
    UnknownError,
}

impl Game {
    pub fn remove(&mut self, card: Card) {
        match card {
            Card::FaceUp(idx) => {
                self.flip(self.cards.get(idx - 1));
                self.flip(self.cards.get(idx + 1));
                self.cards[idx] = Card::Removed(idx);
            },
            _ => {
                panic!(GameError::InvalidRemove(card));
            }
        }
    }

    fn flip(&mut self, card: Option<&Card>) {

    }
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = vec![];
        let mut err: Option<Self::Err> = None;
        for (idx, char) in s.chars().enumerate() {
            if char == '0' {
                cards.push(Card::FaceDown(idx));
            } else if char == '1' {
                cards.push(Card::FaceUp(idx));
            } else {
                err = Some(GameError::ParseError(char));
            }
        }

        match err {
          Some(error) => Err(error),
          None => Ok(Game{ cards })
        }
    }
}
