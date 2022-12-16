use super::{
    character::get_ability_score_rolls, character::AbilityName, character::Alignment,
    character::CharacterSheet, classes::AvailableClasses, races::AvailableRaces,
};
use enum_iterator::{all, Sequence};
use num::Integer;
use std::{
    cmp, fmt,
    io::{self, stdin, stdout, Stdout, Write},
    ops::Index,
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};
use util::EnumIter;

/// Represents a menu item with a name and a value.
#[derive(Clone)]
struct MenuItem<T: Copy> {
    name: String,
    value: T,
}

fn prompt_and_read_selection<T: Copy>(
    prompt: &str,
    menu: &Vec<MenuItem<T>>,
) -> Result<T, io::Error> {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let prompt = prompt.to_string() + "\r\n";
    let prompt = prompt.as_str();

    let next_cursor: u16 = prompt.to_string().split("\n").count() as u16 + 1;

    let mut cursor_pos = 0;

    render_menu(prompt, &menu, &cursor_pos);

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, next_cursor),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Esc => exit(&mut stdout),
            Key::Ctrl('c') => exit(&mut stdout),
            Key::Up => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                }
                render_menu(prompt, &menu, &cursor_pos);
                stdout.flush().unwrap();
            }
            Key::Down => {
                if cursor_pos < menu.len() - 1 {
                    cursor_pos += 1;
                }
                render_menu(prompt, &menu, &cursor_pos);
                stdout.flush().unwrap();
            }
            Key::Char('\r') => break,
            Key::Char('\n') => break,
            _ => {}
        }
    }

    let selected_item = &menu[cursor_pos];

    write!(stdout, "{}", termion::cursor::Show).unwrap();

    return Ok(selected_item.value);
}

fn render_menu<T: Copy>(prompt: &str, menu: &Vec<MenuItem<T>>, cursor_pos: &usize) {
    let cursor_pos = cursor_pos.clone();
    let mut to_render = String::new();
    to_render.push_str(prompt);
    to_render.push_str("\r\n");

    let menu_render = menu
        .iter()
        .enumerate()
        .map(|(i, item)| {
            if i == cursor_pos {
                // highlight the selected item
                format!(
                    "{}{}{}",
                    termion::style::Invert,
                    item.name,
                    termion::style::NoInvert
                )
            } else {
                format!("{}", item.name)
            }
        })
        .collect::<Vec<String>>()
        .join("\r\n");

    to_render.push_str(&menu_render);

    render(to_render);
}

fn render(to_render: String) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        to_render + "\r\n",
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();
}

fn prompt_and_read_input<T: fmt::Display>(
    prompt: T,
    existing_value: &String,
) -> Result<String, io::Error> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine
    )
    .unwrap();

    let prompt = prompt.to_string() + "\r\n";
    let prompt = prompt.as_str();

    let next_cursor_row: u16 = prompt.to_string().split("\n").count() as u16 + 1;

    let mut input = existing_value.to_string();

    write!(
        stdout,
        "{}{}{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        prompt,
        termion::cursor::Goto(1, next_cursor_row),
        input,
        termion::cursor::Show
    )
    .unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();
    // show the input as the user types
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => exit(&mut stdout),
            Key::Ctrl('c') => exit(&mut stdout),
            Key::Char('\r') => break,
            Key::Char('\n') => break,
            Key::Char(c) => {
                input.push(c);
                write!(
                    stdout,
                    "{}{}{}",
                    termion::cursor::Goto(1, next_cursor_row),
                    termion::clear::CurrentLine,
                    input
                )
                .unwrap();
            }
            Key::Backspace => {
                input.pop();
                write!(
                    stdout,
                    "{}{}{}",
                    termion::cursor::Goto(1, next_cursor_row),
                    termion::clear::CurrentLine,
                    input,
                )
                .unwrap();
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    return Ok(input);
}

pub struct App {
    character_sheet: CharacterSheet,
    page_stack: Vec<Page>,
    current_page: usize,
    ability_score_rolls: Vec<i8>,
}

impl App {
    pub fn new() -> Self {
        let character_sheet = CharacterSheet::new();
        let page_stack = Page::iter();
        Self {
            character_sheet,
            page_stack,
            current_page: 0,
            ability_score_rolls: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        while self.current_page < self.page_stack.len() {
            match self.page_stack[self.current_page] {
                Page::Name => self.name_page(),
                Page::Race => self.race_page(),
                Page::Class => self.class_page(),
                Page::Abilities => self.abilities_page(),
                _ => break,
            }
        }

        print_character_sheet(&self.character_sheet);
    }

    fn adjust_ability_score_for_race(&mut self) {
        let race = self.character_sheet.race;
        let ability_score_increases = race.get_ability_score_increases();
        for (name, increase) in ability_score_increases {
            let score = self.character_sheet.ability_scores.get(name);
            self.character_sheet
                .ability_scores
                .set(name, score + increase);
        }
    }

    fn name_page(&mut self) {
        let mut name = self.character_sheet.name.clone();
        name = prompt_and_read_input("What is your character's name?", &name).unwrap();

        self.character_sheet.name = name.trim().to_string();
        self.current_page += 1;
    }

    fn race_page(&mut self) {
        let mut menu_items = Vec::new();
        for race in AvailableRaces::iter() {
            let menu_item = MenuItem {
                name: race.to_string(),
                value: race,
            };
            menu_items.push(menu_item);
        }

        let result = prompt_and_read_selection("What is your character's race?", &menu_items);
        self.character_sheet.race = result.unwrap();

        // show alignment options
        let mut alignment_menu_items = Vec::new();
        for alignment in Alignment::iter() {
            let menu_item = MenuItem {
                name: alignment.to_string(),
                value: alignment,
            };
            alignment_menu_items.push(menu_item);
        }

        let result =
            prompt_and_read_selection("What is your character's alignment?", &alignment_menu_items);
        self.character_sheet.alignment = result.unwrap();

        self.current_page += 1;
    }

    fn class_page(&mut self) {
        let mut menu_items = Vec::new();
        for class in AvailableClasses::iter() {
            let menu_item = MenuItem {
                name: class.to_string(),
                value: class,
            };
            menu_items.push(menu_item);
        }

        let result = prompt_and_read_selection("What is your character's class?", &menu_items);
        self.character_sheet.class = result.unwrap();
        self.current_page += 1;
    }

    fn abilities_page(&mut self) {
        let mut point_buy_menu_items = Vec::new();
        point_buy_menu_items.push(MenuItem {
            name: "Roll".to_string(),
            value: false,
        });

        point_buy_menu_items.push(MenuItem {
            name: "Point Buy".to_string(),
            value: true,
        });

        let point_buy = prompt_and_read_selection(
            "Would you like to roll for your ability scores or use point buy?",
            &point_buy_menu_items,
        )
        .unwrap();

        if point_buy {
            return self.point_buy_page();
        }

        self.ability_score_rolls = get_ability_score_rolls();
        return self.roll_page();
    }

    fn point_buy_page(&mut self) {
        let mut menu_items = Vec::new();
        let ability_names = AbilityName::iter();

        for ability in AbilityName::iter() {
            let menu_item = MenuItem {
                name: ability.to_string(),
                value: ability,
            };
            menu_items.push(menu_item);
        }

        let mut pool = 27;
        let max = 15;
        let min = 8;
        let mut i = 0;

        let mut ability_scores = self.character_sheet.ability_scores.clone();

        while i < menu_items.len() {
            let ability = menu_items[i].value;
            let mut score = ability_scores.get(ability);
            // get whichever is less, the remaining pool or the max score
            score = prompt_and_read_score_inc_dec(
                format!("Adjust points for {}:", ability),
                &score,
                &mut pool,
            )
            .unwrap();
            ability_scores.set(ability, score);
            i += 1;
            if i == menu_items.len() && pool > 0 {
                let menu_items = vec![
                    MenuItem {
                        name: "Yes".to_string(),
                        value: true,
                    },
                    MenuItem {
                        name: "No".to_string(),
                        value: false,
                    },
                ];
                let confirmed = prompt_and_read_selection(
                    &format!(
                        "You have {} points remaining. Are you sure you want to proceed?",
                        pool
                    ),
                    &menu_items,
                )
                .unwrap();
                if !confirmed {
                    i = 0;
                }
            }
        }

        self.character_sheet.ability_scores = ability_scores;
        self.current_page += 1;
    }

    fn roll_page(&mut self) {
        let rolls = self.ability_score_rolls.clone();
        let mut rolls_clone = rolls.clone();
        let mut menu_items = Vec::new();

        for name in AbilityName::iter() {
            let menu_item = MenuItem {
                name: name.to_string(),
                value: name,
            };
            menu_items.push(menu_item);
        }

        for roll in rolls {
            let available_scores = rolls_clone
                .iter()
                .filter(|r| !menu_items.iter().any(|item| item.name == r.to_string()))
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            let result = prompt_and_read_selection(
                &format!(
                    "{}\r\nWhat ability score would you like to assign {} to?",
                    available_scores, roll
                ),
                &menu_items,
            )
            .unwrap();
            self.character_sheet.ability_scores.set(result, roll);
            menu_items.retain(|item| item.value != result);
            // remove the first item from the rolls_clone
            rolls_clone.remove(0);
        }

        // prompt the user to confirm the ability scores
        let prompt = "Proceed with these ability scores?\r\n\r\n";
        let mut prompt = prompt.to_string();

        for ability in AbilityName::iter() {
            let score = self.character_sheet.ability_scores.get(ability);
            prompt += &format!("{}: {}\r\n", ability, score);
        }

        // remove the last \r\n from the prompt
        prompt.pop();

        let menu_items = vec![
            MenuItem {
                name: "Yes".to_string(),
                value: true,
            },
            MenuItem {
                name: "No".to_string(),
                value: false,
            },
        ];
        let confirmed = prompt_and_read_selection(&prompt, &menu_items).unwrap();
        if !confirmed {
            self.roll_page();
            return;
        }

        self.current_page += 1;
    }
}

#[derive(Debug, Sequence, EnumIter)]
enum Page {
    Name,
    Race,
    Class,
    Abilities,
    Background,
    Equipment,
    Spells,
    Feats,
    Bio,
    Review,
}

fn print_character_sheet(character_sheet: &CharacterSheet) {
    let mut to_render = String::new();
    for key in CharacterSheet::keys() {
        let value = character_sheet[key].to_string().replace("\n", "");
        let key = key.to_string() + ": ";
        let key = key.replace("_", " ");
        // make the first letter of the key uppercase
        let key = key
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();
        to_render.push_str(&format!("{: <20}{}\r\n", key, value));
    }

    render(to_render);
}

fn exit(stdout: &mut RawTerminal<Stdout>) {
    write!(
        stdout,
        "{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        "Goodbye!\r\n",
        termion::cursor::Hide
    )
    .unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    std::process::exit(0);
}

fn prompt_and_read_score_inc_dec<T, Y>(
    _prompt: T,
    existing_value: &Y,
    pool: &mut Y,
) -> Result<Y, io::Error>
where
    T: fmt::Display,
    Y: Integer + fmt::Display + Copy,
{
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::CurrentLine
    )
    .unwrap();

    let prompt_remaining = format!("Pool Remaining: {}\r\n", pool);
    let prompt = _prompt.to_string() + "\r\n";
    let prompt = prompt_remaining + prompt.as_str();

    let mut pool_used = Y::zero();

    let next_cursor_row: u16 = prompt.to_string().split("\n").count() as u16 + 1;

    let mut input = existing_value.clone() as Y;

    /// the minimum score
    let min_score: Y = int(8);

    /// the maximum score
    let max_score: Y = int(15);

    /// the threshold at which we start incrementing by 2
    let threshold: Y = int(13);

    write!(
        stdout,
        "{}{}{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        prompt,
        termion::cursor::Goto(1, next_cursor_row),
        input,
        termion::cursor::Show
    )
    .unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();
    // show the input as the user types
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => exit(&mut stdout),
            Key::Ctrl('c') => exit(&mut stdout),
            Key::Char('\r') => break,
            Key::Char('\n') => break,
            Key::Up => {
                if input >= max_score || pool_used >= pool.clone() {
                    continue;
                }
                input = input + Y::one();
                if input <= threshold {
                    pool_used = pool_used + Y::one();
                } else {
                    pool_used = pool_used + int(2);
                }
                let prompt_remaining = format!("Pool Remaining: {}\r\n", pool.clone() - pool_used);
                let prompt = _prompt.to_string() + "\r\n";
                let prompt = prompt_remaining + prompt.as_str();

                write!(
                    stdout,
                    "{}{}{}{}{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    prompt,
                    termion::cursor::Goto(1, next_cursor_row),
                    input,
                    termion::cursor::Show
                )
                .unwrap();
                stdout.flush().unwrap();

                write!(
                    stdout,
                    "{}{}{}",
                    termion::cursor::Goto(1, next_cursor_row),
                    termion::clear::CurrentLine,
                    input
                )
                .unwrap();
                stdout.flush().unwrap();
            }
            Key::Down => {
                if input <= min_score {
                    continue;
                }
                input = input - Y::one();
                if input < threshold {
                    pool_used = cmp::max(Y::zero(), pool_used - Y::one());
                } else {
                    pool_used = cmp::max(Y::zero(), pool_used - int(2));
                }

                let prompt_remaining = format!("Pool Remaining: {}\r\n", pool.clone() - pool_used);
                let prompt = _prompt.to_string() + "\r\n";
                let prompt = prompt_remaining + prompt.as_str();

                write!(
                    stdout,
                    "{}{}{}{}{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1),
                    prompt,
                    termion::cursor::Goto(1, next_cursor_row),
                    input,
                    termion::cursor::Show
                )
                .unwrap();
                stdout.flush().unwrap();
                write!(
                    stdout,
                    "{}{}{}",
                    termion::cursor::Goto(1, next_cursor_row),
                    termion::clear::CurrentLine,
                    input,
                )
                .unwrap();
                stdout.flush().unwrap();
            }
            _ => {}
        }
    }

    if pool.clone() > int(0) {
        *pool = pool.clone() - pool_used;
    }

    return Ok(input);
}

/// ! HACK
///
/// This is a garbage solution to the fact that I can't figure out how to
/// coerce an integer literal to a generic type Y in the prompt_and_read_score_inc_dec function.
///
/// # Example
/// ```
/// let x = int(5);
/// ```
fn int<Y: Integer>(x: i32) -> Y {
    let mut num = Y::zero();
    for _ in 0..x {
        num = num + Y::one();
    }
    return num;
}
