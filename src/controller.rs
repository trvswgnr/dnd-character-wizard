use super::{character::CharacterSheet, races::AvailableRaces};
use enum_iterator::{all, Sequence};
use std::{
    fmt,
    io::{self, stdin, stdout, Stdout, Write},
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
            }
            Key::Down => {
                if cursor_pos < menu.len() - 1 {
                    cursor_pos += 1;
                }
                render_menu(prompt, &menu, &cursor_pos);
            }
            Key::Char('\r') => break,
            Key::Char('\n') => break,
            _ => {}
        }
        stdout.flush().unwrap();
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

    let next_cursor: u16 = prompt.to_string().split("\n").count() as u16 + 1;

    write!(
        stdout,
        "{}{}{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        prompt,
        termion::cursor::Goto(1, next_cursor),
        termion::cursor::Show
    )
    .unwrap();
    stdout.flush().unwrap();

    let stdin = stdin();
    // show the input as the user types
    let mut input = existing_value.to_string();
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
                    termion::cursor::Goto(1, next_cursor),
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
                    termion::cursor::Goto(1, next_cursor),
                    termion::clear::CurrentLine,
                    input
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
}

impl App {
    pub fn new() -> Self {
        let character_sheet = CharacterSheet::new();
        let page_stack = Page::iter();
        Self {
            character_sheet,
            page_stack,
            current_page: 0,
        }
    }

    pub fn run(&mut self) {
        while self.current_page < self.page_stack.len() {
            match self.page_stack[self.current_page] {
                Page::Name => self.name_page(),
                Page::Race => self.race_page(),
                _ => break,
            }
        }
        print_character_sheet(&self.character_sheet);
    }

    fn name_page(&mut self) {
        let mut name = self.character_sheet.name.clone();
        name = prompt_and_read_input("What is your character's name?", &name).unwrap();

        self.character_sheet.name = name.trim().to_string();
        self.current_page += 1;
    }

    fn race_page(&mut self) {
        let mut menu_items: Vec<MenuItem<AvailableRaces>> = Vec::new();
        for race in AvailableRaces::iter() {
            let menu_item = MenuItem {
                name: race.to_string(),
                value: race,
            };
            menu_items.push(menu_item);
        }
        // reset the cursor position
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();
        let result = prompt_and_read_selection("What is your character's race?", &menu_items);
        self.character_sheet.race = result.unwrap();
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
        "Goodbye!",
        termion::cursor::Hide
    )
    .unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    std::process::exit(0);
}
