use super::cards::Card;
use super::round::{PlayerId, Round};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Game {
    score: [i8; 4],
    round: usize,
    current_round: Round,
}

#[derive(Debug)]
pub enum Move {
    Play(Card),
}

#[derive(Debug)]
pub enum Turn {
    PlayCard(PlayerId, Vec<Card>),
    PassCard(PlayerId, Direction),
    RoundEnd([i8; 4]),
    GameEnd([i8; 4]),
}

#[derive(Debug)]
pub enum Direction {
    Clockwise,
    Counterclockwise,
    Across,
}

impl Game {
    pub fn new() -> (Game, Turn) {
        let round = 1;
        let mut game = Game {
            score: [0, 0, 0, 0],
            round,
            current_round: Round::new(),
        };
        let turn = game.next_turn();

        (game, turn)
    }

    pub fn play(&mut self, player_id: PlayerId, card: Card) -> Result<Turn, String> {
        self.current_round
            .play(player_id, card)
            .map(|_| self.next_turn())
    }

    pub fn get_hands(&self) -> Vec<&Vec<Card>> {
        vec![
            self.current_round.get_player_cards(0),
            self.current_round.get_player_cards(1),
            self.current_round.get_player_cards(2),
            self.current_round.get_player_cards(3),
        ]
    }

    fn next_turn(&mut self) -> Turn {
        // FIXME Not Implemented: Passing
        match self.current_round.get_turn() {
            None => {
                let points = self.current_round.score();
                for (player_id, score) in points.iter().enumerate() {
                    self.score[player_id] += score;
                }

                Turn::RoundEnd(points)
            }
            Some(player_id) => {
                Turn::PlayCard(player_id, self.current_round.get_valid_plays(player_id))
            }
        }
    }
}
