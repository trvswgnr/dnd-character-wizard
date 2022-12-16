use super::*;
use enum_iterator::{all, Sequence};
use rand::Rng;
use std::{collections::HashMap, error::Error, io};
use util::{EnumIter, EnumString};

#[derive(Copy, Clone, Debug, Sequence, EnumIter, EnumString)]
pub enum AvailableClasses {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}
