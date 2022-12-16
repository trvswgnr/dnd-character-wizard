use super::*;
use classes::AvailableClasses;
use enum_iterator::{all, Sequence};
use num::Integer;
use races::AvailableRaces;
use rand::Rng;
use std::ops::Index;
use std::{collections::HashMap, error::Error, io};
use util::{EnumIter, EnumString};

pub struct CharacterSheet {
    pub name: Name,
    pub race: AvailableRaces,
    pub alignment: Alignment,
    pub class: AvailableClasses,
    pub level: Level,
    pub experience_points: ExperiencePoints,
    pub ability_scores: AbilityScores,
    pub point_buy: bool,
}

impl CharacterSheet {
    pub fn new() -> CharacterSheet {
        return CharacterSheet {
            name: "".to_string(),
            race: AvailableRaces::Human,
            alignment: Alignment::TrueNeutral,
            class: AvailableClasses::Barbarian,
            level: 1,
            experience_points: 0,
            ability_scores: AbilityScores::default(),
            point_buy: false,
        };
    }

    pub fn keys() -> Vec<&'static str> {
        return vec![
            "name",
            "race",
            "alignment",
            "class",
            "level",
            "experience_points",
            "ability_scores",
        ];
    }
}

impl Index<&str> for CharacterSheet {
    type Output = str;

    fn index(&self, index: &str) -> &Self::Output {
        /// return a member of this struct with the given name
        let x: String = match index {
            "name" => self.name.to_string(),
            "race" => self.race.to_string(),
            "alignment" => self.alignment.to_string(),
            "class" => self.class.to_string(),
            "level" => self.level.to_string(),
            "experience_points" => self.experience_points.to_string(),
            "ability_scores" => self.ability_scores.to_string(),
            "point_buy" => self.point_buy.to_string(),
            _ => "".to_string(),
        };
        return Box::leak(x.into_boxed_str());
    }
}
type Name = String;

#[derive(Copy, Clone, Debug, Sequence, EnumIter, EnumString)]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

type Level = i8;

type ExperiencePoints = u32;

type AbilityScore = i8;

trait AbilityModifier {
    fn get_modifier(&self) -> i8;
}

impl AbilityModifier for AbilityScore {
    fn get_modifier(&self) -> i8 {
        return (self - 10) / 2;
    }
}

#[derive(Debug, Clone)]
pub struct AbilityScores(HashMap<AbilityName, AbilityScore>);

impl Default for AbilityScores {
    fn default() -> Self {
        let mut ability_scores = HashMap::new();
        for ability in AbilityName::iter() {
            ability_scores.insert(ability, 8);
        }
        return AbilityScores(ability_scores);
    }
}

impl AbilityScores {
    pub fn get(&self, ability: AbilityName) -> AbilityScore {
        return self.0.get(&ability).unwrap().clone();
    }

    pub fn set(&mut self, ability: AbilityName, score: AbilityScore) {
        self.0.insert(ability, score);
    }

    // Sort the ability scores specically: STR, DEX, CON, INT, WIS, CHA
    pub fn get_sorted(&self) -> Vec<(AbilityName, AbilityScore)> {
        return vec![
            (AbilityName::Strength, self.get(AbilityName::Strength)),
            (AbilityName::Dexterity, self.get(AbilityName::Dexterity)),
            (
                AbilityName::Constitution,
                self.get(AbilityName::Constitution),
            ),
            (
                AbilityName::Intelligence,
                self.get(AbilityName::Intelligence),
            ),
            (AbilityName::Wisdom, self.get(AbilityName::Wisdom)),
            (AbilityName::Charisma, self.get(AbilityName::Charisma)),
        ];
    }
}

impl ToString for AbilityScores {
    fn to_string(&self) -> String {
        let mut ability_scores = String::new();
        for (ability, score) in self.get_sorted() {
            ability_scores.push_str(&format!("{}: {:<2}   ", ability, score));
        }
        return ability_scores;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence, EnumIter, EnumString)]
pub enum AbilityName {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
    ANY,
}

#[derive(Debug, Sequence, EnumIter, EnumString)]
enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

#[derive(Copy, Clone)]
enum Dice {
    D4 = 4,
    D6 = 6,
    D8 = 8,
    D10 = 10,
    D12 = 12,
    D20 = 20,
    D100 = 100,
}

trait Roll {
    fn roll(&mut self, num: i8) -> i8;
}

impl Roll for Dice {
    fn roll(&mut self, num: i8) -> i8 {
        let mut rng = rand::thread_rng();
        let mut total = 0;

        for _ in 0..num {
            total += rng.gen_range(1..=*self as i8);
        }

        total
    }
}

/// Roll for an ability score. This is done by rolling 4d6 and dropping the lowest roll.
fn roll_for_ability_score() -> i8 {
    let mut dice = Dice::D6;
    let mut rolls = [0; 4];

    for i in 0..4 {
        rolls[i] = Dice::D6.roll(1);
    }

    rolls.sort();

    rolls[1] + rolls[2] + rolls[3]
}

pub fn get_ability_score_rolls() -> Vec<i8> {
    let mut ability_score_rolls = Vec::new();

    for _ in 0..6 {
        ability_score_rolls.push(roll_for_ability_score());
    }

    // Sort the rolls from lowest to highest (ascending order)
    ability_score_rolls.sort();

    ability_score_rolls
}

// 2. Implement a function create_character_sheet() that prompts the user for input for each field in the character sheet and returns a new character sheet instance with the specified values.
fn create_character_sheet() -> CharacterSheet {
    let mut character_sheet = CharacterSheet::new();

    // Name
    // Controller::get_name(&mut character_sheet);

    // // Race
    // Controller::get_race(&mut character_sheet);

    character_sheet
}
