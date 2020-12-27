use hearts::game::{Game, Turn};

fn main() {
    let mut card_played = 0;
    let (mut game, mut turn) = Game::new();
    let hands = game.get_hands();

    println!("Player 1: {:?}", hands[0]);
    println!("Player 2: {:?}", hands[1]);
    println!("Player 3: {:?}", hands[2]);
    println!("Player 4: {:?}", hands[3]);

    while let Turn::PlayCard(player_id, choices) = turn {
        if card_played % 4 == 0 {
            println!("");
            println!("Trick {:?}:", card_played / 4 + 1);
        }

        println!(
            "  Player {}: {:?} <- {:?}",
            player_id + 1,
            choices[0],
            choices
        );

        card_played += 1;
        turn = game.play(player_id, choices[0].clone()).unwrap()
    }

    match turn {
        Turn::RoundEnd(score) => {
            println!("\nRound Score:");
            println!("  Player 1: {:?}", score[0]);
            println!("  Player 2: {:?}", score[1]);
            println!("  Player 3: {:?}", score[2]);
            println!("  Player 4: {:?}", score[3]);
        }
        _ => unimplemented!("Not Implemented"),
    }
}
