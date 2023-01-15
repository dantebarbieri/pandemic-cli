pub mod player_card {
    use std::collections::HashSet;

    use crate::common::common::Color;

    #[derive(Debug)]
    pub struct City {
        name: &'static str,
        color: Color,
        country: &'static str,
        flag: &'static str,
        population: u32,
        population_density: u16,
        adjacent_cities: HashSet<&'static str>,
    }

    #[derive(Debug)]
    pub struct Event {
        name: &'static str,
        event: &'static str,
    }

    #[derive(Debug)]
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
                population: 4_715_000,
                population_density: 700,
                adjacent_cities: HashSet::from(["Chicago", "Miami", "Washington"]),
            }),
            PlayerCard::CityCard(City {
                name: "Chicago",
                color: Color::Blue,
                country: "United States",
                flag: "🇺🇸",
                population: 9_121_000,
                population_density: 1_300,
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
                population: 575_000,
                population_density: 2_800,
                adjacent_cities: HashSet::from(["London", "Milan", "Paris", "Saint Petersburg"]),
            }),
            PlayerCard::CityCard(City {
                name: "London",
                color: Color::Blue,
                country: "United Kingdom",
                flag: "🇬🇧",
                population: 8_586_000,
                population_density: 5_300,
                adjacent_cities: HashSet::from(["Essen", "Madrid", "New York", "Paris"]),
            }),
            PlayerCard::CityCard(City {
                name: "Madrid",
                color: Color::Blue,
                country: "Spain",
                flag: "🇪🇸",
                population: 5_427_000,
                population_density: 5_700,
                adjacent_cities: HashSet::from([
                    "Algiers",
                    "London",
                    "New York",
                    "Paris",
                    "São Paulo",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Milan",
                color: Color::Blue,
                country: "Italy",
                flag: "🇮🇹",
                population: 5_232_000,
                population_density: 2_800,
                adjacent_cities: HashSet::from(["Essen", "Istanbul", "Paris"]),
            }),
            PlayerCard::CityCard(City {
                name: "Montréal",
                color: Color::Blue,
                country: "Canada",
                flag: "🇨🇦",
                population: 3_429_000,
                population_density: 2_200,
                adjacent_cities: HashSet::from(["Chicago", "New York", "Washington"]),
            }),
            PlayerCard::CityCard(City {
                name: "New York",
                color: Color::Blue,
                country: "United States",
                flag: "🇺🇸",
                population: 20_464_000,
                population_density: 1_800,
                adjacent_cities: HashSet::from(["London", "Madrid", "Montréal", "Washington"]),
            }),
            PlayerCard::CityCard(City {
                name: "Paris",
                color: Color::Blue,
                country: "France",
                flag: "🇫🇷",
                population: 10_755_000,
                population_density: 3_800,
                adjacent_cities: HashSet::from(["Algiers", "Essen", "London", "Madrid", "Milan"]),
            }),
            PlayerCard::CityCard(City {
                name: "Saint Petersburg",
                color: Color::Blue,
                country: "Russia",
                flag: "🇷🇺",
                population: 4_879_000,
                population_density: 4_100,
                adjacent_cities: HashSet::from(["Essen", "Istanbul", "Moscow"]),
            }),
            PlayerCard::CityCard(City {
                name: "San Francisco",
                color: Color::Blue,
                country: "United States",
                flag: "🇺🇸",
                population: 5_864_000,
                population_density: 2_100,
                adjacent_cities: HashSet::from(["Chicago", "Los Angeles", "Manila", "Tokyo"]),
            }),
        ]
    }

    fn make_yellow_cities() -> Vec<PlayerCard> {
        vec![
            PlayerCard::CityCard(City {
                name: "Bogotá",
                color: Color::Yellow,
                country: "Colombia",
                flag: "🇨🇴",
                population: 8_702_000,
                population_density: 21_000,
                adjacent_cities: HashSet::from([
                    "Buenos Aires",
                    "Lima",
                    "Mexico City",
                    "Miami",
                    "São Paulo",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Buenos Aires",
                color: Color::Yellow,
                country: "Argentina",
                flag: "🇦🇷",
                population: 13_639_000,
                population_density: 5_200,
                adjacent_cities: HashSet::from(["Bogotá", "São Paulo"]),
            }),
            PlayerCard::CityCard(City {
                name: "Johannesburg",
                color: Color::Yellow,
                country: "South Africa",
                flag: "🇿🇦",
                population: 3_888_000,
                population_density: 2_400,
                adjacent_cities: HashSet::from(["Kinshasa", "Khartoum"]),
            }),
            PlayerCard::CityCard(City {
                name: "Kinshasa",
                color: Color::Yellow,
                country: "Democratic Republic of the Congo",
                flag: "🇨🇩",
                population: 9_046_000,
                population_density: 15_500,
                adjacent_cities: HashSet::from(["Johannesburg", "Khartoum", "Lagos"]),
            }),
            PlayerCard::CityCard(City {
                name: "Khartoum",
                color: Color::Yellow,
                country: "Sudan",
                flag: "🇸🇩",
                population: 4_887_000,
                population_density: 4_500,
                adjacent_cities: HashSet::from(["Cairo", "Johannesburg", "Kinshasa", "Lagos"]),
            }),
            PlayerCard::CityCard(City {
                name: "Lagos",
                color: Color::Yellow,
                country: "Nigeria",
                flag: "🇳🇬",
                population: 11_547_000,
                population_density: 12_700,
                adjacent_cities: HashSet::from(["Khartoum", "Kinshasa", "São Paulo"]),
            }),
            PlayerCard::CityCard(City {
                name: "Lima",
                color: Color::Yellow,
                country: "Peru",
                flag: "🇵🇪",
                population: 9_121_000,
                population_density: 14_100,
                adjacent_cities: HashSet::from(["Bogotá", "Mexico City", "Santiago"]),
            }),
            PlayerCard::CityCard(City {
                name: "Los Angeles",
                color: Color::Yellow,
                country: "United States",
                flag: "🇺🇸",
                population: 14_900_000,
                population_density: 2_400,
                adjacent_cities: HashSet::from([
                    "Chicago",
                    "Mexico City",
                    "San Francisco",
                    "Sydney",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Mexico City",
                color: Color::Yellow,
                country: "Mexico",
                flag: "🇲🇽",
                population: 19_463_000,
                population_density: 9_500,
                adjacent_cities: HashSet::from([
                    "Bogotá",
                    "Chicago",
                    "Lima",
                    "Los Angeles",
                    "Miami",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Miami",
                color: Color::Yellow,
                country: "United States",
                flag: "🇺🇸",
                population: 5_582_000,
                population_density: 1_700,
                adjacent_cities: HashSet::from(["Atlanta", "Bogotá", "Mexico City", "Washington"]),
            }),
            PlayerCard::CityCard(City {
                name: "Santiago",
                color: Color::Yellow,
                country: "Chile",
                flag: "🇨🇱",
                population: 6_015_000,
                population_density: 6_500,
                adjacent_cities: HashSet::from(["Lima"]),
            }),
            PlayerCard::CityCard(City {
                name: "São Paulo",
                color: Color::Yellow,
                country: "Brazil",
                flag: "🇧🇷",
                population: 20_186_000,
                population_density: 6_400,
                adjacent_cities: HashSet::from(["Bogotá", "Buenos Aires", "Lagos", "Madrid"]),
            }),
        ]
    }

    fn make_black_cities() -> Vec<PlayerCard> {
        vec![
            PlayerCard::CityCard(City {
                name: "Algiers",
                color: Color::Black,
                country: "Algeria",
                flag: "🇩🇿",
                population: 2_946_000,
                population_density: 6_500,
                adjacent_cities: HashSet::from(["Cairo", "Istanbul", "Madrid", "Paris"]),
            }),
            PlayerCard::CityCard(City {
                name: "Baghdad",
                color: Color::Black,
                country: "Iraq",
                flag: "🇮🇶",
                population: 6_204_000,
                population_density: 10_400,
                adjacent_cities: HashSet::from([
                    "Cairo", "Istanbul", "Karachi", "Riyadh", "Tehran",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Cairo",
                color: Color::Black,
                country: "Egypt",
                flag: "🇪🇬",
                population: 14_718_000,
                population_density: 8_900,
                adjacent_cities: HashSet::from([
                    "Algiers", "Baghdad", "Istanbul", "Khartoum", "Riyadh",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Chennai",
                color: Color::Black,
                country: "India",
                flag: "🇮🇳",
                population: 8_865_000,
                population_density: 14_600,
                adjacent_cities: HashSet::from([
                    "Bangkok", "Delhi", "Jakarta", "Kolkata", "Mumbai",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Delhi",
                color: Color::Black,
                country: "India",
                flag: "🇮🇳",
                population: 22_242_000,
                population_density: 11_500,
                adjacent_cities: HashSet::from([
                    "Chennai", "Karachi", "Kolkata", "Mumbai", "Tehran",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Istanbul",
                color: Color::Black,
                country: "Turkey",
                flag: "🇹🇷",
                population: 13_576_000,
                population_density: 9_700,
                adjacent_cities: HashSet::from([
                    "Algiers",
                    "Baghdad",
                    "Cairo",
                    "Milan",
                    "Moscow",
                    "Saint Petersburg",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Karachi",
                color: Color::Black,
                country: "Pakistan",
                flag: "🇵🇰",
                population: 20_711_000,
                population_density: 25_800,
                adjacent_cities: HashSet::from(["Baghdad", "Delhi", "Mumbai", "Riyadh", "Tehran"]),
            }),
            PlayerCard::CityCard(City {
                name: "Kolkata",
                color: Color::Black,
                country: "India",
                flag: "🇮🇳",
                population: 14_374_000,
                population_density: 11_900,
                adjacent_cities: HashSet::from(["Bangkok", "Chennai", "Delhi", "Hong Kong"]),
            }),
            PlayerCard::CityCard(City {
                name: "Moscow",
                color: Color::Black,
                country: "Russia",
                flag: "🇷🇺",
                population: 15_512_000,
                population_density: 3_500,
                adjacent_cities: HashSet::from(["Istanbul", "Saint Petersburg", "Tehran"]),
            }),
            PlayerCard::CityCard(City {
                name: "Riyadh",
                color: Color::Black,
                country: "Saudi Arabia",
                flag: "🇸🇦",
                population: 5_037_000,
                population_density: 3_400,
                adjacent_cities: HashSet::from(["Baghdad", "Cairo", "Karachi"]),
            }),
            PlayerCard::CityCard(City {
                name: "Tehran",
                color: Color::Black,
                country: "Iran",
                flag: "🇮🇷",
                population: 7_419_000,
                population_density: 9_500,
                adjacent_cities: HashSet::from(["Baghdad", "Delhi", "Karachi", "Moscow"]),
            }),
        ]
    }

    fn make_red_cities() -> Vec<PlayerCard> {
        vec![
            PlayerCard::CityCard(City {
                name: "Bangkok",
                color: Color::Red,
                country: "Thailand",
                flag: "🇹🇭",
                population: 7_151_000,
                population_density: 3_200,
                adjacent_cities: HashSet::from([
                    "Chennai",
                    "Ho Chi Minh City",
                    "Hong Kong",
                    "Jakarta",
                    "Kolkata",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Beijing",
                color: Color::Red,
                country: "People's Republic of China",
                flag: "🇨🇳",
                population: 17_311_000,
                population_density: 5_000,
                adjacent_cities: HashSet::from(["Seoul", "Shanghai"]),
            }),
            PlayerCard::CityCard(City {
                name: "Ho Chi Minh City",
                color: Color::Red,
                country: "Vietnam",
                flag: "🇻🇳",
                population: 8_314_000,
                population_density: 9_900,
                adjacent_cities: HashSet::from(["Bangkok", "Hong Kong", "Jakarta", "Manila"]),
            }),
            PlayerCard::CityCard(City {
                name: "Hong Kong",
                color: Color::Red,
                country: "Special Administrative Region of the People's Republic of China",
                flag: "🇭🇰",
                population: 7_106_000,
                population_density: 25_900,
                adjacent_cities: HashSet::from([
                    "Bangkok",
                    "Ho Chi Minh City",
                    "Kolkata",
                    "Manila",
                    "Shanghai",
                    "Taipei",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Jakarta",
                color: Color::Red,
                country: "Indonesia",
                flag: "🇮🇩",
                population: 26_063_000,
                population_density: 9_400,
                adjacent_cities: HashSet::from([
                    "Bangkok",
                    "Chennai",
                    "Ho Chi Minh City",
                    "Sydney",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Manila",
                color: Color::Red,
                country: "Philippines",
                flag: "🇵🇭",
                population: 20_767_000,
                population_density: 14_400,
                adjacent_cities: HashSet::from([
                    "Ho Chi Minh City",
                    "Hong Kong",
                    "San Francisco",
                    "Sydney",
                    "Taipei",
                ]),
            }),
            PlayerCard::CityCard(City {
                name: "Osaka",
                color: Color::Red,
                country: "Japan",
                flag: "🇯🇵",
                population: 2_871_000,
                population_density: 13_000,
                adjacent_cities: HashSet::from(["Taipei", "Tokyo"]),
            }),
            PlayerCard::CityCard(City {
                name: "Seoul",
                color: Color::Red,
                country: "South Korea",
                flag: "🇰🇷",
                population: 22_547_000,
                population_density: 10_400,
                adjacent_cities: HashSet::from(["Beijing", "Shanghai", "Tokyo"]),
            }),
            PlayerCard::CityCard(City {
                name: "Shanghai",
                color: Color::Red,
                country: "People's Republic of China",
                flag: "🇨🇳",
                population: 13_482_000,
                population_density: 2_200,
                adjacent_cities: HashSet::from(["Beijing", "Hong Kong", "Seoul", "Taipei", "Tokyo"]),
            }),
            PlayerCard::CityCard(City {
                name: "Sydney",
                color: Color::Red,
                country: "Australia",
                flag: "🇦🇺",
                population: 3_785_000,
                population_density: 2_100,
                adjacent_cities: HashSet::from(["Jakarta", "Los Angeles", "Manila"]),
            }),
            PlayerCard::CityCard(City {
                name: "Taipei",
                color: Color::Red,
                country: "Taiwan",
                flag: "🇹🇼",
                population: 13_482_000,
                population_density: 2_200,
                adjacent_cities: HashSet::from(["Hong Kong", "Manila", "Osaka", "Shanghai"]),
            }),
            PlayerCard::CityCard(City {
                name: "Tokyo",
                color: Color::Red,
                country: "Japan",
                flag: "🇯🇵",
                population: 13_189_000,
                population_density: 6_030,
                adjacent_cities: HashSet::from(["Osaka", "San Francisco", "Seoul", "Shanghai"]),
            }),
        ]
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
