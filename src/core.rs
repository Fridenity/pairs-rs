use crate::{cards, utils};
use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use std::{fmt, ops::Deref};
use termion::color;

#[derive(Clone)]
pub struct CardProxy {
    card: cards::Card,
    flipped: bool,
}

impl CardProxy {
    fn new(card: cards::Card) -> Self {
        CardProxy {
            card,
            flipped: true,
        }
    }
}

#[derive(Debug)]
pub struct InvalidBoardSizeError;

pub struct Board(Vec<Vec<CardProxy>>);

impl Deref for Board {
    type Target = Vec<Vec<CardProxy>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = self
            .iter()
            .map(|c| {
                c.iter()
                    .map(|c| {
                        if c.flipped {
                            format!(
                                "{}---{}",
                                color::Fg(color::LightBlue),
                                color::Fg(color::Reset)
                            )
                        } else {
                            c.card.to_string()
                        }
                    })
                    .join(" ")
            })
            .collect_vec();
        write!(f, "{}", formatted.join("\n"))
    }
}

impl Board {
    pub fn flip(&mut self, x: u8, y: u8) {
        let flipped = &mut self.0[x as usize][y as usize].flipped;
        *flipped = !*flipped;
    }
}

pub fn generate_board(size: u8) -> Result<Board, InvalidBoardSizeError> {
    if size > 26 || size <= 0 {
        return Err(InvalidBoardSizeError);
    }

    let [x, _] = utils::squarest_rect_with_even_area(size as u32);
    let deck = cards::Deck::paired_shuffled();

    let mut taken_cards: Vec<CardProxy> = deck
        .iter()
        .take(size as usize * 2)
        .map(|c| CardProxy::new(c.to_owned()))
        .collect();
    taken_cards.shuffle(&mut thread_rng());
    let vec = taken_cards
        .chunks(x as usize)
        .map(|c| c.to_vec())
        .collect_vec();
    Ok(Board(vec))
}
