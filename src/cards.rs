use itertools::Itertools;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    thread_rng, Rng,
};
use std::{fmt, ops::Deref};
use termion::color;

#[derive(Debug)]
pub enum CardConvertionError {
    IntoRankError,
    IntoSuitError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl TryFrom<u8> for Rank {
    type Error = CardConvertionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 13 {
            return Err(CardConvertionError::IntoRankError);
        }
        Ok(match value {
            0 => Rank::ACE,
            1 => Rank::TWO,
            2 => Rank::THREE,
            3 => Rank::FOUR,
            4 => Rank::FIVE,
            5 => Rank::SIX,
            6 => Rank::SEVEN,
            7 => Rank::EIGHT,
            8 => Rank::NINE,
            9 => Rank::TEN,
            10 => Rank::JACK,
            11 => Rank::QUEEN,
            12 => Rank::KING,
            _ => unreachable!(),
        })
    }
}

impl Distribution<Rank> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        rng.gen_range(0..13).try_into().unwrap()
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rank::ACE => "A".to_owned(),
            Rank::JACK => "J".to_owned(),
            Rank::QUEEN => "Q".to_owned(),
            Rank::KING => "K".to_owned(),
            _ => (*self as u8 + 1).to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    CLUBS,
    DIAMONDS,
    HEARTS,
    SPADES,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum SuitColor {
    RED,
    BLACK,
}

impl TryFrom<u8> for Suit {
    type Error = CardConvertionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= 4 {
            return Err(CardConvertionError::IntoSuitError);
        }
        Ok(match value {
            0 => Suit::CLUBS,
            1 => Suit::DIAMONDS,
            2 => Suit::HEARTS,
            3 => Suit::SPADES,
            _ => unreachable!(),
        })
    }
}

impl Suit {
    fn color(&self) -> SuitColor {
        match self {
            &Suit::CLUBS | &Suit::SPADES => SuitColor::BLACK,
            _ => SuitColor::RED,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suits = ['♧', '♢', '♡', '♤'];
        write!(f, "{}", suits[*self as usize])
    }
}

impl Distribution<Suit> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        rng.gen_range(0..4).try_into().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card { suit, rank }
    }

    fn color(&self) -> SuitColor {
        self.suit.color()
    }
}

impl TryFrom<[u8; 2]> for Card {
    type Error = CardConvertionError;

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        let [suit, rank] = value;
        let suit = suit.try_into()?;
        let rank = rank.try_into()?;
        Ok(Card::new(suit, rank))
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let colorize_fn = |s: &str| match self.color() {
            SuitColor::RED => format!(
                "{}{:>3}{}",
                color::Fg(color::Red),
                s,
                color::Fg(color::Reset)
            ),
            _ => format!(
                "{}{}{}",
                color::Fg(color::Black),
                s,
                color::Fg(color::Reset)
            ),
        };

        let card_fmt = format!("{:>2}{}", self.rank.to_string(), self.suit);
        write!(f, "{}", colorize_fn(&card_fmt))
    }
}

impl Distribution<Card> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _: &mut R) -> Card {
        let suit: Suit = rand::random();
        let rank: Rank = rand::random();
        Card::new(suit, rank)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Deck([Card; 52]);

impl Deref for Deck {
    type Target = [Card; 52];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(dead_code)] // TODO: deal with this later
impl Deck {
    fn all_cards() -> [Card; 52] {
        (0..4)
            .cartesian_product(0..13)
            .map(|(r, s)| Card::try_from([r, s]).unwrap())
            .collect_vec()
            .try_into()
            .unwrap()
    }

    fn all_cards_paired() -> [Card; 52] {
        let mut cards = Deck::all_cards();
        cards.sort_by_key(|c| c.color());
        cards.sort_by_key(|c| c.rank);
        cards
    }

    fn all_cards_shuffled() -> [Card; 52] {
        let mut cards = Deck::all_cards();
        cards.shuffle(&mut thread_rng());
        cards
    }

    fn all_cards_paired_shuffled() -> [Card; 52] {
        let cards = Deck::all_cards_paired();
        let mut chunked = cards.chunks_exact(2).collect_vec();
        chunked.shuffle(&mut thread_rng());
        chunked
            .into_iter()
            .flatten()
            .map(|c| c.to_owned())
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn new() -> Deck {
        Deck(Deck::all_cards())
    }

    pub fn paired() -> Deck {
        Deck(Deck::all_cards_paired())
    }

    pub fn shuffled() -> Deck {
        Deck(Deck::all_cards_shuffled())
    }

    pub fn paired_shuffled() -> Deck {
        Deck(Deck::all_cards_paired_shuffled())
    }
}
