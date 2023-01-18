pub mod player {
    use std::u32::MIN;

    use crossterm::style::Stylize;

    use crate::{
        map::map::Map, menu::{menu, menu_cancelable}, player_card::player_card::PlayerCard, role::role::Role,
    };

    #[derive(Debug, Clone)]
    pub struct Player {
        pub name: String,
        hand: Vec<PlayerCard>,
        location: &'static str,
        role: Role,
    }

    impl Player {
        pub fn new(name: &str, role: Role) -> Self {
            Self {
                name: String::from(name),
                hand: Vec::new(),
                location: "Atlanta",
                role,
            }
        }

        pub fn add_to_hand(&mut self, card: PlayerCard) {
            self.hand.push(card)
        }

        pub fn max_population_city(&self) -> u32 {
            let mut max = MIN;
            for card in &self.hand {
                max = std::cmp::max(
                    max,
                    match card {
                        PlayerCard::CityCard(card) => card.population,
                        _ => max,
                    },
                )
            }
            max
        }

        pub fn actions(&self) -> &[&str] {
            &[
                "Drive / Ferry",
                "Direct Flight",
                "Charter Flight",
                "Shuttle Flight",
                "Build a Research Station",
                "Treat Disease",
                "Share Knowledge",
                "Discover a Cure",
            ]
        }

        pub fn act(&mut self, action: usize) {
            match action {
                // TODO: 0 should repeat previous menu
                1 => self.drive_ferry(),
                2 => self.direct_flight(),
                _ => (),
            }
        }

        pub fn drive_ferry(&mut self) {
            let map = Map::new();
            let mut adjacent_cities =
                Vec::from_iter((*map.adjacent_to(self.location).unwrap()).clone());
            adjacent_cities.sort_unstable();
            // TODO: Handle cancel menu
            let selection = menu_cancelable(
                format!("{}'s Drive / Ferry Menu from {}", &self.name, self.location).as_str(),
                &adjacent_cities,
            )
            .expect("Expected a number.");
            self.location = adjacent_cities[selection];
        }

        pub fn direct_flight(&mut self) {
            let mut city_cards = Vec::new();
            for card in &self.hand {
                match card {
                    PlayerCard::CityCard(city) => city_cards.push(city.name),
                    _ => (),
                }
            }
            city_cards.sort_unstable();
            // TODO: Handle cancel menu
            let selection = menu_cancelable(
                format!("{}'s Direct Flight Menu from {}", &self.name, self.location).as_str(),
                &city_cards,
            )
            .expect("Expected a number.");
            self.location = city_cards[selection];
            let mut i = 0;
            while i < self.hand.len() {
                if let PlayerCard::CityCard(city) = &self.hand[i] {
                    if city.name == self.location {
                        self.hand.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
        }
    }

    impl std::fmt::Display for Player {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(
                f,
                "{} ({}) in {}\nwith",
                self.name.clone().bold(),
                self.role,
                self.location
            )?;

            for card in &self.hand {
                writeln!(f, "\t{}", &card)?;
            }

            write!(f, "{}", "in hand.")
        }
    }
}
