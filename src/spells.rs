use super::*;
use classes::*;
use enum_iterator::{all, Sequence};
use rand::Rng;
use std::{collections::HashMap, error::Error, io};
use util::{EnumIter, EnumString};

struct Spell {
    name: String,
    level: Level,
    school: School,
    casting_time: CastingTime,
    range: Range,
    components: Components,
    duration: Duration,
    description: String,
    higher_level: Option<String>,
    ritual: bool,
    concentration: bool,
    classes: Vec<AvailableClasses>,
    tags: Vec<String>,
    effect: Option<Effect>,
}

impl Spell {
    fn new() -> Spell {
        return Spell {
            name: "".to_string(),
            level: Level::Cantrip,
            school: School::Abjuration,
            casting_time: CastingTime::Action,
            range: Range::Touch,
            components: Components::default(),
            duration: Duration::Instantaneous,
            description: "".to_string(),
            higher_level: None,
            ritual: false,
            concentration: false,
            classes: vec![],
            tags: vec![],
            effect: None,
        };
    }
}

type Name = String;

#[derive(Debug, Sequence, EnumIter)]
enum School {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation,
}

#[derive(Debug, Sequence, EnumIter)]
enum CastingTime {
    Action,
    BonusAction,
    Reaction,
    Minute,
    Hour,
}

enum Range {
    OnSelf,
    Touch,
    Feet(i32),
    Miles(i32),
    Special,
}

#[derive(Debug, Sequence, EnumIter)]
enum Level {
    Cantrip,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
}

#[derive(Default)]
struct Components {
    verbal: bool,
    somatic: bool,
    material: Option<String>,
}

enum Duration {
    Concentration,
    Instantaneous,
    Rounds(i32),
    Minutes(i32),
    Hours(i32),
    Days(i32),
    Special,
}
enum Effect {
    Damage,
    Healing,
    Buff,
    Debuff,
    Summon,
    Other,
}

fn get_spell_by_name(name: &Name) -> Spell {
    let mut spell = Spell::new();
    spell.name = name.to_string();
    return spell;
}

fn get_spells() -> HashMap<Name, Spell> {
    let mut spells = HashMap::new();
    for school in School::iter() {
        for level in Level::iter() {
            let name: String = format!("{:?} {:?}", school, level);
            let spell = get_spell_by_name(&name);

            spells.insert(name, spell);
        }
    }
    return spells;
}
