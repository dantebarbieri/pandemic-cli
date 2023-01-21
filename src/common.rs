use crossterm::style::{Color as ConsoleColor, Stylize};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Color {
    Yellow,
    Black,
    Blue,
    Red,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yellow => write!(f, "{}", "Yellow".with(ConsoleColor::Yellow)),
            Self::Black => write!(f, "{}", "Black".with(ConsoleColor::DarkMagenta)),
            Self::Blue => write!(f, "{}", "Blue".with(ConsoleColor::Blue)),
            Self::Red => write!(f, "{}", "Red".with(ConsoleColor::Red)),
        }
    }
}
