use std::collections::VecDeque;

use crate::{deck::Deck, board::Cities, common::Color};

pub struct InfectionCard {
    pub(crate) city: Cities,
    color: Color
}

impl Deck<InfectionCard> {
    pub fn new() -> Self {
        Self(VecDeque::from(make_deck()))
    }
}

fn make_deck() -> Vec<InfectionCard> {
    let mut cities = Vec::new();

    cities.append(&mut make_blue_cities());
    cities.append(&mut make_yellow_cities());
    cities.append(&mut make_black_cities());
    cities.append(&mut make_red_cities());

    cities
}

fn make_blue_cities() -> Vec<InfectionCard> {
    vec![
        InfectionCard {
            city: Cities::Atlanta,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::Chicago,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::Essen,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::London,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::Madrid,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::Milan,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::Montreal,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::NewYork,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::Paris,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::SaintPetersburg,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::SanFrancisco,
            color: Color::Blue,
        },
        InfectionCard {
            city: Cities::Washington,
            color: Color::Blue,
        },
    ]
}

fn make_yellow_cities() -> Vec<InfectionCard> {
    vec![
        InfectionCard {
            city: Cities::Bogota,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::BuenosAires,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::Johannesburg,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::Kinshasa,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::Khartoum,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::Lagos,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::Lima,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::LosAngeles,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::MexicoCity,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::Miami,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::Santiago,
            color: Color::Yellow,
        },
        InfectionCard {
            city: Cities::SaoPaulo,
            color: Color::Yellow,
        },
    ]
}

fn make_black_cities() -> Vec<InfectionCard> {
    vec![
        InfectionCard {
            city: Cities::Algiers,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Baghdad,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Cairo,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Chennai,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Delhi,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Istanbul,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Karachi,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Kolkata,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Mumbai,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Moscow,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Riyadh,
            color: Color::Black,
        },
        InfectionCard {
            city: Cities::Tehran,
            color: Color::Black,
        },
    ]
}

fn make_red_cities() -> Vec<InfectionCard> {
    vec![
        InfectionCard {
            city: Cities::Bangkok,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Beijing,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::HoChiMinhCity,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::HongKong,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Jakarta,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Manila,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Osaka,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Seoul,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Shanghai,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Sydney,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Taipei,
            color: Color::Red,
        },
        InfectionCard {
            city: Cities::Tokyo,
            color: Color::Red,
        },
    ]
}