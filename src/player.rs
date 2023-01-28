use std::u32::MIN;

use crossterm::style::{StyledContent, Stylize};

use crate::{
    board::{self, Board, Cities, DiseaseState},
    common::Color,
    menu::menu_cancelable,
    player_card::{Event, PlayerCard},
    role::Role,
};

pub const MAX_CARDS_IN_HAND: usize = 7;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
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

    pub fn name(&self) -> StyledContent<String> {
        self.name.clone().bold().with(self.role.color())
    }

    pub fn actions(&self) -> Vec<String> {
        let mut actions = Vec::from([
            "Drive / Ferry (Cost: 1 action)".to_owned(),
            "Direct Flight (Cost: 1 action)".to_owned(),
            "Charter Flight (Cost: 1 action)".to_owned(),
            "Shuttle Flight (Cost: 1 action)".to_owned(),
            "Build a Research Station (Cost: 1 action)".to_owned(),
            "Treat Disease (Cost: 1 action)".to_owned(),
            "Share Knowledge (Cost: 1 action)".to_owned(),
            "Discover a Cure (Cost: 1 action)".to_owned(),
        ]);
        actions
    }

    pub fn act(&mut self, board: &mut Board, players: &mut [Player], action: usize) -> bool {
        match action {
            0 => true,
            1 => self.drive_ferry(board),
            2 => self.direct_flight(board),
            3 => self.chartered_flight(board),
            4 => self.shuttle_flight(board),
            5 => self.build_research_station(board),
            6 => self.treat_disease(board),
            7 => self.share_knowledge(players),
            8 => self.discover_cure(board),
            _ => false,
        }
    }

    pub fn play_event(&mut self, board: &mut Board, event: Event) -> bool {
        println!("{} played {}. TODO Implementation ;)", self.name(), &event);
        board
            .player_discard
            .discard_to_top(PlayerCard::EventCard(event));
        true
    }

    pub fn drive_ferry(&mut self, board: &mut Board) -> bool {
        let mut adjacent_cities = Vec::from_iter(board.adjacent_to(self.location).unwrap());
        adjacent_cities.sort_unstable();
        let selection = menu_cancelable(
            format!(
                "{}'s Drive / Ferry Menu from {}",
                self.name(),
                self.location
            )
            .as_str(),
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
            format!(
                "{}'s Direct Flight Menu from {}",
                self.name(),
                self.location
            )
            .as_str(),
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
                    self.name(),
                    self.location
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
            println!("{}'s current city, {}, does not have a research station, so shuttle flight is not available.", self.name(), &self.location);
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
                self.name(),
                self.location
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
        let selection = menu_cancelable(format!("Should {} consume a card to build a Research Station? Currently {}/{} Research Stations", self.name(), board.total_research_stations(), board::MAX_RESEARCH_STATIONS).as_str(), &city_cards);
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
                    format!("{} can move a Research Station", self.name()).as_str(),
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
        let (blue_total, yellow_total, black_total, red_total) = (
            board.total_cubes(Color::Blue),
            board.total_cubes(Color::Yellow),
            board.total_cubes(Color::Black),
            board.total_cubes(Color::Red),
        );
        let (blue_status, yellow_status, black_status, red_status) = (
            if board.blue_disease != DiseaseState::Default {
                format!(" {}", &board.blue_disease)
            } else {
                String::default()
            },
            if board.yellow_disease != DiseaseState::Default {
                format!(" {}", &board.yellow_disease)
            } else {
                String::default()
            },
            if board.black_disease != DiseaseState::Default {
                format!(" {}", &board.black_disease)
            } else {
                String::default()
            },
            if board.red_disease != DiseaseState::Default {
                format!(" {}", &board.red_disease)
            } else {
                String::default()
            },
        );
        match board.map.get_mut(&self.location) {
            Some(city) => {
                let options = [
                    format!(
                        "Treat {} ({}/{}) [{}/{}{}]",
                        Color::Blue,
                        city.blue_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY,
                        blue_total,
                        board::MAX_INFECTION_PER_TYPE,
                        blue_status
                    ),
                    format!(
                        "Treat {} ({}/{}) [{}/{}{}]",
                        Color::Yellow,
                        city.yellow_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY,
                        yellow_total,
                        board::MAX_INFECTION_PER_TYPE,
                        yellow_status
                    ),
                    format!(
                        "Treat {} ({}/{}) [{}/{}{}]",
                        Color::Black,
                        city.black_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY,
                        black_total,
                        board::MAX_INFECTION_PER_TYPE,
                        black_status
                    ),
                    format!(
                        "Treat {} ({}/{}) [{}/{}{}]",
                        Color::Red,
                        city.red_infection_count,
                        board::MAX_INFECTION_PER_TYPE_PER_CITY,
                        red_total,
                        board::MAX_INFECTION_PER_TYPE,
                        red_status
                    ),
                ];
                let selection = menu_cancelable(
                    format!("{} is Treating Disease in {}", self.name(), &self.location).as_str(),
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

    pub fn share_knowledge(&mut self, players: &mut [Player]) -> bool {
        let mut options = Vec::new();
        let mut actions = Vec::new();
        for (i, player) in players.iter().enumerate() {
            if self.location == player.location {
                for (j, card) in self.hand.iter().enumerate() {
                    match card {
                        PlayerCard::CityCard(city) => {
                            if self.role == Role::Researcher || city.city == self.location {
                                options.push(format!(
                                    "Give {} to {} | {}/{} cards",
                                    card,
                                    player.name(),
                                    player.hand.len(),
                                    MAX_CARDS_IN_HAND
                                ));
                                actions.push((players.len(), i, j));
                            }
                        }
                        _ => (),
                    }
                }
                for (j, card) in player.hand.iter().enumerate() {
                    match card {
                        PlayerCard::CityCard(city) => {
                            if player.role == Role::Researcher || city.city == self.location {
                                options.push(format!(
                                    "Take {} from {} | {}/{} cards",
                                    card,
                                    player.name(),
                                    player.hand.len(),
                                    MAX_CARDS_IN_HAND
                                ));
                                actions.push((i, players.len(), j));
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
        let selection = menu_cancelable(
            format!(
                "{}'s Share Knowledge Menu | {}/{} cards",
                self.name(),
                self.hand.len(),
                MAX_CARDS_IN_HAND
            )
            .as_str(),
            &options,
        );
        if selection == 0 {
            return false;
        }
        let (from_idx, to_idx, card_idx) = actions[selection - 1];
        if from_idx == to_idx {
            return false;
        }
        if from_idx == players.len() {
            players[to_idx].add_to_hand(self.hand.remove(card_idx));
        } else if to_idx == players.len() {
            self.add_to_hand(players[from_idx].hand.remove(card_idx));
        } else {
            return false;
        }
        true
    }

    pub fn discover_cure(&mut self, board: &mut Board) -> bool {
        const MIN_CARDS_TO_CURE: u8 = 5;
        let city = board.map.get(&self.location).unwrap();
        true
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} ({}) in {}\nwith",
            self.name(),
            self.role,
            self.location
        )?;

        for card in &self.hand {
            writeln!(f, "\t{}", &card)?;
        }

        write!(f, "{}", "in hand.")
    }
}
