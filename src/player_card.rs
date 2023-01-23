use std::collections::VecDeque;

use rand::seq::SliceRandom;

use crate::{common::Color, deck::Deck, board::Cities, player::Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct City {
    pub(crate) city: Cities,
    color: Color,
    country: &'static str,
    flag: &'static str,
    pub(crate) population: u32,
    population_density: u16,
}

impl PartialOrd for City {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.color.partial_cmp(&other.color) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.city.partial_cmp(&other.city) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.population.partial_cmp(&other.population) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.country.partial_cmp(&other.country) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.flag.partial_cmp(&other.flag) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.population_density.partial_cmp(&other.population_density)
    }
}

impl Ord for City {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.color.partial_cmp(&other.color) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(ord) => return ord,
            None => {}
        }
        self.city.cmp(&other.city)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Events {
    Airlift,
    Forecast,
    GovernmentGrant,
    OneQuietNight,
    ResilientPopulation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    event: Events,
    description: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerCard {
    CityCard(City),
    EpidemicCard,
    EventCard(Event),
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.event, self.description)
    }
}

impl std::fmt::Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} | {} {} | 👥 {} | 👤/◼️ {}/km²",
            self.city, self.flag, self.country, self.population, self.population_density
        )
    }
}

impl std::fmt::Display for PlayerCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CityCard(city) => write!(f, "City: {}", city),
            Self::EpidemicCard => write!(f, "{}", "Epidemic!"),
            Self::EventCard(event) => write!(f, "Event: {}", event),
        }
    }
}

impl Deck<PlayerCard> {
    pub fn new() -> Self {
        Self(VecDeque::from(make_deck()))
    }

    pub fn add_epidemic_cards(&mut self, epidemics: u8) {
        let chunk_size = (self.0.len() as f64 / epidemics as f64).round() as usize;
        let mut old_deck = Vec::from(self.0.clone());
        let mut new_deck = VecDeque::new();
        let mut count = 0;
        for chunk in old_deck.chunks_mut(chunk_size) {
            let mut chunk = chunk.to_vec();
            if count < epidemics {
                chunk.push(PlayerCard::EpidemicCard);
            }
            count += 1;
            println!("Chunk {} :: {:?}", count, chunk);
            chunk.shuffle(&mut rand::thread_rng());
            new_deck.append(&mut VecDeque::from(chunk));
        }
        self.0 = new_deck;
    }

    pub fn deal(&mut self, players: &mut [Player]) {
        for player in players {
            match self.draw_from_top() {
                Some(card) => player.add_to_hand(card),
                None => (),
            }
        }
    }
}

fn make_deck() -> Vec<PlayerCard> {
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
            city: Cities::Atlanta,
            color: Color::Blue,
            country: "United States",
            flag: "🇺🇸",
            population: 4_715_000,
            population_density: 700,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Chicago,
            color: Color::Blue,
            country: "United States",
            flag: "🇺🇸",
            population: 9_121_000,
            population_density: 1_300,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Essen,
            color: Color::Blue,
            country: "Germany",
            flag: "🇩🇪",
            population: 575_000,
            population_density: 2_800,
        }),
        PlayerCard::CityCard(City {
            city: Cities::London,
            color: Color::Blue,
            country: "United Kingdom",
            flag: "🇬🇧",
            population: 8_586_000,
            population_density: 5_300,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Madrid,
            color: Color::Blue,
            country: "Spain",
            flag: "🇪🇸",
            population: 5_427_000,
            population_density: 5_700,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Milan,
            color: Color::Blue,
            country: "Italy",
            flag: "🇮🇹",
            population: 5_232_000,
            population_density: 2_800,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Montreal,
            color: Color::Blue,
            country: "Canada",
            flag: "🇨🇦",
            population: 3_429_000,
            population_density: 2_200,
        }),
        PlayerCard::CityCard(City {
            city: Cities::NewYork,
            color: Color::Blue,
            country: "United States",
            flag: "🇺🇸",
            population: 20_464_000,
            population_density: 1_800,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Paris,
            color: Color::Blue,
            country: "France",
            flag: "🇫🇷",
            population: 10_755_000,
            population_density: 3_800,
        }),
        PlayerCard::CityCard(City {
            city: Cities::SaintPetersburg,
            color: Color::Blue,
            country: "Russia",
            flag: "🇷🇺",
            population: 4_879_000,
            population_density: 4_100,
        }),
        PlayerCard::CityCard(City {
            city: Cities::SanFrancisco,
            color: Color::Blue,
            country: "United States",
            flag: "🇺🇸",
            population: 5_864_000,
            population_density: 2_100,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Washington,
            color: Color::Blue,
            country: "United States",
            flag: "🇺🇸",
            population: 4_679_000,
            population_density: 2_100,
        }),
    ]
}

fn make_yellow_cities() -> Vec<PlayerCard> {
    vec![
        PlayerCard::CityCard(City {
            city: Cities::Bogota,
            color: Color::Yellow,
            country: "Colombia",
            flag: "🇨🇴",
            population: 8_702_000,
            population_density: 21_000,
        }),
        PlayerCard::CityCard(City {
            city: Cities::BuenosAires,
            color: Color::Yellow,
            country: "Argentina",
            flag: "🇦🇷",
            population: 13_639_000,
            population_density: 5_200,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Johannesburg,
            color: Color::Yellow,
            country: "South Africa",
            flag: "🇿🇦",
            population: 3_888_000,
            population_density: 2_400,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Kinshasa,
            color: Color::Yellow,
            country: "Democratic Republic of the Congo",
            flag: "🇨🇩",
            population: 9_046_000,
            population_density: 15_500,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Khartoum,
            color: Color::Yellow,
            country: "Sudan",
            flag: "🇸🇩",
            population: 4_887_000,
            population_density: 4_500,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Lagos,
            color: Color::Yellow,
            country: "Nigeria",
            flag: "🇳🇬",
            population: 11_547_000,
            population_density: 12_700,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Lima,
            color: Color::Yellow,
            country: "Peru",
            flag: "🇵🇪",
            population: 9_121_000,
            population_density: 14_100,
        }),
        PlayerCard::CityCard(City {
            city: Cities::LosAngeles,
            color: Color::Yellow,
            country: "United States",
            flag: "🇺🇸",
            population: 14_900_000,
            population_density: 2_400,
        }),
        PlayerCard::CityCard(City {
            city: Cities::MexicoCity,
            color: Color::Yellow,
            country: "Mexico",
            flag: "🇲🇽",
            population: 19_463_000,
            population_density: 9_500,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Miami,
            color: Color::Yellow,
            country: "United States",
            flag: "🇺🇸",
            population: 5_582_000,
            population_density: 1_700,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Santiago,
            color: Color::Yellow,
            country: "Chile",
            flag: "🇨🇱",
            population: 6_015_000,
            population_density: 6_500,
        }),
        PlayerCard::CityCard(City {
            city: Cities::SaoPaulo,
            color: Color::Yellow,
            country: "Brazil",
            flag: "🇧🇷",
            population: 20_186_000,
            population_density: 6_400,
        }),
    ]
}

fn make_black_cities() -> Vec<PlayerCard> {
    vec![
        PlayerCard::CityCard(City {
            city: Cities::Algiers,
            color: Color::Black,
            country: "Algeria",
            flag: "🇩🇿",
            population: 2_946_000,
            population_density: 6_500,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Baghdad,
            color: Color::Black,
            country: "Iraq",
            flag: "🇮🇶",
            population: 6_204_000,
            population_density: 10_400,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Cairo,
            color: Color::Black,
            country: "Egypt",
            flag: "🇪🇬",
            population: 14_718_000,
            population_density: 8_900,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Chennai,
            color: Color::Black,
            country: "India",
            flag: "🇮🇳",
            population: 8_865_000,
            population_density: 14_600,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Delhi,
            color: Color::Black,
            country: "India",
            flag: "🇮🇳",
            population: 22_242_000,
            population_density: 11_500,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Istanbul,
            color: Color::Black,
            country: "Turkey",
            flag: "🇹🇷",
            population: 13_576_000,
            population_density: 9_700,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Karachi,
            color: Color::Black,
            country: "Pakistan",
            flag: "🇵🇰",
            population: 20_711_000,
            population_density: 25_800,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Kolkata,
            color: Color::Black,
            country: "India",
            flag: "🇮🇳",
            population: 14_374_000,
            population_density: 11_900,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Mumbai,
            color: Color::Black,
            country: "India",
            flag: "🇮🇳",
            population: 16_910_000,
            population_density: 30_900,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Moscow,
            color: Color::Black,
            country: "Russia",
            flag: "🇷🇺",
            population: 15_512_000,
            population_density: 3_500,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Riyadh,
            color: Color::Black,
            country: "Saudi Arabia",
            flag: "🇸🇦",
            population: 5_037_000,
            population_density: 3_400,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Tehran,
            color: Color::Black,
            country: "Iran",
            flag: "🇮🇷",
            population: 7_419_000,
            population_density: 9_500,
        }),
    ]
}

fn make_red_cities() -> Vec<PlayerCard> {
    vec![
        PlayerCard::CityCard(City {
            city: Cities::Bangkok,
            color: Color::Red,
            country: "Thailand",
            flag: "🇹🇭",
            population: 7_151_000,
            population_density: 3_200,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Beijing,
            color: Color::Red,
            country: "People's Republic of China",
            flag: "🇨🇳",
            population: 17_311_000,
            population_density: 5_000,
        }),
        PlayerCard::CityCard(City {
            city: Cities::HoChiMinhCity,
            color: Color::Red,
            country: "Vietnam",
            flag: "🇻🇳",
            population: 8_314_000,
            population_density: 9_900,
        }),
        PlayerCard::CityCard(City {
            city: Cities::HongKong,
            color: Color::Red,
            country: "Special Administrative Region of the People's Republic of China",
            flag: "🇭🇰",
            population: 7_106_000,
            population_density: 25_900,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Jakarta,
            color: Color::Red,
            country: "Indonesia",
            flag: "🇮🇩",
            population: 26_063_000,
            population_density: 9_400,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Manila,
            color: Color::Red,
            country: "Philippines",
            flag: "🇵🇭",
            population: 20_767_000,
            population_density: 14_400,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Osaka,
            color: Color::Red,
            country: "Japan",
            flag: "🇯🇵",
            population: 2_871_000,
            population_density: 13_000,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Seoul,
            color: Color::Red,
            country: "South Korea",
            flag: "🇰🇷",
            population: 22_547_000,
            population_density: 10_400,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Shanghai,
            color: Color::Red,
            country: "People's Republic of China",
            flag: "🇨🇳",
            population: 13_482_000,
            population_density: 2_200,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Sydney,
            color: Color::Red,
            country: "Australia",
            flag: "🇦🇺",
            population: 3_785_000,
            population_density: 2_100,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Taipei,
            color: Color::Red,
            country: "Taiwan",
            flag: "🇹🇼",
            population: 13_482_000,
            population_density: 2_200,
        }),
        PlayerCard::CityCard(City {
            city: Cities::Tokyo,
            color: Color::Red,
            country: "Japan",
            flag: "🇯🇵",
            population: 13_189_000,
            population_density: 6_030,
        }),
    ]
}

fn make_events() -> Vec<PlayerCard> {
    vec![
            PlayerCard::EventCard(Event {
                event: Events::Airlift,
                description: "Move any 1 pawn to any city. Get permission before moving another player's pawn."
            }), PlayerCard::EventCard(Event {
                event: Events::Forecast,
                description: "Draw, look at, and rearrange the top 6 cards of the Infection Deck. Put them back on top."
            }), PlayerCard::EventCard(Event {
                event: Events::GovernmentGrant,
                description: "Add 1 research station to any city (no City card needed)."
            }), PlayerCard::EventCard(Event {
                event: Events::OneQuietNight,
                description: "Skip the next Infect Cities step (do not flip over any Infection cards)."
            }), PlayerCard::EventCard(Event {
                event: Events::ResilientPopulation,
                description: "Remove any 1 card in the Infection Discard Pile from the game. You may play this between the Infect and Intensify steps of an epidemic."
            })
        ]
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Airlift => write!(f, "Airlift"),
            Self::Forecast => write!(f, "Forecast"),
            Self::GovernmentGrant => write!(f, "Government Grant"),
            Self::OneQuietNight => write!(f, "One Quiet Night"),
            Self::ResilientPopulation => write!(f, "Resilient Population"),
        }
    }
}
