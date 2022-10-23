pub mod cards;
pub mod extras;
pub mod utils;

use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use std::{fmt, ops::Deref};

#[derive(Clone)]
pub struct CardProxy<'a> {
    card: &'a cards::Card,
    flipped: bool,
}

impl<'a> CardProxy<'a> {
    fn new(card: &'a cards::Card) -> Self {
        Self {
            card,
            flipped: true,
        }
    }
}

#[derive(Debug)]
pub struct InvalidBoardSizeError;

pub struct Board<'a>(Vec<Vec<Option<CardProxy<'a>>>>);

impl<'a> Board<'a> {
    pub fn new(deck: &'a cards::Deck, size: u8) -> Result<Board<'a>, InvalidBoardSizeError> {
        if size > 26 || size <= 0 {
            return Err(InvalidBoardSizeError);
        }

        let [x, _] = utils::squarest_rect_with_even_area(size as u32);

        let mut taken_cards = deck
            .iter()
            .take(size as usize * 2)
            .map(|c| Some(CardProxy::new(c)))
            .collect_vec();
        taken_cards.shuffle(&mut thread_rng());
        let vec = taken_cards
            .chunks(x as usize)
            .map(|c| c.to_owned())
            .collect_vec();
        Ok(Board(vec))
    }
}

impl<'a> Deref for Board<'a> {
    type Target = Vec<Vec<Option<CardProxy<'a>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self
            .iter()
            .map(|c| {
                c.iter()
                    .map(|c| match c {
                        Some(c) => {
                            if c.flipped {
                                "---".to_owned()
                                // format!(
                                //     "{}---{}",
                                //     color::Fg(color::LightBlue),
                                //     color::Fg(color::Reset)
                                // )
                            } else {
                                c.card.to_string()
                            }
                        }
                        _ => "   ".to_owned(),
                    })
                    .join(" ")
            })
            .collect_vec();
        write!(f, "{}", formatted.join("\n"))
    }
}

impl Board<'_> {
    pub fn flip(&mut self, x: u8, y: u8) {
        let flipped = &mut self.0[x as usize][y as usize].as_mut().unwrap().flipped;
        *flipped = !*flipped;
    }
}

pub struct Player<'a> {
    pub name: String,
    pub paired_cards: Vec<&'a cards::Card>,
}

impl Player<'_> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            paired_cards: vec![],
        }
    }
}
