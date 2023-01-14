use rand::seq::SliceRandom;

use crate::player_card::player_card::make_deck;

mod common;
mod player_card;

fn main() {
    let mut deck = make_deck();

    deck.shuffle(&mut rand::thread_rng());

    for card in deck {
        println!("{:?}", card);
    }

}
