pub mod map {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use crate::common::common::Color;

    #[derive(Debug)]
    pub struct City {
        name: &'static str,
        color: Color,
        adjacent_cities: HashSet<&'static str>,
    }

    pub struct Map(HashMap<&'static str, City>);

    impl Map {
        pub fn new() -> Self {
            let mut cities = HashMap::new();

            for city in make_cities() {
                cities.insert(city.name, city);
            }

            Self(cities)
        }

        pub fn adjacent_to<'a>(&'a self, city: &'a str) -> Option<&HashSet<&'static str>> {
            match self.0.get(&city) {
                Some(city) => Some(&city.adjacent_cities),
                None => None
            }
        }

        pub fn is_adjacent(&self, from: &str, to: &str) -> bool {
            match self.adjacent_to(from) {
                Some(cities) => cities.contains(to),
                None => match self.adjacent_to(to) {
                    Some(cities) => cities.contains(from),
                    None => false
                }
            }
        }

        pub fn all_cities(&self) -> Vec<&str> {
            let mut cities = Vec::new();
            for key in self.0.keys() {
                cities.push(*key);
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
                name: "Atlanta",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["Chicago", "Miami", "Washington"]),
            },
            City {
                name: "Chicago",
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    "Atlanta",
                    "Los Angeles",
                    "Mexico City",
                    "Montréal",
                    "San Francisco",
                ]),
            },
            City {
                name: "Essen",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["London", "Milan", "Paris", "Saint Petersburg"]),
            },
            City {
                name: "London",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["Essen", "Madrid", "New York", "Paris"]),
            },
            City {
                name: "Madrid",
                color: Color::Blue,
                adjacent_cities: HashSet::from([
                    "Algiers",
                    "London",
                    "New York",
                    "Paris",
                    "São Paulo",
                ]),
            },
            City {
                name: "Milan",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["Essen", "Istanbul", "Paris"]),
            },
            City {
                name: "Montréal",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["Chicago", "New York", "Washington"]),
            },
            City {
                name: "New York",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["London", "Madrid", "Montréal", "Washington"]),
            },
            City {
                name: "Paris",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["Algiers", "Essen", "London", "Madrid", "Milan"]),
            },
            City {
                name: "Saint Petersburg",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["Essen", "Istanbul", "Moscow"]),
            },
            City {
                name: "San Francisco",
                color: Color::Blue,
                adjacent_cities: HashSet::from(["Chicago", "Los Angeles", "Manila", "Tokyo"]),
            },
        ]
    }

    fn make_yellow_cities() -> Vec<City> {
        vec![
            City {
                name: "Bogotá",
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    "Buenos Aires",
                    "Lima",
                    "Mexico City",
                    "Miami",
                    "São Paulo",
                ]),
            },
            City {
                name: "Buenos Aires",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Bogotá", "São Paulo"]),
            },
            City {
                name: "Johannesburg",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Kinshasa", "Khartoum"]),
            },
            City {
                name: "Kinshasa",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Johannesburg", "Khartoum", "Lagos"]),
            },
            City {
                name: "Khartoum",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Cairo", "Johannesburg", "Kinshasa", "Lagos"]),
            },
            City {
                name: "Lagos",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Khartoum", "Kinshasa", "São Paulo"]),
            },
            City {
                name: "Lima",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Bogotá", "Mexico City", "Santiago"]),
            },
            City {
                name: "Los Angeles",
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    "Chicago",
                    "Mexico City",
                    "San Francisco",
                    "Sydney",
                ]),
            },
            City {
                name: "Mexico City",
                color: Color::Yellow,
                adjacent_cities: HashSet::from([
                    "Bogotá",
                    "Chicago",
                    "Lima",
                    "Los Angeles",
                    "Miami",
                ]),
            },
            City {
                name: "Miami",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Atlanta", "Bogotá", "Mexico City", "Washington"]),
            },
            City {
                name: "Santiago",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Lima"]),
            },
            City {
                name: "São Paulo",
                color: Color::Yellow,
                adjacent_cities: HashSet::from(["Bogotá", "Buenos Aires", "Lagos", "Madrid"]),
            },
        ]
    }

    fn make_black_cities() -> Vec<City> {
        vec![
            City {
                name: "Algiers",
                color: Color::Black,
                adjacent_cities: HashSet::from(["Cairo", "Istanbul", "Madrid", "Paris"]),
            },
            City {
                name: "Baghdad",
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    "Cairo", "Istanbul", "Karachi", "Riyadh", "Tehran",
                ]),
            },
            City {
                name: "Cairo",
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    "Algiers", "Baghdad", "Istanbul", "Khartoum", "Riyadh",
                ]),
            },
            City {
                name: "Chennai",
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    "Bangkok", "Delhi", "Jakarta", "Kolkata", "Mumbai",
                ]),
            },
            City {
                name: "Delhi",
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    "Chennai", "Karachi", "Kolkata", "Mumbai", "Tehran",
                ]),
            },
            City {
                name: "Istanbul",
                color: Color::Black,
                adjacent_cities: HashSet::from([
                    "Algiers",
                    "Baghdad",
                    "Cairo",
                    "Milan",
                    "Moscow",
                    "Saint Petersburg",
                ]),
            },
            City {
                name: "Karachi",
                color: Color::Black,
                adjacent_cities: HashSet::from(["Baghdad", "Delhi", "Mumbai", "Riyadh", "Tehran"]),
            },
            City {
                name: "Kolkata",
                color: Color::Black,
                adjacent_cities: HashSet::from(["Bangkok", "Chennai", "Delhi", "Hong Kong"]),
            },
            City {
                name: "Moscow",
                color: Color::Black,
                adjacent_cities: HashSet::from(["Istanbul", "Saint Petersburg", "Tehran"]),
            },
            City {
                name: "Riyadh",
                color: Color::Black,
                adjacent_cities: HashSet::from(["Baghdad", "Cairo", "Karachi"]),
            },
            City {
                name: "Tehran",
                color: Color::Black,
                adjacent_cities: HashSet::from(["Baghdad", "Delhi", "Karachi", "Moscow"]),
            },
        ]
    }

    fn make_red_cities() -> Vec<City> {
        vec![
            City {
                name: "Bangkok",
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    "Chennai",
                    "Ho Chi Minh City",
                    "Hong Kong",
                    "Jakarta",
                    "Kolkata",
                ]),
            },
            City {
                name: "Beijing",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Seoul", "Shanghai"]),
            },
            City {
                name: "Ho Chi Minh City",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Bangkok", "Hong Kong", "Jakarta", "Manila"]),
            },
            City {
                name: "Hong Kong",
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    "Bangkok",
                    "Ho Chi Minh City",
                    "Kolkata",
                    "Manila",
                    "Shanghai",
                    "Taipei",
                ]),
            },
            City {
                name: "Jakarta",
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    "Bangkok",
                    "Chennai",
                    "Ho Chi Minh City",
                    "Sydney",
                ]),
            },
            City {
                name: "Manila",
                color: Color::Red,
                adjacent_cities: HashSet::from([
                    "Ho Chi Minh City",
                    "Hong Kong",
                    "San Francisco",
                    "Sydney",
                    "Taipei",
                ]),
            },
            City {
                name: "Osaka",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Taipei", "Tokyo"]),
            },
            City {
                name: "Seoul",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Beijing", "Shanghai", "Tokyo"]),
            },
            City {
                name: "Shanghai",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Beijing", "Hong Kong", "Seoul", "Taipei", "Tokyo"]),
            },
            City {
                name: "Sydney",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Jakarta", "Los Angeles", "Manila"]),
            },
            City {
                name: "Taipei",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Hong Kong", "Manila", "Osaka", "Shanghai"]),
            },
            City {
                name: "Tokyo",
                color: Color::Red,
                adjacent_cities: HashSet::from(["Osaka", "San Francisco", "Seoul", "Shanghai"]),
            },
        ]
    }
}