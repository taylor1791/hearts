use rand::{thread_rng, Rng};
use std::cmp::min;
use std::fmt::Debug;

#[derive(Clone, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn is_suit(&self, suit: &Suit) -> bool {
        &self.suit == suit
    }

    pub fn has_rank(&self, rank: &Rank) -> bool {
        &self.rank == rank
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("{:?}{:?}", self.rank, self.suit))
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Rank::Ace => f.write_str("A"),
            Rank::Two => f.write_str("2"),
            Rank::Three => f.write_str("3"),
            Rank::Four => f.write_str("4"),
            Rank::Five => f.write_str("5"),
            Rank::Six => f.write_str("6"),
            Rank::Seven => f.write_str("7"),
            Rank::Eight => f.write_str("8"),
            Rank::Nine => f.write_str("9"),
            Rank::Ten => f.write_str("10"),
            Rank::Jack => f.write_str("J"),
            Rank::Queen => f.write_str("Q"),
            Rank::King => f.write_str("K"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

impl Debug for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Suit::Clubs => f.write_str("♣"),
            Suit::Spades => f.write_str("♠"),
            Suit::Hearts => f.write_str("♥"),
            Suit::Diamonds => f.write_str("♦"),
        }
    }
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(52);

        for suit in [Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds].iter() {
            for rank in [
                Rank::Ace,
                Rank::Two,
                Rank::Three,
                Rank::Four,
                Rank::Five,
                Rank::Six,
                Rank::Seven,
                Rank::Eight,
                Rank::Nine,
                Rank::Ten,
                Rank::Jack,
                Rank::Queen,
                Rank::King,
            ]
            .iter()
            {
                cards.push(Card {
                    rank: rank.clone(),
                    suit: suit.clone(),
                });
            }
        }

        shuffle(&mut cards);

        Deck { cards }
    }

    pub fn deal(&mut self, hand_count: usize, n_cards: usize) -> Vec<Vec<Card>> {
        // Pass out the cards to deal with unequal hand sizes.
        let mut hands = Vec::with_capacity(hand_count.into());
        for _ in 0..hand_count {
            hands.push(Vec::with_capacity(self.cards.len() / n_cards + 1))
        }

        for i in 0..min(n_cards * n_cards, self.cards.len()) {
            match self.cards.pop() {
                Some(card) => hands[i % hand_count].push(card),
                None => unreachable!(""),
            }
        }

        hands
    }
}

fn shuffle<A>(cards: &mut Vec<A>) {
    let mut rng = thread_rng();

    for i in 0..cards.len() - 1 {
        let card_position = rng.gen_range(i, cards.len());
        cards.swap(i, card_position);
    }
}
