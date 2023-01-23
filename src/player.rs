use std::u32::MIN;

use crossterm::style::Stylize;

use crate::{
    board::{self, Board, Cities},
    common::Color,
    menu::menu_cancelable,
    player_card::{Event, Events, PlayerCard},
    role::Role,
};

pub const MAX_CARDS_IN_HAND: usize = 7;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub(crate) hand: Vec<PlayerCard>,
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
        self.hand.push(card);
        self.hand.sort_unstable();
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

    pub fn act(&mut self, board: &mut Board, action: usize) -> bool {
        match action {
            0 => true,
            1 => self.drive_ferry(board),
            2 => self.direct_flight(board),
            3 => self.chartered_flight(board),
            4 => self.shuttle_flight(board),
            5 => self.build_research_station(board),
            6 => self.treat_disease(board),
            7 => self.share_knowledge(board),
            8 => self.discover_cure(board),
            _ => false,
        }
    }

    pub fn play_event(&mut self, board: &mut Board, event: Event) -> bool {
        println!("{} played {}. TODO Implementation ;)", self.name, &event);
        board
            .player_discard
            .discard_to_top(PlayerCard::EventCard(event));
        true
    }

    pub fn drive_ferry(&mut self, board: &mut Board) -> bool {
        let mut adjacent_cities = Vec::from_iter(board.adjacent_to(&self.location).unwrap());
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

    pub fn direct_flight(&mut self, board: &mut Board) -> bool {
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
                        board.player_discard.discard_to_top(self.hand.remove(i));
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

    pub fn chartered_flight(&mut self, board: &mut Board) -> bool {
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
        let selection = menu_cancelable("Consume a card to take a Chartered Flight?", &city_cards);
        if selection == 0 {
            false
        } else {
            let mut cities = board.all_cities();
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
                            board.player_discard.discard_to_top(self.hand.remove(i));
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

    pub fn shuttle_flight(&mut self, board: &mut Board) -> bool {
        if !board.map.get(&self.location).unwrap().has_research_station {
            println!("{}'s current city, {}, does not have a research station, so shuttle flight is not available.", &self.name, &self.location);
            return false;
        }
        let mut cities_with_research_stations = Vec::new();
        for city in board.map.values() {
            if city.has_research_station {
                cities_with_research_stations.push(city.city.clone());
            }
        }
        cities_with_research_stations.sort_unstable();
        let selection = menu_cancelable(
            format!(
                "{}'s Shuttle Flight Menu from {}",
                &self.name, self.location
            )
            .as_str(),
            &cities_with_research_stations,
        );
        if selection == 0 {
            false
        } else {
            self.location = cities_with_research_stations[selection - 1];
            true
        }
    }

    pub fn build_research_station(&mut self, board: &mut Board) -> bool {
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
        let selection = menu_cancelable(format!("Should {} consume a card to build a Research Station? Currently {}/{} Research Stations", self.name.clone().bold(), board.total_research_stations(), board::MAX_RESEARCH_STATIONS).as_str(), &city_cards);
        if selection == 0 {
            false
        } else {
            if board.total_research_stations() >= board::MAX_RESEARCH_STATIONS {
                let mut cities_with_research_stations = Vec::new();
                for city in board.map.values() {
                    if city.has_research_station {
                        cities_with_research_stations.push(city.city.clone());
                    }
                }
                cities_with_research_stations.sort_unstable();
                let selection = menu_cancelable(
                    format!("{} can move a Research Station", &self.name).as_str(),
                    &cities_with_research_stations,
                );
                if selection == 0 {
                    return false;
                } else {
                    match board
                        .map
                        .get_mut(&cities_with_research_stations[selection - 1])
                    {
                        Some(x) => x.has_research_station = false,
                        None => (),
                    }
                }
            }
            let mut i = 0;
            while i < self.hand.len() {
                if let PlayerCard::CityCard(city) = &self.hand[i] {
                    if city.city == self.location {
                        board.player_discard.discard_to_top(self.hand.remove(i));
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
            match board.map.get_mut(&self.location) {
                Some(x) => x.has_research_station = true,
                None => (),
            }
            true
        }
    }

    pub fn treat_disease(&mut self, board: &mut Board) -> bool {
        match board.map.get_mut(&self.location) {
            Some(city) => {
                let options = [
                    format!(
                        "Treat {} ({}/{})",
                        Color::Blue,
                        city.blue_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY
                    ),
                    format!(
                        "Treat {} ({}/{})",
                        Color::Yellow,
                        city.yellow_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY
                    ),
                    format!(
                        "Treat {} ({}/{})",
                        Color::Black,
                        city.black_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY
                    ),
                    format!(
                        "Treat {} ({}/{})",
                        Color::Red,
                        city.red_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY
                    ),
                ];
                let selection = menu_cancelable(
                    format!(
                        "{} is Treating Disease in {}",
                        self.name.clone().bold(),
                        &self.location
                    )
                    .as_str(),
                    &options,
                );
                match selection {
                    0 => return false,
                    1 => {
                        if city.blue_infection_count > 0 {
                            if board.blue_disease == board::DiseaseState::Cured {
                                city.blue_infection_count = 0;
                            } else {
                                city.blue_infection_count -= 1;
                            }
                        } else {
                            return false;
                        }
                    }
                    2 => {
                        if city.yellow_infection_count > 0 {
                            if board.yellow_disease == board::DiseaseState::Cured {
                                city.yellow_infection_count = 0;
                            } else {
                                city.yellow_infection_count -= 1;
                            }
                        } else {
                            return false;
                        }
                    }
                    3 => {
                        if city.black_infection_count > 0 {
                            if board.black_disease == board::DiseaseState::Cured {
                                city.black_infection_count = 0;
                            } else {
                                city.black_infection_count -= 1;
                            }
                        } else {
                            return false;
                        }
                    }
                    4 => {
                        if city.red_infection_count > 0 {
                            if board.red_disease == board::DiseaseState::Cured {
                                city.red_infection_count = 0;
                            } else {
                                city.red_infection_count -= 1;
                            }
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                }
                true
            }
            None => false,
        }
    }

    pub fn share_knowledge(&mut self, board: &mut Board) -> bool {
        todo!()
    }

    pub fn discover_cure(&mut self, board: &mut Board) -> bool {
        todo!()
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
