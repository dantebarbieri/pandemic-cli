pub mod player {
    use std::u32::MIN;

    use crossterm::style::Stylize;

    use crate::{
        map::map::{Cities, Map},
        menu::{menu, menu_cancelable},
        player_card::player_card::PlayerCard,
        role::role::Role,
    };

    #[derive(Debug, Clone)]
    pub struct Player {
        pub name: String,
        hand: Vec<PlayerCard>,
        location: Cities,
        role: Role,
    }

    impl Player {
        pub fn new(name: &str, role: Role) -> Self {
            Self {
                name: String::from(name),
                hand: Vec::new(),
                location: Cities::Atlanta,
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

        pub fn act(&mut self, action: usize) -> bool {
            match action {
                1 => self.drive_ferry(),
                2 => self.direct_flight(),
                3 => self.chartered_flight(),
                _ => false,
            }
        }

        pub fn drive_ferry(&mut self) -> bool {
            let map = Map::new();
            let mut adjacent_cities = Vec::from_iter(map.adjacent_to(&self.location).unwrap());
            adjacent_cities.sort_unstable();
            let selection = menu_cancelable(
                format!("{}'s Drive / Ferry Menu from {}", &self.name, self.location).as_str(),
                &adjacent_cities,
            );
            if selection == 0 {
                false
            } else {
                self.location = adjacent_cities[selection - 1];
                true
            }
        }

        pub fn direct_flight(&mut self) -> bool {
            let mut city_cards = Vec::new();
            for card in &self.hand {
                match card {
                    PlayerCard::CityCard(city) => city_cards.push(city.city),
                    _ => (),
                }
            }
            city_cards.sort_unstable();
            let selection = menu_cancelable(
                format!("{}'s Direct Flight Menu from {}", &self.name, self.location).as_str(),
                &city_cards,
            );
            if selection == 0 {
                false
            } else {
                self.location = city_cards[selection - 1];
                let mut i = 0;
                while i < self.hand.len() {
                    if let PlayerCard::CityCard(city) = &self.hand[i] {
                        if city.city == self.location {
                            self.hand.remove(i);
                        } else {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                true
            }
        }

        pub fn chartered_flight(&mut self) -> bool {
            let mut city_cards = Vec::new();
            for card in &self.hand {
                match card {
                    PlayerCard::CityCard(city) => {
                        if city.city == self.location {
                            city_cards.push(city.city);
                        }
                    }
                    _ => (),
                }
            }
            let selection =
                menu_cancelable("Consume a card to take a Chartered Flight?", &city_cards);
            if selection == 0 {
                false
            } else {
                let map = Map::new();
                let mut cities = map.all_cities();
                cities.sort_unstable();
                let selection = menu_cancelable(
                    format!(
                        "{}'s Charter Flight Menu from {}",
                        &self.name, self.location
                    )
                    .as_str(),
                    &cities,
                );
                if selection == 0 {
                    false
                } else {
                    let mut i = 0;
                    while i < self.hand.len() {
                        if let PlayerCard::CityCard(city) = &self.hand[i] {
                            if city.city == self.location {
                                self.hand.remove(i);
                            } else {
                                i += 1;
                            }
                        } else {
                            i += 1;
                        }
                    }
                    self.location = cities[selection - 1];
                    true
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
