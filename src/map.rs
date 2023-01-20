pub mod map {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use crate::common::common::Color;

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
        city: Cities,
        color: Color,
        adjacent_cities: HashSet<Cities>,
    }

    pub struct Map(HashMap<Cities, City>);

    impl Map {
        pub fn new() -> Self {
            let mut cities = HashMap::new();

            for city in make_cities() {
                cities.insert(city.city, city);
            }

            Self(cities)
        }

        pub fn adjacent_to(&self, city: &Cities) -> Option<HashSet<Cities>> {
            match self.0.get(&city) {
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
            for key in self.0.keys() {
                cities.push(key.clone());
            }
            cities
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
            City {
                city: Cities::Atlanta,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Chicago,
                    Cities::Miami,
                    Cities::Washington,
                ]),
            },
            City {
                city: Cities::Chicago,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Atlanta,
                    Cities::LosAngeles,
                    Cities::MexicoCity,
                    Cities::Montreal,
                    Cities::SanFrancisco,
                ]),
            },
            City {
                city: Cities::Essen,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::London,
                    Cities::Milan,
                    Cities::Paris,
                    Cities::SaintPetersburg,
                ]),
            },
            City {
                city: Cities::London,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Essen,
                    Cities::Madrid,
                    Cities::NewYork,
                    Cities::Paris,
                ]),
            },
            City {
                city: Cities::Madrid,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Algiers,
                    Cities::London,
                    Cities::NewYork,
                    Cities::Paris,
                    Cities::SaoPaulo,
                ]),
            },
            City {
                city: Cities::Milan,
                color: Color::Blue,
                adjacent_cities: HashSet::from([Cities::Essen, Cities::Istanbul, Cities::Paris]),
            },
            City {
                city: Cities::Montreal,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Chicago,
                    Cities::NewYork,
                    Cities::Washington,
                ]),
            },
            City {
                city: Cities::NewYork,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::London,
                    Cities::Madrid,
                    Cities::Montreal,
                    Cities::Washington,
                ]),
            },
            City {
                city: Cities::Paris,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Algiers,
                    Cities::Essen,
                    Cities::London,
                    Cities::Madrid,
                    Cities::Milan,
                ]),
            },
            City {
                city: Cities::SaintPetersburg,
                color: Color::Blue,
                adjacent_cities: HashSet::from([Cities::Essen, Cities::Istanbul, Cities::Moscow]),
            },
            City {
                city: Cities::SanFrancisco,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Chicago,
                    Cities::LosAngeles,
                    Cities::Manila,
                    Cities::Tokyo,
                ]),
            },
            City {
                city: Cities::Washington,
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    Cities::Atlanta,
                    Cities::Miami,
                    Cities::Montreal,
                    Cities::NewYork,
                ]),
            },
        ]
    }

    fn make_yellow_cities() -> Vec<City> {
        vec![
            City {
                city: Cities::Bogota,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::BuenosAires,
                    Cities::Lima,
                    Cities::MexicoCity,
                    Cities::Miami,
                    Cities::SaoPaulo,
                ]),
            },
            City {
                city: Cities::BuenosAires,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([Cities::Bogota, Cities::SaoPaulo]),
            },
            City {
                city: Cities::Johannesburg,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([Cities::Kinshasa, Cities::Khartoum]),
            },
            City {
                city: Cities::Kinshasa,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Johannesburg,
                    Cities::Khartoum,
                    Cities::Lagos,
                ]),
            },
            City {
                city: Cities::Khartoum,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Cairo,
                    Cities::Johannesburg,
                    Cities::Kinshasa,
                    Cities::Lagos,
                ]),
            },
            City {
                city: Cities::Lagos,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Khartoum,
                    Cities::Kinshasa,
                    Cities::SaoPaulo,
                ]),
            },
            City {
                city: Cities::Lima,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Bogota,
                    Cities::MexicoCity,
                    Cities::Santiago,
                ]),
            },
            City {
                city: Cities::LosAngeles,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Chicago,
                    Cities::MexicoCity,
                    Cities::SanFrancisco,
                    Cities::Sydney,
                ]),
            },
            City {
                city: Cities::MexicoCity,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Bogota,
                    Cities::Chicago,
                    Cities::Lima,
                    Cities::LosAngeles,
                    Cities::Miami,
                ]),
            },
            City {
                city: Cities::Miami,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Atlanta,
                    Cities::Bogota,
                    Cities::MexicoCity,
                    Cities::Washington,
                ]),
            },
            City {
                city: Cities::Santiago,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([Cities::Lima]),
            },
            City {
                city: Cities::SaoPaulo,
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    Cities::Bogota,
                    Cities::BuenosAires,
                    Cities::Lagos,
                    Cities::Madrid,
                ]),
            },
        ]
    }

    // TODO: Add Mumbai
    fn make_black_cities() -> Vec<City> {
        vec![
            City {
                city: Cities::Algiers,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Cairo,
                    Cities::Istanbul,
                    Cities::Madrid,
                    Cities::Paris,
                ]),
            },
            City {
                city: Cities::Baghdad,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Cairo,
                    Cities::Istanbul,
                    Cities::Karachi,
                    Cities::Riyadh,
                    Cities::Tehran,
                ]),
            },
            City {
                city: Cities::Cairo,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Algiers,
                    Cities::Baghdad,
                    Cities::Istanbul,
                    Cities::Khartoum,
                    Cities::Riyadh,
                ]),
            },
            City {
                city: Cities::Chennai,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Bangkok,
                    Cities::Delhi,
                    Cities::Jakarta,
                    Cities::Kolkata,
                    Cities::Mumbai,
                ]),
            },
            City {
                city: Cities::Delhi,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Chennai,
                    Cities::Karachi,
                    Cities::Kolkata,
                    Cities::Mumbai,
                    Cities::Tehran,
                ]),
            },
            City {
                city: Cities::Istanbul,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Algiers,
                    Cities::Baghdad,
                    Cities::Cairo,
                    Cities::Milan,
                    Cities::Moscow,
                    Cities::SaintPetersburg,
                ]),
            },
            City {
                city: Cities::Karachi,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Baghdad,
                    Cities::Delhi,
                    Cities::Mumbai,
                    Cities::Riyadh,
                    Cities::Tehran,
                ]),
            },
            City {
                city: Cities::Kolkata,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Bangkok,
                    Cities::Chennai,
                    Cities::Delhi,
                    Cities::HongKong,
                ]),
            },
            City {
                city: Cities::Moscow,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Istanbul,
                    Cities::SaintPetersburg,
                    Cities::Tehran,
                ]),
            },
            City {
                city: Cities::Riyadh,
                color: Color::Black,
                adjacent_cities: HashSet::from([Cities::Baghdad, Cities::Cairo, Cities::Karachi]),
            },
            City {
                city: Cities::Tehran,
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    Cities::Baghdad,
                    Cities::Delhi,
                    Cities::Karachi,
                    Cities::Moscow,
                ]),
            },
        ]
    }

    fn make_red_cities() -> Vec<City> {
        vec![
            City {
                city: Cities::Bangkok,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::Chennai,
                    Cities::HoChiMinhCity,
                    Cities::HongKong,
                    Cities::Jakarta,
                    Cities::Kolkata,
                ]),
            },
            City {
                city: Cities::Beijing,
                color: Color::Red,
                adjacent_cities: HashSet::from([Cities::Seoul, Cities::Shanghai]),
            },
            City {
                city: Cities::HoChiMinhCity,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::Bangkok,
                    Cities::HongKong,
                    Cities::Jakarta,
                    Cities::Manila,
                ]),
            },
            City {
                city: Cities::HongKong,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::Bangkok,
                    Cities::HoChiMinhCity,
                    Cities::Kolkata,
                    Cities::Manila,
                    Cities::Shanghai,
                    Cities::Taipei,
                ]),
            },
            City {
                city: Cities::Jakarta,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::Bangkok,
                    Cities::Chennai,
                    Cities::HoChiMinhCity,
                    Cities::Sydney,
                ]),
            },
            City {
                city: Cities::Manila,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::HoChiMinhCity,
                    Cities::HongKong,
                    Cities::SanFrancisco,
                    Cities::Sydney,
                    Cities::Taipei,
                ]),
            },
            City {
                city: Cities::Osaka,
                color: Color::Red,
                adjacent_cities: HashSet::from([Cities::Taipei, Cities::Tokyo]),
            },
            City {
                city: Cities::Seoul,
                color: Color::Red,
                adjacent_cities: HashSet::from([Cities::Beijing, Cities::Shanghai, Cities::Tokyo]),
            },
            City {
                city: Cities::Shanghai,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::Beijing,
                    Cities::HongKong,
                    Cities::Seoul,
                    Cities::Taipei,
                    Cities::Tokyo,
                ]),
            },
            City {
                city: Cities::Sydney,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::Jakarta,
                    Cities::LosAngeles,
                    Cities::Manila,
                ]),
            },
            City {
                city: Cities::Taipei,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::HongKong,
                    Cities::Manila,
                    Cities::Osaka,
                    Cities::Shanghai,
                ]),
            },
            City {
                city: Cities::Tokyo,
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    Cities::Osaka,
                    Cities::SanFrancisco,
                    Cities::Seoul,
                    Cities::Shanghai,
                ]),
            },
        ]
    }

    impl std::fmt::Display for Cities {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Algiers => write!(f, "Algiers"),
                Self::Atlanta => write!(f, "Atlanta"),
                Self::Baghdad => write!(f, "Baghdad"),
                Self::Bangkok => write!(f, "Bangkok"),
                Self::Beijing => write!(f, "Beijing"),
                Self::Bogota => write!(f, "Bogotá"),
                Self::BuenosAires => write!(f, "Buenos Aires"),
                Self::Cairo => write!(f, "Cairo"),
                Self::Chennai => write!(f, "Chennai"),
                Self::Chicago => write!(f, "Chicago"),
                Self::Delhi => write!(f, "Delhi"),
                Self::Essen => write!(f, "Essen"),
                Self::HoChiMinhCity => write!(f, "Ho Chi Minh City"),
                Self::HongKong => write!(f, "Hong Kong"),
                Self::Istanbul => write!(f, "Istanbul"),
                Self::Jakarta => write!(f, "Jakarta"),
                Self::Johannesburg => write!(f, "Johannesburg"),
                Self::Karachi => write!(f, "Karachi"),
                Self::Khartoum => write!(f, "Khartoum"),
                Self::Kinshasa => write!(f, "Kinshasa"),
                Self::Kolkata => write!(f, "Kolkata"),
                Self::Lagos => write!(f, "Lagos"),
                Self::Lima => write!(f, "Lima"),
                Self::London => write!(f, "London"),
                Self::LosAngeles => write!(f, "Los Angeles"),
                Self::Madrid => write!(f, "Madrid"),
                Self::Manila => write!(f, "Manila"),
                Self::MexicoCity => write!(f, "Mexico City"),
                Self::Miami => write!(f, "Miami"),
                Self::Milan => write!(f, "Milan"),
                Self::Montreal => write!(f, "Montréal"),
                Self::Moscow => write!(f, "Moscow"),
                Self::Mumbai => write!(f, "Mumbai"),
                Self::NewYork => write!(f, "New York"),
                Self::Osaka => write!(f, "Osaka"),
                Self::Paris => write!(f, "Paris"),
                Self::Riyadh => write!(f, "Riyadh"),
                Self::SaintPetersburg => write!(f, "Saint Petersburg"),
                Self::SanFrancisco => write!(f, "San Francisco"),
                Self::Santiago => write!(f, "Santiago"),
                Self::SaoPaulo => write!(f, "São Paulo"),
                Self::Seoul => write!(f, "Seoul"),
                Self::Shanghai => write!(f, "Shanghai"),
                Self::Sydney => write!(f, "Sydney"),
                Self::Taipei => write!(f, "Taipei"),
                Self::Tehran => write!(f, "Tehran"),
                Self::Tokyo => write!(f, "Tokyo"),
                Self::Washington => write!(f, "Washington"),
            }
        }
    }
}
