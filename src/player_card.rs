pub mod player_card {
    use std::collections::HashSet;

    use crate::common::common::Color;

    pub struct City {
        name: &'static str,
        color: Color,
        country: &'static str,
        flag: &'static str,
        population: u32,
        population_density: u16,
        adjacent_cities: HashSet<&'static str>,
    }

    pub struct Event {
        name: &'static str,
        event: &'static str,
    }

    pub enum PlayerCard {
        CityCard(City),
        EpidemicCard,
        EventCard(Event),
    }

    pub fn make_deck() -> Vec<PlayerCard> {
        let mut deck = Vec::new();

        deck.append(&mut make_cities());
        deck.append(&mut make_events());

        deck
    }

    fn make_cities() -> Vec<PlayerCard> {
        let mut cities = Vec::new();

        cities.append(&mut make_blue_cities());
        cities.append(&mut make_yellow_cities());
        cities.append(&mut make_black_cities());
        cities.append(&mut make_red_cities());

        cities
    }

    fn make_blue_cities() -> Vec<PlayerCard> {
        vec![
            PlayerCard::CityCard(City {
                name: "Atlanta",
                color: Color::Blue,
                country: "United States",
                flag: "🇺🇸",
                population: 4715000,
                population_density: 700,
                adjacent_cities: HashSet::from(["Chicago", "Miami", "Washington"]),
            }),
            PlayerCard::CityCard(City {
                name: "Chicago",
                color: Color::Blue,
                country: "United States",
                flag: "🇺🇸",
                population: 9121000,
                population_density: 1300,
                adjacent_cities: HashSet::from([
                    "Atlanta",
                    "Los Angeles",
                    "Mexico City",
                    "Montréal",
                    "San Francisco",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Essen",
                color: Color::Blue,
                country: "Germany",
                flag: "🇩🇪",
                population: 575000,
                population_density: 2800,
                adjacent_cities: HashSet::from(["London", "Milan", "Paris", "Saint Petersburg"]),
            }),
            
        ]
    }

    fn make_yellow_cities() -> Vec<PlayerCard> {
        vec![]
    }

    fn make_black_cities() -> Vec<PlayerCard> {
        vec![]
    }

    fn make_red_cities() -> Vec<PlayerCard> {
        vec![]
    }

    fn make_events() -> Vec<PlayerCard> {
        vec![
            PlayerCard::EventCard(Event {
                name: "Airlift",
                event: "Move any 1 pawn to any city. Get permission before moving another player's pawn."
            }), PlayerCard::EventCard(Event {
                name: "Forecast",
                event: "Draw, look at, and rearrange the top 6 cards of the Infection Deck. Put them back on top."
            }), PlayerCard::EventCard(Event {
                name: "Government Grant",
                event: "Add 1 research station to any city (no City card needed)."
            }), PlayerCard::EventCard(Event {
                name: "One Quiet Night",
                event: "Skip the next Infect Cities step (do not flip over any Infection cards)."
            }), PlayerCard::EventCard(Event {
                name: "Resilient Population",
                event: "Remove any 1 card in the Infection Discard Pile from the game. You may play this between the Infect and Intensify steps of an epidemic."
            })
        ]
    }
}
