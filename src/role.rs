use std::collections::VecDeque;

use crossterm::style::{Color, Stylize};

use crate::{deck::Deck, player_card::PlayerCard};

#[derive(Clone)]
pub struct RoleCard {
    pub(crate) role: Role,
    pub description: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub enum Role {
    ContingencyPlanner,
    Dispatcher,
    Medic,
    OperationsExpert(Option<PlayerCard>),
    QuarantineSpecialist,
    Researcher,
    Scientist,
}

impl Deck<RoleCard> {
    pub fn new() -> Self {
        Self(VecDeque::from(vec![
                RoleCard {
                    role: Role::ContingencyPlanner,
                    description: vec!["As an action, take any discarded Event card and store it on this card.", "When you play the stored Event card, remove it from the game.", "Limit: 1 Event card on this card at a time, which is not part of your hand."]
                },
                RoleCard {
                    role: Role::Dispatcher,
                    description: vec!["Move another player's pawn as if it were yours.", "As an action, move any pawn to a city with another pawn.", "Get permission before moving another player's pawn."]
                },
                RoleCard { role: Role::Medic, description: vec!["Remove all cubes of one color when doing Treat Disease.", "Automatically remove cubes of cured diseases from the city you are in (and prevent them from being placed there)."] },
                RoleCard {
                    role: Role::OperationsExpert(None),
                    description: vec!["As an action, build a research station in the city you are in (no City card needed).", "Once per turn as an action, move from a research station to any city by discarding any City card."]
                },
                RoleCard {
                    role: Role::QuarantineSpecialist,
                    description: vec!["Prevent disease cube placements (and outbreaks) in the city you are in and all cities connected to it."]
                },
                RoleCard {
                    role: Role::Researcher,
                    description: vec!["You may give any 1 of your City cards when you Share Knowledge. It need not match your city. A player who Shares Knowledge with you on their turn can take any 1 of your City cards."]
                },
                RoleCard {
                    role: Role::Scientist,
                    description: vec!["You need only 4 cards of the same color to do the Discover a Cure action."]
                },
            ]))
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ContingencyPlanner => {
                write!(f, "{}", "Contingency Planner".with(Color::Cyan))
            }
            Self::Dispatcher => write!(f, "{}", "Dispatcher".with(Color::Magenta)),
            Self::Medic => write!(f, "{}", "Medic".with(Color::DarkRed)),
            Self::OperationsExpert(card) => match card {
                Some(card) => write!(
                    f,
                    "{} holding {}",
                    "Operations Expert".with(Color::Green),
                    card
                ),
                None => write!(f, "{}", "Operations Expert".with(Color::Green)),
            },
            Self::QuarantineSpecialist => {
                write!(f, "{}", "Quarantine Specialist".with(Color::DarkGreen))
            }
            Self::Researcher => write!(f, "{}", "Researcher".with(Color::DarkYellow)),
            Self::Scientist => write!(f, "{}", "Scientist".with(Color::White)),
        }
    }
}
