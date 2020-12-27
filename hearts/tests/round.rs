use hearts::cards::{Card, Rank, Suit};
use hearts::round::Round;

// IMPORTANT: The following test cases use a round of hearts that was master planned to test all
// test cases. Chaning any of these is dangerous unless you manually verify each test cases still
// holds. A bad idea? Certainly, but it did reduce the number of test cases until I have a better
// way to express them.
#[test]
fn rejects_non_two_clubs_as_first_card() {
    let mut round = create_round();

    let err = round.play(1, card(Rank::Three, Suit::Clubs));

    assert_eq!(err.unwrap_err(), "Player 1 violated rules by playing 3♣.");
}

#[test]
fn cannot_play_out_of_turn() {
    let mut round = create_round();
    play_cards(&mut round, 1);

    let err = round.play(0, card(Rank::Three, Suit::Diamonds));

    assert_eq!(err.unwrap_err(), "Player 0 played out of turn.");
}

#[test]
fn must_follow_suit() {
    let mut round = create_round();
    play_cards(&mut round, 1);

    let err = round.play(2, card(Rank::Ace, Suit::Spades));

    assert_eq!(err.unwrap_err(), "Player 2 violated rules by playing A♠.");
}

#[test]
fn cannot_play_heart_on_first_round() {
    let mut round = create_round();
    play_cards(&mut round, 3);

    let err = round.play(0, card(Rank::Ace, Suit::Hearts));

    assert_eq!(err.unwrap_err(), "Player 0 violated rules by playing A♥.");
}

#[test]
fn cannot_play_queen_spades_on_first_round() {
    let mut round = create_round();
    play_cards(&mut round, 3);

    let err = round.play(0, card(Rank::Queen, Suit::Spades));

    assert_eq!(err.unwrap_err(), "Player 0 violated rules by playing Q♠.");
}

#[test]
fn cannot_lead_heart_when_not_broken() {
    let mut round = create_round();
    play_cards(&mut round, 4);

    let err = round.play(0, card(Rank::Three, Suit::Hearts));

    assert_eq!(err.unwrap_err(), "Player 0 violated rules by playing 3♥.");
}

#[test]
fn queen_of_spades_does_not_break_hearts() {
    let mut round = create_round();
    play_cards(&mut round, 8);

    let err = round.play(0, card(Rank::Three, Suit::Hearts));

    assert_eq!(err.unwrap_err(), "Player 0 violated rules by playing 3♥.");
}

#[test]
fn can_break_hearts() {
    let mut round = create_round();
    play_cards(&mut round, 9);

    let err = round.play(1, card(Rank::Ace, Suit::Hearts));

    assert!(err.is_ok());
}

#[test]
fn can_lead_hearts_after_broken() {
    let mut round = create_round();
    play_cards(&mut round, 12);

    let err = round.play(3, card(Rank::Two, Suit::Hearts));

    assert!(err.is_ok());
}

#[test]
fn can_lead_heats_unbroken_as_only_choice() {
    let mut round = Round::with_hands(vec![
        vec![
            card(Rank::Two, Suit::Clubs),
            card(Rank::Three, Suit::Spades),
        ],
        vec![
            card(Rank::Ace, Suit::Hearts),
            card(Rank::King, Suit::Hearts),
        ],
        vec![
            // I don't know what is allowed in this case
            card(Rank::Queen, Suit::Spades),
            card(Rank::Queen, Suit::Hearts),
        ],
        vec![
            card(Rank::Two, Suit::Hearts),
            card(Rank::Three, Suit::Hearts),
        ],
    ]);

    round.play(0, card(Rank::Two, Suit::Clubs)).unwrap();
    let err = round.play(1, card(Rank::Ace, Suit::Hearts));

    println!("{:?}", err);
    assert!(err.is_ok())
}

#[test]
fn test_plays() {
    let mut round = create_round();
    play_cards(&mut round, 20);
}

// The game plated looks the following, where * designates the card led.
// 0: A♣  5♣* 2♦* 3♥  8♣
// 1: 2♣* 3♣  A♥  5♥  7♠*
// 2: 4♣  A♠  5♦  4♥  6♥
// 3: A♦  Q♠  K♦  2♥* 6♦
fn create_round() -> Round {
    Round::with_hands(vec![
        vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            },
            Card {
                // Lead
                rank: Rank::Five,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Two,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Eight,
                suit: Suit::Clubs,
            },
        ],
        vec![
            Card {
                // Lead
                rank: Rank::Two,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Hearts,
            },
            Card {
                // Lead
                rank: Rank::Seven,
                suit: Suit::Spades,
            },
        ],
        vec![
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs,
            },
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts,
            },
        ],
        vec![
            Card {
                rank: Rank::Ace,
                suit: Suit::Diamonds,
            },
            Card {
                rank: Rank::Queen,
                suit: Suit::Spades,
            },
            Card {
                // Lead
                rank: Rank::King,
                suit: Suit::Diamonds,
            },
            Card {
                // Lead
                rank: Rank::Two,
                suit: Suit::Hearts,
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Diamonds,
            },
        ],
    ])
}

fn play_cards(round: &mut Round, n: usize) {
    for _ in 0..n {
        let player_id = round.get_turn().unwrap();
        let card = round.get_player_cards(player_id)[0].clone();
        round.play(player_id, card).unwrap();
    }
}

fn card(rank: Rank, suit: Suit) -> Card {
    Card { rank, suit }
}
