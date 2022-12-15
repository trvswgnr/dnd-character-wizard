use super::*;
use character::*;
use enum_iterator::{all, Sequence};
use rand::Rng;
use std::{collections::HashMap, error::Error, io};
use util::{EnumIter, EnumString};

#[derive(Copy, Clone, Debug, Sequence, EnumIter, EnumString)]
pub enum AvailableRaces {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
    Tiefling,
}

struct Race {
    name: String,
    ability_score_increases: Vec<(AbilityName, i8)>,
}

impl Race {
    fn new(race: AvailableRaces) -> Race {
        let name = race.to_string();
        let ability_score_increases = race.get_ability_score_increase();
        return Race {
            name,
            ability_score_increases,
        };
    }
}

impl AvailableRaces {
    fn get_ability_score_increase(&self) -> Vec<(AbilityName, i8)> {
        match *self {
            AvailableRaces::Dragonborn => {
                vec![(AbilityName::Strength, 2), (AbilityName::Charisma, 1)]
            }
            AvailableRaces::Dwarf => vec![(AbilityName::Constitution, 2)],
            AvailableRaces::Elf => vec![(AbilityName::Dexterity, 2)],
            AvailableRaces::Gnome => vec![(AbilityName::Intelligence, 2)],
            AvailableRaces::HalfElf => {
                vec![
                    (AbilityName::Charisma, 2),
                    (AbilityName::ANY, 1),
                    (AbilityName::ANY, 1),
                ]
            }
            AvailableRaces::HalfOrc => {
                vec![(AbilityName::Strength, 2), (AbilityName::Constitution, 1)]
            }
            AvailableRaces::Halfling => vec![(AbilityName::Dexterity, 2)],
            AvailableRaces::Human => vec![(AbilityName::ANY, 1), (AbilityName::ANY, 1)],
            AvailableRaces::Tiefling => {
                vec![(AbilityName::Intelligence, 1), (AbilityName::Charisma, 2)]
            }
        }
    }

    fn from<T: Into<String>>(string: T) -> AvailableRaces {
        let string = string.into();
        for race in AvailableRaces::iter() {
            if string.to_lowercase() == race.to_string().to_lowercase() {
                return race;
            }
        }

        return AvailableRaces::Human;
    }
}
