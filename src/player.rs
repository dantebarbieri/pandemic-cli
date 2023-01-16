pub mod player {
    use std::u32::MIN;

    use crossterm::style::Stylize;

    use crate::{
        map::map::Map, menu::menu, player_card::player_card::PlayerCard, role::role::Role,
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
                0 => self.drive(),
                _ => ()
            }
        }

        pub fn drive(&mut self) {
            let map = Map::new();
            let mut adjacent_cities =
                Vec::from_iter((*map.adjacent_to(self.location).unwrap()).clone());
            adjacent_cities.sort_unstable();
            let selection = menu(
                format!("{}'s Drive / Ferry Menu from {}", &self.name, self.location).as_str(),
                &adjacent_cities,
            ).expect("Expected a number.");
            self.location = adjacent_cities[selection];
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
