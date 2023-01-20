mod common;
mod deck;
mod map;
mod menu;
mod player;
mod player_card;
mod role;

use std::io::Write;

use crossterm::style::Stylize;

use crate::{
    deck::Deck, player::player::Player, player_card::player_card::PlayerCard, role::role::RoleCard, menu::menu,
};

fn main() {
    let mut deck: Deck<RoleCard> = Deck::new();
    deck.fill();
    deck.shuffle();

    print!("Enter number of players [2-4]: ");
    std::io::stdout().flush();

    let mut player_count = String::new();

    std::io::stdin()
        .read_line(&mut player_count)
        .expect("Failed to read line");

    let player_count: u32 = player_count.trim().parse().expect("Please type a number!");

    let mut players = Vec::new();

    for i in 0..player_count {
        let mut player_name = String::new();

        print!("Enter Player #{}'s Name: ", i + 1);
        std::io::stdout().flush();

        std::io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

        match deck.draw_from_top() {
            Some(role_card) => players.push(Player::new(player_name.trim(), role_card.0)),
            None => (),
        }
    }

    let mut deck: Deck<PlayerCard> = Deck::new();
    deck.fill();
    deck.shuffle();

    for _ in 0..6 - player_count {
        deck.deal(&mut players);
    }

    let difficulties = ["Introductory (4 Epidemics)", "Standard (5 Epidemics)", "Heroic (6 Epidemics)"];

    let difficulty = menu("Set Difficulty", &difficulties) - 1;

    deck.add_epidemic_cards((difficulty + 4).try_into().unwrap_or(4));

    let difficulty = difficulties[difficulty];

    players.sort_by(|a, b| a.max_population_city().cmp(&b.max_population_city()));
    players.reverse();

    for player in &players {
        println!("{}", &player);
    }

    println!("Using {} difficulty.", difficulty);

    let mut turn_idx = 0 as usize;
    loop {
        let mut action = 0;
        while action < 4 {
            println!("{}\nPlease take your turn. Used {}/4 actions.", players[turn_idx], action);
            let actions = players[turn_idx].actions();
            // TODO: Loop until selection is nonzero
            let selection = menu(format!("Action Menu For {}", players[turn_idx].name).as_str(), actions);

            println!("{} selected {}.", players[turn_idx].name, actions[selection]);
            if players[turn_idx].act(selection)
            {
                action += 1;
            }
        }
        
        turn_idx += 1;
        turn_idx %= players.len();
    }
}
