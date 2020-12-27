use super::cards::{Card, Deck, Rank, Suit};

pub type PlayerId = usize;

#[derive(Debug)]
pub struct Round {
    first_trick: bool,
    hearts_broken: bool,
    lead: PlayerId,
    in_play: Vec<PlayedCard>,
    players: [Player; 4],
}

impl Round {
    pub fn new() -> Round {
        let mut deck = Deck::new();
        let hands = deck.deal(4, 13);

        Round::with_hands(hands)
    }

    pub fn with_hands(mut hands: Vec<Vec<Card>>) -> Round {
        let mut round = Round {
            first_trick: true,
            hearts_broken: false,
            lead: 255,
            in_play: vec![],
            players: [
                Player {
                    player_id: 0,
                    hand: std::mem::replace(&mut hands[0], vec![]),
                    tricks: vec![],
                },
                Player {
                    player_id: 1,
                    hand: std::mem::replace(&mut hands[1], vec![]),
                    tricks: vec![],
                },
                Player {
                    player_id: 2,
                    hand: std::mem::replace(&mut hands[2], vec![]),
                    tricks: vec![],
                },
                Player {
                    player_id: 3,
                    hand: std::mem::replace(&mut hands[3], vec![]),
                    tricks: vec![],
                },
            ],
        };

        round.lead = round
            .find_in_hand(&Card {
                suit: Suit::Clubs,
                rank: Rank::Two,
            })
            .expect("No hand contains the two of clubs.");

        round
    }

    pub fn play(&mut self, player_id: PlayerId, card: Card) -> Result<(), String> {
        if self.get_turn() != Some(player_id) {
            return Err(format!("Player {} played out of turn.", player_id));
        }

        let choices = self.get_valid_plays(player_id);
        if !choices.contains(&card) {
            return Err(format!(
                "Player {} violated rules by playing {:?}.",
                player_id, card
            ));
        }

        self.in_play.push(PlayedCard { player_id, card });

        if self.in_play.len() == 4 {
            let winner = trick_winner(&self.in_play);
            self.lead = winner;
            self.first_trick = false;

            let mut trick_cards = vec![];
            std::mem::swap(&mut self.in_play, &mut trick_cards);

            for PlayedCard { player_id, card } in &trick_cards {
                if card.is_suit(&Suit::Hearts) {
                    self.hearts_broken = true;
                }

                let hand = &mut self.players[*player_id].hand;
                if let Some(position) = hand.iter().position(|c| c == card) {
                    hand.remove(position);
                }
            }

            self.players[winner].tricks.push(Trick {
                lead: self.lead,
                played: trick_cards,
            });
        }

        Ok(())
    }

    pub fn get_turn(&self) -> Option<PlayerId> {
        if self.in_play.len() == 0 {
            if self.players[0].hand.len() == 0 {
                return None;
            }

            return Some(self.lead);
        }

        Some((self.lead + self.in_play.len()) % 4)
    }

    pub fn get_valid_plays(&self, player_id: PlayerId) -> Vec<Card> {
        let hand = &self.players[player_id].hand;

        // Not leading
        if self.in_play.len() > 0 {
            // First round heart restriction
            let queen_spades = Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
            };
            let point_restriction = limit_hand(&hand, |card| {
                !(self.first_trick && (card.is_suit(&Suit::Hearts) || card == &&queen_spades))
            });

            // Follow suit
            let allowed_cards = limit_hand(&point_restriction, |card| {
                card.is_suit(&self.in_play[0].card.suit)
            });

            return allowed_cards;
        }

        // Leading
        let two_clubs = Card {
            suit: Suit::Clubs,
            rank: Rank::Two,
        };

        let mut valid_leads = Vec::with_capacity(hand.len());

        for card in hand {
            if card == &two_clubs {
                return vec![two_clubs];
            }

            if !card.is_suit(&Suit::Hearts) || self.hearts_broken {
                valid_leads.push(card.clone());
            }
        }

        return valid_leads;
    }

    pub fn get_player_cards<'a>(self: &'a Self, player: PlayerId) -> &'a Vec<Card> {
        &self.players[player].hand
    }

    pub fn score(&self) -> [i8; 4] {
        let mut scores = [0; 4];

        for (player_id, player) in self.players.iter().enumerate() {
            scores[player_id] += score_hand(&player.tricks);
        }

        return scores;
    }

    fn find_in_hand(&self, card: &Card) -> Option<PlayerId> {
        for player in self.players.iter() {
            for c in player.hand.iter() {
                if c == card {
                    return Some(player.player_id);
                }
            }
        }

        None
    }
}

fn trick_winner(played_cards: &Vec<PlayedCard>) -> PlayerId {
    let lead_card = &played_cards[0].card;

    let mut followed_suit = played_cards
        .iter()
        .filter(|played| played.card.is_suit(&lead_card.suit))
        .collect::<Vec<&PlayedCard>>();

    followed_suit.sort_unstable_by_key(|played_card| &played_card.card.rank);

    followed_suit[0].player_id
}

fn limit_hand(hand: &Vec<Card>, filter: impl FnMut(&&Card) -> bool) -> Vec<Card> {
    let cards = hand
        .iter()
        .filter(filter)
        .map(|card| (*card).clone())
        .collect::<Vec<Card>>();

    if cards.len() > 0 {
        return cards;
    }

    hand.clone()
}

fn score_hand(tricks: &Vec<Trick>) -> i8 {
    let mut score = 0;

    for trick in tricks {
        for played_card in &trick.played {
            score += point_value(&played_card.card);
        }
    }

    if score == 26 {
        return -26;
    }

    score
}

fn point_value(card: &Card) -> i8 {
    if card.is_suit(&Suit::Hearts) {
        return 1;
    } else if card.is_suit(&Suit::Spades) && card.has_rank(&Rank::Queen) {
        return 13;
    }

    return 0;
}

#[derive(Debug)]
pub struct PlayedCard {
    pub card: Card,
    pub player_id: PlayerId,
}

#[derive(Debug)]
struct Player {
    player_id: PlayerId,
    hand: Vec<Card>,
    tricks: Vec<Trick>,
}

#[derive(Debug)]
struct Trick {
    lead: PlayerId,
    played: Vec<PlayedCard>,
}
