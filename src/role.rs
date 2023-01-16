pub mod role {
    use std::collections::VecDeque;

    use crossterm::style::{Color, Stylize};

    use crate::deck::Deck;

    #[derive(Clone)]
    pub struct RoleCard(pub Role);

    #[derive(Debug, Clone)]
    pub enum Role {
        ContingencyPlanner,
        Dispatcher,
        Medic,
        OperationsExpert,
        QuarantineSpecialist,
        Researcher,
        Scientist,
    }

    impl Deck<RoleCard> {
        pub fn fill(&mut self) {
            self.0.append(&mut VecDeque::from(vec![
                RoleCard(Role::ContingencyPlanner),
                RoleCard(Role::Dispatcher),
                RoleCard(Role::Medic),
                RoleCard(Role::OperationsExpert),
                RoleCard(Role::QuarantineSpecialist),
                RoleCard(Role::Researcher),
                RoleCard(Role::Scientist),
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
                Self::OperationsExpert => write!(f, "{}", "Operations Expert".with(Color::Green)),
                Self::QuarantineSpecialist => {
                    write!(f, "{}", "Quarantine Specialist".with(Color::DarkGreen))
                }
                Self::Researcher => write!(f, "{}", "Researcher".with(Color::DarkYellow)),
                Self::Scientist => write!(f, "{}", "Scientist".with(Color::White)),
            }
        }
    }
}
