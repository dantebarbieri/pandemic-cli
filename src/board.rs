use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Color;
use crate::deck::Deck;
use crate::infection_card::InfectionCard;
use crate::player_card::PlayerCard;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Cities {
    Algiers,
    Atlanta,
    Baghdad,
    Bangkok,
    Beijing,
    Bogota,
    BuenosAires,
    Cairo,
    Chennai,
    Chicago,
    Delhi,
    Essen,
    HoChiMinhCity,
    HongKong,
    Istanbul,
    Jakarta,
    Johannesburg,
    Karachi,
    Khartoum,
    Kinshasa,
    Kolkata,
    Lagos,
    Lima,
    London,
    LosAngeles,
    Madrid,
    Manila,
    MexicoCity,
    Miami,
    Milan,
    Montreal,
    Moscow,
    Mumbai,
    NewYork,
    Osaka,
    Paris,
    Riyadh,
    SaintPetersburg,
    SanFrancisco,
    Santiago,
    SaoPaulo,
    Seoul,
    Shanghai,
    Sydney,
    Taipei,
    Tehran,
    Tokyo,
    Washington,
}

#[derive(Debug)]
pub struct City {
    pub(crate) city: Cities,
    pub(crate) color: Color,
    pub(crate) adjacent_cities: HashSet<Cities>,
    pub(crate) has_research_station: bool,
    pub(crate) blue_infection_count: u8,
    pub(crate) yellow_infection_count: u8,
    pub(crate) black_infection_count: u8,
    pub(crate) red_infection_count: u8,
    current_outbreak: bool
}

impl City {
    pub fn new(city: Cities, color: Color, adjacent_cities: HashSet<Cities>) -> Self {
        Self {
            city,
            color,
            adjacent_cities,
            has_research_station: false,
            blue_infection_count: 0,
            yellow_infection_count: 0,
            black_infection_count: 0,
            red_infection_count: 0,
            current_outbreak: false,
        }
    }

    pub fn new_with_research_station(
        city: Cities,
        color: Color,
        adjacent_cities: HashSet<Cities>,
    ) -> Self {
        Self {
            city,
            color,
            adjacent_cities,
            has_research_station: true,
            blue_infection_count: 0,
            yellow_infection_count: 0,
            black_infection_count: 0,
            red_infection_count: 0,
            current_outbreak: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum DiseaseState {
    Default,
    Cured,
    Eradicated,
}

pub const DRAW_CARDS_PER_ROUND: u8 = 2;
pub const NUMBER_OF_CITIES_TO_INFECT_PER_ROUND_AT_START: u8 = 3;
pub const MAX_RESEARCH_STATIONS: u8 = 6;
pub const MAX_INFECTION_PER_TYPE: u8 = 24;
pub const MAX_INFECTION_PER_TYPE_PER_CITY: u8 = 3;
pub const MAX_OUTBREAKS: u8 = 8;
pub const INFECTION_RATE: [u8; 7] = [2, 2, 2, 3, 3, 4, 4];

pub struct Board {
    pub(crate) map: HashMap<Cities, City>,
    pub(crate) player_deck: Deck<PlayerCard>,
    pub(crate) player_discard: Deck<PlayerCard>,
    pub(crate) infection_deck: Deck<InfectionCard>,
    pub(crate) infection_discard: Deck<InfectionCard>,
    pub(crate) blue_disease: DiseaseState,
    pub(crate) yellow_disease: DiseaseState,
    pub(crate) black_disease: DiseaseState,
    pub(crate) red_disease: DiseaseState,
    pub(crate) outbreaks: u8,
    pub(crate) epidemics: u8,
    max_epidemics: u8,
    infection_rate: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut cities = HashMap::new();

        for city in make_cities() {
            cities.insert(city.city, city);
        }

        Self {
            map: cities,
            player_deck: Deck::<PlayerCard>::new(),
            player_discard: Deck::new_empty(),
            infection_deck: Deck::<InfectionCard>::new(),
            infection_discard: Deck::new_empty(),
            blue_disease: DiseaseState::Default,
            yellow_disease: DiseaseState::Default,
            black_disease: DiseaseState::Default,
            red_disease: DiseaseState::Default,
            outbreaks: 0,
            epidemics: 0,
            max_epidemics: 0,
            infection_rate: 0,
        }
    }

    pub fn adjacent_to(&self, city: &Cities) -> Option<HashSet<Cities>> {
        match self.map.get(&city) {
            Some(city) => Some(city.adjacent_cities.clone()),
            None => None,
        }
    }

    pub fn is_adjacent(&self, from: &Cities, to: &Cities) -> bool {
        match self.adjacent_to(from) {
            Some(cities) => cities.contains(to),
            None => match self.adjacent_to(to) {
                Some(cities) => cities.contains(from),
                None => false,
            },
        }
    }

    pub fn all_cities(&self) -> Vec<Cities> {
        let mut cities = Vec::new();
        for key in self.map.keys() {
            cities.push(key.clone());
        }
        cities
    }

    pub fn add_epidemic_cards(&mut self, epidemics: u8) {
        self.max_epidemics = epidemics;
        self.player_deck.add_epidemic_cards(self.max_epidemics)
    }

    pub fn total_cubes(&self, color: &Color) -> u8 {
        let mut acc = 0;
        for city in self.map.values() {
            if city.color == *color {
                acc += match color {
                    Color::Blue => city.blue_infection_count,
                    Color::Yellow => city.yellow_infection_count,
                    Color::Black => city.black_infection_count,
                    Color::Red => city.red_infection_count,
                }
            }
        }
        acc
    }

    pub fn total_research_stations(&self) -> u8 {
        let mut acc = 0;
        for city in self.map.values() {
            acc += city.has_research_station as u8;
        }
        acc
    }

    pub fn infect_city(&mut self, city: &Cities) -> bool {
        let city_obj = self.map.get(city).unwrap();
        match &city_obj.color {
            Color::Blue => if self.blue_disease == DiseaseState::Eradicated { return true; },
            Color::Yellow => if self.yellow_disease == DiseaseState::Eradicated { return true; },
            Color::Black => if self.black_disease == DiseaseState::Eradicated { return true; },
            Color::Red => if self.red_disease == DiseaseState::Eradicated { return true; },
        }
        let count = &mut match city_obj.color {
            Color::Blue => city_obj.blue_infection_count,
            Color::Yellow => city_obj.yellow_infection_count,
            Color::Black => city_obj.black_infection_count,
            Color::Red => city_obj.red_infection_count
        };
        if self.total_cubes(&city_obj.color) + 1 > MAX_INFECTION_PER_TYPE {
            false
        }
        else if *count + 1 > MAX_INFECTION_PER_TYPE_PER_CITY {
            self.outbreak_city(city)
        }
        else {
            *count += 1;
            true
        }
    }

    pub fn infect_city_color(&mut self, city: &Cities, color: &Color) -> bool {
        match color {
            Color::Blue => if self.blue_disease == DiseaseState::Eradicated { return true; },
            Color::Yellow => if self.yellow_disease == DiseaseState::Eradicated { return true; },
            Color::Black => if self.black_disease == DiseaseState::Eradicated { return true; },
            Color::Red => if self.red_disease == DiseaseState::Eradicated { return true; },
        }
        let city_obj = self.map.get(city).unwrap();
        let count = &mut match color {
            Color::Blue => city_obj.blue_infection_count,
            Color::Yellow => city_obj.yellow_infection_count,
            Color::Black => city_obj.black_infection_count,
            Color::Red => city_obj.red_infection_count
        };
        if self.total_cubes(color) + 1 > MAX_INFECTION_PER_TYPE {
            false
        }
        else if *count + 1 > MAX_INFECTION_PER_TYPE_PER_CITY {
            self.outbreak_city_color(city, color)
        }
        else {
            *count += 1;
            true
        }
    }

    pub fn outbreak_city(&mut self, city: &Cities) -> bool {
        let mut city = self.map.get_mut(city).unwrap();
        match &city.color {
            Color::Blue => if self.blue_disease == DiseaseState::Eradicated { return true; },
            Color::Yellow => if self.yellow_disease == DiseaseState::Eradicated { return true; },
            Color::Black => if self.black_disease == DiseaseState::Eradicated { return true; },
            Color::Red => if self.red_disease == DiseaseState::Eradicated { return true; },
        }
        if city.current_outbreak { return true; }
        println!("{} had an outbreak!", &city.city);
        city.current_outbreak = true;
        let city_color = city.color.clone();
        let adjacent_cities = city.adjacent_cities.clone();
        self.outbreaks += 1;
        if self.outbreaks > MAX_OUTBREAKS {
            false
        } else {
            let mut game_continue = true;
            for adjacent_city in &adjacent_cities {
                game_continue = self.infect_city_color(adjacent_city, &city_color) && game_continue;
            }
            game_continue
        }
    }

    pub fn outbreak_city_color(&mut self, city: &Cities, color: &Color) -> bool {
        match color {
            Color::Blue => if self.blue_disease == DiseaseState::Eradicated { return true; },
            Color::Yellow => if self.yellow_disease == DiseaseState::Eradicated { return true; },
            Color::Black => if self.black_disease == DiseaseState::Eradicated { return true; },
            Color::Red => if self.red_disease == DiseaseState::Eradicated { return true; },
        }
        let mut city = self.map.get_mut(city).unwrap();
        if city.current_outbreak { return true; }
        println!("{} had an outbreak of {}!", &city.city, color);
        city.current_outbreak = true;
        let adjacent_cities = city.adjacent_cities.clone();
        self.outbreaks += 1;
        if self.outbreaks > MAX_OUTBREAKS {
            false
        } else {
            let mut game_continue = true;
            for adjacent_city in &adjacent_cities {
                game_continue = self.infect_city_color(adjacent_city, &color) && game_continue;
            }
            game_continue
        }
    }

    pub fn increase_outbreaks(&mut self) -> bool {
        self.outbreaks += 1;
        self.outbreaks < MAX_OUTBREAKS
    }

    pub fn increase_infection_rate(&mut self) -> bool {
        self.infection_rate += 1;
        self.infection_rate < INFECTION_RATE.len()
    }

    pub fn infection_rate(&self) -> u8 {
        INFECTION_RATE[self.infection_rate]
    }
}

fn make_cities() -> Vec<City> {
    let mut cities = Vec::new();

    cities.append(&mut make_blue_cities());
    cities.append(&mut make_yellow_cities());
    cities.append(&mut make_black_cities());
    cities.append(&mut make_red_cities());

    cities
}

fn make_blue_cities() -> Vec<City> {
    vec![
        City::new_with_research_station(
            Cities::Atlanta,
            Color::Blue,
            HashSet::from([Cities::Chicago, Cities::Miami, Cities::Washington]),
        ),
        City::new(
            Cities::Chicago,
            Color::Blue,
            HashSet::from([
                Cities::Atlanta,
                Cities::LosAngeles,
                Cities::MexicoCity,
                Cities::Montreal,
                Cities::SanFrancisco,
            ]),
        ),
        City::new(
            Cities::Essen,
            Color::Blue,
            HashSet::from([
                Cities::London,
                Cities::Milan,
                Cities::Paris,
                Cities::SaintPetersburg,
            ]),
        ),
        City::new(
            Cities::London,
            Color::Blue,
            HashSet::from([
                Cities::Essen,
                Cities::Madrid,
                Cities::NewYork,
                Cities::Paris,
            ]),
        ),
        City::new(
            Cities::Madrid,
            Color::Blue,
            HashSet::from([
                Cities::Algiers,
                Cities::London,
                Cities::NewYork,
                Cities::Paris,
                Cities::SaoPaulo,
            ]),
        ),
        City::new(
            Cities::Milan,
            Color::Blue,
            HashSet::from([Cities::Essen, Cities::Istanbul, Cities::Paris]),
        ),
        City::new(
            Cities::Montreal,
            Color::Blue,
            HashSet::from([Cities::Chicago, Cities::NewYork, Cities::Washington]),
        ),
        City::new(
            Cities::NewYork,
            Color::Blue,
            HashSet::from([
                Cities::London,
                Cities::Madrid,
                Cities::Montreal,
                Cities::Washington,
            ]),
        ),
        City::new(
            Cities::Paris,
            Color::Blue,
            HashSet::from([
                Cities::Algiers,
                Cities::Essen,
                Cities::London,
                Cities::Madrid,
                Cities::Milan,
            ]),
        ),
        City::new(
            Cities::SaintPetersburg,
            Color::Blue,
            HashSet::from([Cities::Essen, Cities::Istanbul, Cities::Moscow]),
        ),
        City::new(
            Cities::SanFrancisco,
            Color::Blue,
            HashSet::from([
                Cities::Chicago,
                Cities::LosAngeles,
                Cities::Manila,
                Cities::Tokyo,
            ]),
        ),
        City::new(
            Cities::Washington,
            Color::Blue,
            HashSet::from([
                Cities::Atlanta,
                Cities::Miami,
                Cities::Montreal,
                Cities::NewYork,
            ]),
        ),
    ]
}

fn make_yellow_cities() -> Vec<City> {
    vec![
        City::new(
            Cities::Bogota,
            Color::Yellow,
            HashSet::from([
                Cities::BuenosAires,
                Cities::Lima,
                Cities::MexicoCity,
                Cities::Miami,
                Cities::SaoPaulo,
            ]),
        ),
        City::new(
            Cities::BuenosAires,
            Color::Yellow,
            HashSet::from([Cities::Bogota, Cities::SaoPaulo]),
        ),
        City::new(
            Cities::Johannesburg,
            Color::Yellow,
            HashSet::from([Cities::Kinshasa, Cities::Khartoum]),
        ),
        City::new(
            Cities::Kinshasa,
            Color::Yellow,
            HashSet::from([Cities::Johannesburg, Cities::Khartoum, Cities::Lagos]),
        ),
        City::new(
            Cities::Khartoum,
            Color::Yellow,
            HashSet::from([
                Cities::Cairo,
                Cities::Johannesburg,
                Cities::Kinshasa,
                Cities::Lagos,
            ]),
        ),
        City::new(
            Cities::Lagos,
            Color::Yellow,
            HashSet::from([Cities::Khartoum, Cities::Kinshasa, Cities::SaoPaulo]),
        ),
        City::new(
            Cities::Lima,
            Color::Yellow,
            HashSet::from([Cities::Bogota, Cities::MexicoCity, Cities::Santiago]),
        ),
        City::new(
            Cities::LosAngeles,
            Color::Yellow,
            HashSet::from([
                Cities::Chicago,
                Cities::MexicoCity,
                Cities::SanFrancisco,
                Cities::Sydney,
            ]),
        ),
        City::new(
            Cities::MexicoCity,
            Color::Yellow,
            HashSet::from([
                Cities::Bogota,
                Cities::Chicago,
                Cities::Lima,
                Cities::LosAngeles,
                Cities::Miami,
            ]),
        ),
        City::new(
            Cities::Miami,
            Color::Yellow,
            HashSet::from([
                Cities::Atlanta,
                Cities::Bogota,
                Cities::MexicoCity,
                Cities::Washington,
            ]),
        ),
        City::new(
            Cities::Santiago,
            Color::Yellow,
            HashSet::from([Cities::Lima]),
        ),
        City::new(
            Cities::SaoPaulo,
            Color::Yellow,
            HashSet::from([
                Cities::Bogota,
                Cities::BuenosAires,
                Cities::Lagos,
                Cities::Madrid,
            ]),
        ),
    ]
}

fn make_black_cities() -> Vec<City> {
    vec![
        City::new(
            Cities::Algiers,
            Color::Black,
            HashSet::from([
                Cities::Cairo,
                Cities::Istanbul,
                Cities::Madrid,
                Cities::Paris,
            ]),
        ),
        City::new(
            Cities::Baghdad,
            Color::Black,
            HashSet::from([
                Cities::Cairo,
                Cities::Istanbul,
                Cities::Karachi,
                Cities::Riyadh,
                Cities::Tehran,
            ]),
        ),
        City::new(
            Cities::Cairo,
            Color::Black,
            HashSet::from([
                Cities::Algiers,
                Cities::Baghdad,
                Cities::Istanbul,
                Cities::Khartoum,
                Cities::Riyadh,
            ]),
        ),
        City::new(
            Cities::Chennai,
            Color::Black,
            HashSet::from([
                Cities::Bangkok,
                Cities::Delhi,
                Cities::Jakarta,
                Cities::Kolkata,
                Cities::Mumbai,
            ]),
        ),
        City::new(
            Cities::Delhi,
            Color::Black,
            HashSet::from([
                Cities::Chennai,
                Cities::Karachi,
                Cities::Kolkata,
                Cities::Mumbai,
                Cities::Tehran,
            ]),
        ),
        City::new(
            Cities::Istanbul,
            Color::Black,
            HashSet::from([
                Cities::Algiers,
                Cities::Baghdad,
                Cities::Cairo,
                Cities::Milan,
                Cities::Moscow,
                Cities::SaintPetersburg,
            ]),
        ),
        City::new(
            Cities::Karachi,
            Color::Black,
            HashSet::from([
                Cities::Baghdad,
                Cities::Delhi,
                Cities::Mumbai,
                Cities::Riyadh,
                Cities::Tehran,
            ]),
        ),
        City::new(
            Cities::Kolkata,
            Color::Black,
            HashSet::from([
                Cities::Bangkok,
                Cities::Chennai,
                Cities::Delhi,
                Cities::HongKong,
            ]),
        ),
        City::new(
            Cities::Mumbai,
            Color::Black,
            HashSet::from([Cities::Chennai, Cities::Delhi, Cities::Karachi]),
        ),
        City::new(
            Cities::Moscow,
            Color::Black,
            HashSet::from([Cities::Istanbul, Cities::SaintPetersburg, Cities::Tehran]),
        ),
        City::new(
            Cities::Riyadh,
            Color::Black,
            HashSet::from([Cities::Baghdad, Cities::Cairo, Cities::Karachi]),
        ),
        City::new(
            Cities::Tehran,
            Color::Black,
            HashSet::from([
                Cities::Baghdad,
                Cities::Delhi,
                Cities::Karachi,
                Cities::Moscow,
            ]),
        ),
    ]
}

fn make_red_cities() -> Vec<City> {
    vec![
        City::new(
            Cities::Bangkok,
            Color::Red,
            HashSet::from([
                Cities::Chennai,
                Cities::HoChiMinhCity,
                Cities::HongKong,
                Cities::Jakarta,
                Cities::Kolkata,
            ]),
        ),
        City::new(
            Cities::Beijing,
            Color::Red,
            HashSet::from([Cities::Seoul, Cities::Shanghai]),
        ),
        City::new(
            Cities::HoChiMinhCity,
            Color::Red,
            HashSet::from([
                Cities::Bangkok,
                Cities::HongKong,
                Cities::Jakarta,
                Cities::Manila,
            ]),
        ),
        City::new(
            Cities::HongKong,
            Color::Red,
            HashSet::from([
                Cities::Bangkok,
                Cities::HoChiMinhCity,
                Cities::Kolkata,
                Cities::Manila,
                Cities::Shanghai,
                Cities::Taipei,
            ]),
        ),
        City::new(
            Cities::Jakarta,
            Color::Red,
            HashSet::from([
                Cities::Bangkok,
                Cities::Chennai,
                Cities::HoChiMinhCity,
                Cities::Sydney,
            ]),
        ),
        City::new(
            Cities::Manila,
            Color::Red,
            HashSet::from([
                Cities::HoChiMinhCity,
                Cities::HongKong,
                Cities::SanFrancisco,
                Cities::Sydney,
                Cities::Taipei,
            ]),
        ),
        City::new(
            Cities::Osaka,
            Color::Red,
            HashSet::from([Cities::Taipei, Cities::Tokyo]),
        ),
        City::new(
            Cities::Seoul,
            Color::Red,
            HashSet::from([Cities::Beijing, Cities::Shanghai, Cities::Tokyo]),
        ),
        City::new(
            Cities::Shanghai,
            Color::Red,
            HashSet::from([
                Cities::Beijing,
                Cities::HongKong,
                Cities::Seoul,
                Cities::Taipei,
                Cities::Tokyo,
            ]),
        ),
        City::new(
            Cities::Sydney,
            Color::Red,
            HashSet::from([Cities::Jakarta, Cities::LosAngeles, Cities::Manila]),
        ),
        City::new(
            Cities::Taipei,
            Color::Red,
            HashSet::from([
                Cities::HongKong,
                Cities::Manila,
                Cities::Osaka,
                Cities::Shanghai,
            ]),
        ),
        City::new(
            Cities::Tokyo,
            Color::Red,
            HashSet::from([
                Cities::Osaka,
                Cities::SanFrancisco,
                Cities::Seoul,
                Cities::Shanghai,
            ]),
        ),
    ]
}

impl std::fmt::Display for Cities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Algiers => write!(f, "Algiers ({})", Color::Black),
            Self::Atlanta => write!(f, "Atlanta ({})", Color::Blue),
            Self::Baghdad => write!(f, "Baghdad ({})", Color::Black),
            Self::Bangkok => write!(f, "Bangkok ({})", Color::Red),
            Self::Beijing => write!(f, "Beijing ({})", Color::Red),
            Self::Bogota => write!(f, "Bogotá ({})", Color::Yellow),
            Self::BuenosAires => write!(f, "Buenos Aires ({})", Color::Yellow),
            Self::Cairo => write!(f, "Cairo ({})", Color::Black),
            Self::Chennai => write!(f, "Chennai ({})", Color::Black),
            Self::Chicago => write!(f, "Chicago ({})", Color::Blue),
            Self::Delhi => write!(f, "Delhi ({})", Color::Black),
            Self::Essen => write!(f, "Essen ({})", Color::Blue),
            Self::HoChiMinhCity => write!(f, "Ho Chi Minh City ({})", Color::Red),
            Self::HongKong => write!(f, "Hong Kong ({})", Color::Red),
            Self::Istanbul => write!(f, "Istanbul ({})", Color::Black),
            Self::Jakarta => write!(f, "Jakarta ({})", Color::Red),
            Self::Johannesburg => write!(f, "Johannesburg ({})", Color::Yellow),
            Self::Karachi => write!(f, "Karachi ({})", Color::Black),
            Self::Khartoum => write!(f, "Khartoum ({})", Color::Yellow),
            Self::Kinshasa => write!(f, "Kinshasa ({})", Color::Yellow),
            Self::Kolkata => write!(f, "Kolkata ({})", Color::Black),
            Self::Lagos => write!(f, "Lagos ({})", Color::Yellow),
            Self::Lima => write!(f, "Lima ({})", Color::Yellow),
            Self::London => write!(f, "London ({})", Color::Blue),
            Self::LosAngeles => write!(f, "Los Angeles ({})", Color::Yellow),
            Self::Madrid => write!(f, "Madrid ({})", Color::Blue),
            Self::Manila => write!(f, "Manila ({})", Color::Red),
            Self::MexicoCity => write!(f, "Mexico City ({})", Color::Yellow),
            Self::Miami => write!(f, "Miami ({})", Color::Yellow),
            Self::Milan => write!(f, "Milan ({})", Color::Blue),
            Self::Montreal => write!(f, "Montréal ({})", Color::Blue),
            Self::Moscow => write!(f, "Moscow ({})", Color::Black),
            Self::Mumbai => write!(f, "Mumbai ({})", Color::Black),
            Self::NewYork => write!(f, "New York ({})", Color::Blue),
            Self::Osaka => write!(f, "Osaka ({})", Color::Red),
            Self::Paris => write!(f, "Paris ({})", Color::Blue),
            Self::Riyadh => write!(f, "Riyadh ({})", Color::Black),
            Self::SaintPetersburg => write!(f, "Saint Petersburg ({})", Color::Blue),
            Self::SanFrancisco => write!(f, "San Francisco ({})", Color::Blue),
            Self::Santiago => write!(f, "Santiago ({})", Color::Yellow),
            Self::SaoPaulo => write!(f, "São Paulo ({})", Color::Yellow),
            Self::Seoul => write!(f, "Seoul ({})", Color::Red),
            Self::Shanghai => write!(f, "Shanghai ({})", Color::Red),
            Self::Sydney => write!(f, "Sydney ({})", Color::Red),
            Self::Taipei => write!(f, "Taipei ({})", Color::Red),
            Self::Tehran => write!(f, "Tehran ({})", Color::Black),
            Self::Tokyo => write!(f, "Tokyo ({})", Color::Red),
            Self::Washington => write!(f, "Washington ({})", Color::Blue),
        }
    }
}
