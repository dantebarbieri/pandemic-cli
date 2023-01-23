mod common;
mod deck;
mod board;
mod infection_card;
mod menu;
mod player;
mod player_card;
mod role;

use std::io::Write;

use crossterm::style::Stylize;

use crate::{
    deck::Deck, player::Player, role::RoleCard, menu::{menu, menu_injectable}, board::Board, player_card::PlayerCard,
};

fn main() {
    let mut deck = Deck::<RoleCard>::new();
    deck.shuffle();

    print!("Enter number of players [2-4]: ");
    std::io::stdout().flush().unwrap_or_default();

    let mut player_count = String::new();

    std::io::stdin()
        .read_line(&mut player_count)
        .expect("Failed to read line");

    let player_count: u32 = player_count.trim().parse().expect("Please type a number!");

    let mut players = Vec::new();

    for i in 0..player_count {
        let mut player_name = String::new();

        print!("Enter Player #{}'s Name: ", i + 1);
        std::io::stdout().flush().unwrap_or_default();

        std::io::stdin()
            .read_line(&mut player_name)
            .expect("Failed to read line");

        match deck.draw_from_top() {
            Some(role_card) => players.push(Player::new(player_name.trim(), role_card.role)),
            None => (),
        }
    }

    let mut board = Board::new();
    board.player_deck.shuffle();
    board.infection_deck.shuffle();

    for _ in 0..6 - player_count {
        board.player_deck.deal(&mut players);
    }

    let difficulties = ["Introductory (4 Epidemics)", "Standard (5 Epidemics)", "Heroic (6 Epidemics)"];

    let difficulty = menu("Set Difficulty", &difficulties) - 1;

    board.add_epidemic_cards((difficulty + 4).try_into().unwrap_or(4));

    let difficulty = difficulties[difficulty];

    players.sort_by(|a, b| a.max_population_city().cmp(&b.max_population_city()));
    players.reverse();

    for player in &players {
        println!("{}", &player);
    }

    println!("Using {} difficulty.", difficulty);

    for i in 0..board::MAX_INFECTION_PER_TYPE_PER_CITY {
        for _ in 0..board::NUMBER_OF_CITIES_TO_INFECT_PER_ROUND_AT_START {
            match board.infection_deck.draw_from_top() {
                Some(infection_card) => {
                    let quantity = board::MAX_INFECTION_PER_TYPE_PER_CITY - i;
                    println!("Infected {} {} time{}!", &infection_card.city, quantity, if quantity > 1 { "s" } else { "" });
                    for _ in 0..quantity {
                        board.infect_city(&infection_card.city);
                    }
                    board.infection_discard.discard_to_top(infection_card);
                },
                None => ()
            }
        }
    }

    println!("{:?}", board.player_deck);

    let mut turn_idx = 0 as usize;
    loop {
        let mut action = 0;
        while action < 4 {
            println!("{}\nPlease take your turn. Used {}/4 actions.", players[turn_idx], action);
            let actions = players[turn_idx].actions();
            let selection = menu_injectable(format!("Action Menu For {}", players[turn_idx].name).as_str(), "Do nothing (consume an action)", actions);

            if players[turn_idx].act(&mut board, selection)
            {
                action += 1;
            }
        }

        let mut event_cards = Vec::new();

        for card in players[turn_idx].hand.clone() {
            match card {
                PlayerCard::EventCard(event) => event_cards.push(event),
                _ => ()
            }
        }

        while !event_cards.is_empty() {
            let selection = menu_injectable(format!("Does {} wish to play any Event cards before drawing 2 cards? {}/{} cards in hand", players[turn_idx].name, players[turn_idx].hand.len(), player::MAX_CARDS_IN_HAND).as_str(), "Draw 2 cards from player deck", &event_cards);

            if selection == 0 { break; }
            let selection = selection - 1;
            let idx = players[turn_idx].hand.iter().position(|x| *x == PlayerCard::EventCard(event_cards[selection])).unwrap();
            players[turn_idx].hand.remove(idx);
            let event = event_cards.remove(selection);
            players[turn_idx].play_event(&mut board, event);
        }

        for _ in 0..board::DRAW_CARDS_PER_ROUND {
            match board.player_deck.draw_from_top() {
                Some(card) => match card {
                    PlayerCard::EpidemicCard => {
                        println!("{} drew an Epidemic Card!", players[turn_idx].name);
                        board.increase_infection_rate();
                        match board.infection_deck.draw_from_bottom() {
                            Some(infection_card) => {
                                println!("An Epidemic breaks out in {}", &infection_card.city);
                                let mut game_continue = true;
                                for _ in 0..board::MAX_INFECTION_PER_TYPE_PER_CITY {
                                    game_continue = board.infect_city(&infection_card.city) && game_continue;
                                }
                                if !game_continue {
                                    if board.outbreaks >= board::MAX_OUTBREAKS {
                                        println!("Game Over: A worldwide pandemic happens! (8 outbreaks occurred)");
                                    } else {
                                        println!("Game Over: A disease spread too much! (Not enough disease cubes are left)");
                                    }
                                    return;
                                }
                                board.infection_discard.discard_to_top(infection_card);
                                board.infection_discard.shuffle();
                                board.infection_deck.append(&mut board.infection_discard);
                            },
                            None => ()
                        }

                    },
                    _ => {
                        println!("{} drew {}", &players[turn_idx].name, &card);
                        players[turn_idx].add_to_hand(card)
                    }
                },
                None => {
                    println!("Game Over: Your team ran out of time! (There are not enough player cards left)");
                    return;
                }
            }
        }

        while players[turn_idx].hand.len() > player::MAX_CARDS_IN_HAND {
            let selection = menu(format!("Discard Cards in {}'s Hand: {}/{} cards", players[turn_idx].name.clone().bold(), players[turn_idx].hand.len(), player::MAX_CARDS_IN_HAND).as_str(), &players[turn_idx].hand);
            players[turn_idx].hand.remove(selection - 1);
        }

        for _ in 0..board.infection_rate() {
            match board.infection_deck.draw_from_top() {
                Some(infection_card) => {
                    println!("{} was infected!", &infection_card.city);
                    board.infect_city(&infection_card.city);
                }
                None => ()
            }
        }
        
        turn_idx += 1;
        turn_idx %= players.len();
    }
}
