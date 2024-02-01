//! core [`crate::calculator`] sub-demo code

#![deny(warnings)]
#![allow(unused)]

use std::{collections::HashMap, fmt, io, thread, time::Duration};

use crossterm::{
    event::{read, Event, KeyCode},
    style::Colorize,
};
use iced::color;

use crate::{
    calculator::DumbCalculator,
    dlt_comps, dltc,
    lblscreen::{DumbLineByLineScreen, LBLScreenMapValueTrait, LBLScreenSettings},
    ltemp::{
        DumbLineTemplate, EscapedLineTempComp, LineTempComp, LineTempCompMapValueTrait,
        LineTempCompTrait, MappedLineTempComp, MappedLineTempCompBuilder,
        MappedLineTempCompSettings, FLEXIBLE_WIDTH_EX,
    },
};

pub fn handle_demo_calculator() {
    let mut ui = CalculatorUI::new_and_init();
    ui.run();
}

const RESULT_WIDTH: u16 = 11;
const DISPLAY_WIDTH: u16 = RESULT_WIDTH + 2;
const FIXED_WIDTH: u16 = DISPLAY_WIDTH;
const ENTER_DELAY_MILLIS: u64 = 100;

struct CalculatorUI {
    calculator: DumbCalculator,
    screen: DumbLineByLineScreen,
    key_map: Vec<Vec<char>>,
    selected_key_rc: (usize, usize),
    refresh_state: RefreshState,
}
impl CalculatorUI {
    fn new_and_init() -> Self {
        let mut line_temps = Vec::<DumbLineTemplate>::new();

        let mut comps = dlt_comps![dltc!("display", fixed_width = DISPLAY_WIDTH)];
        let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            " ",
            CalculatorUI::_create_key('7', 1),
            " ",
            CalculatorUI::_create_key('8', 1),
            " ",
            CalculatorUI::_create_key('9', 1),
            " | ",
            CalculatorUI::_create_key('C', 3),
            " "
        ];
        let keys_8 = CalculatorUI::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            " ",
            CalculatorUI::_create_key('4', 1),
            " ",
            CalculatorUI::_create_key('5', 1),
            " ",
            CalculatorUI::_create_key('6', 1),
            " | ",
            CalculatorUI::_create_key('*', 1),
            " ",
            CalculatorUI::_create_key('/', 1),
            " "
        ];
        let keys_5 = CalculatorUI::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            " ",
            CalculatorUI::_create_key('1', 1),
            " ",
            CalculatorUI::_create_key('2', 1),
            " ",
            CalculatorUI::_create_key('3', 1),
            " | ",
            CalculatorUI::_create_key('+', 1),
            " ",
            CalculatorUI::_create_key('-', 1),
            " "
        ];
        let keys_2 = CalculatorUI::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            " ",
            CalculatorUI::_create_key('%', 1),
            " ",
            CalculatorUI::_create_key('0', 1),
            " ",
            CalculatorUI::_create_key('.', 1),
            " | ",
            CalculatorUI::_create_key('=', 3),
            " "
        ];
        let keys_0 = CalculatorUI::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(DISPLAY_WIDTH as u16, &comps);
        line_temps.push(temp);

        let settings = LBLScreenSettings {
            line_prefix: Some("\t|".to_string()),
            line_suffix: Some("|".to_string()),
            top_line: Some(format!("\n\t{}", "=".repeat(FIXED_WIDTH as usize + 2))),
            bottom_line: Some(format!("\t{}\n", "=".repeat(FIXED_WIDTH as usize + 2))),
            ..LBLScreenSettings::default()
        };
        let mut screen = DumbLineByLineScreen::new(line_temps, settings);
        println!();
        println!("* arrow keys to move selected key");
        println!("* space key to commit selected key");
        println!("* can press corresponding keys directly");
        println!("* note that 'c' is the same as 'C' and the enter key is the same as '='");

        // 1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣
        // ±
        // ➀ (U+2780)

        // let v = 0x1F600;
        // let character = std::char::from_u32(v).unwrap();
        // let string = character.to_string();

        // // Split the string into grapheme clusters
        // let graphemes: Vec<&str> = string.graphemes(true).collect();

        // // Print each grapheme cluster
        // for grapheme in graphemes {
        //     println!("{}", grapheme);
        // }

        // use unicode_segmentation::UnicodeSegmentation;

        // let string = "😄👋🏽";
        // let graphemes: Vec<&str> = string.graphemes(true).collect();
        // let num_graphemes = graphemes.len();

        // println!("Number of graphemes: {}", num_graphemes); // Output: Number of graphemes: 3

        // [dependencies]
        // unicode-segmentation = "1.8.0"
        if false {
            println!("{} :({}):{} ", "1️⃣", "1️⃣".len(), "123".red());
            for i in 0..=9 {
                let v = i + 0x277f;
                let char1 = std::char::from_u32(v).unwrap();
                print!("{} ", char1);
            }
            println!();
        }

        if false {
            // ➕➖✖️➗🟰🇦🇨▪%±
            println!("±\u{2780}ABC1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣0️⃣ABC±\u{2780}");
            let v = 0x1F600;
            let character = std::char::from_u32(v).unwrap();
            let string = character.to_string();
            println!("{}", string); // Output: 😄
            let a = '😄' as u32;
            let a_char = std::char::from_u32(a).unwrap();
            println!("A:{}", a_char);
        }

        screen.init();

        let key_map = vec![keys_8, keys_5, keys_2, keys_0];
        let key = '0';
        let key_pressed_coor = CalculatorUI::_get_key_coor(key, &key_map).unwrap();
        Self {
            calculator: DumbCalculator::new(),
            screen: screen,
            key_map: key_map,
            selected_key_rc: key_pressed_coor,
            refresh_state: RefreshState {
                display: String::from("0"),
                selected_key: Some(key),
                highlight_selected: false,
            },
        }
    }
    fn _create_key(key: char, fixed_width: u16) -> MappedLineTempCompBuilder {
        dltc!(&key.to_string(), fixed_width = fixed_width, align = 'C')
    }
    fn _scan_for_keys(components: &Vec<LineTempComp>) -> Vec<char> {
        let mut keys = Vec::<char>::new();
        for comp in components {
            if let LineTempComp::Mapped(mapped_comp) = comp {
                let key = mapped_comp.get_map_key().chars().next().unwrap();
                keys.push(key);
                if mapped_comp.get_min_width() > 2 {
                    keys.push(key);
                }
            }
        }
        keys
    }
    fn run(mut self) {
        self._refresh();
        let key = self.refresh_state.selected_key.unwrap();
        self.refresh_state.selected_key = Some(key);
        // self.key_press_state.highlight_selected = true;
        self._refresh_for_keys(&vec![key.to_string().as_ref()]);
        // thread::sleep(Duration::from_millis(500));
        // self.key_press_state.highlight_selected = false;
        // self._refresh_for_keys(&keys);

        // let stdin = io::stdin();
        // for c in stdin.keys() {
        //     match c.unwrap() {
        //         Key::Char('q') => break,
        //         Key::Char(c)   => println!("You pressed {}", c),
        //         Key::Ctrl(c)   => println!("You pressed Ctrl-{}", c),
        //         Key::Alt(c)    => println!("You pressed Alt-{}", c),
        //         _              => println!("You pressed a key"),
        //     }
        // }
        loop {
            if let Event::Key(event) = read().unwrap() {
                match event.code {
                    KeyCode::Up => {
                        self._move_key_selected(MoveDir::Up);
                    }
                    KeyCode::Down => {
                        self._move_key_selected(MoveDir::Down);
                    }
                    KeyCode::Left => {
                        self._move_key_selected(MoveDir::Left);
                    }
                    KeyCode::Right => {
                        self._move_key_selected(MoveDir::Right);
                    }
                    KeyCode::Char(' ') => {
                        self._commit_key_selected();
                    }
                    KeyCode::Char(c) => {
                        self._select_and_enter_key(c);
                    }
                    KeyCode::Enter => {
                        self._select_and_enter_key('=');
                    }
                    // KeyCode::Char(c)   => println!("You pressed {}", c),
                    // KeyCode::Enter     => println!("You pressed Enter"),
                    // KeyCode::Up        => println!("You pressed Up"),
                    // KeyCode::Down      => println!("You pressed Down"),
                    // KeyCode::Left      => println!("You pressed Left"),
                    // KeyCode::Right     => println!("You pressed Right"),
                    _ => {}
                }
            }
        }
    }
    fn _refresh(&mut self) {
        let map_value_fn =
            |key: &str| -> Option<(String, u16)> { self.refresh_state.map_value(key) };
        self.screen.refresh_ex(map_value_fn);
    }
    fn _refresh_for_keys(&mut self, keys: &Vec<&str>) {
        let map_value_fn =
            |key: &str| -> Option<(String, u16)> { self.refresh_state.map_value(key) };
        self.screen.refresh_for_keys_ex(keys, map_value_fn);
    }
    fn _get_key_coor(key: char, key_map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for (row_idx, row) in key_map.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if *cell == key {
                    return Some((row_idx, col_idx));
                }
            }
        }
        None
    }
    fn _commit_key_selected(&mut self) {
        let key = self.key_map[self.selected_key_rc.0][self.selected_key_rc.1];
        self.refresh_state.highlight_selected = true;
        self._refresh_for_keys(&vec![key.to_string().as_ref()]);

        thread::sleep(Duration::from_millis(ENTER_DELAY_MILLIS));

        self.refresh_state.highlight_selected = false;
        self._refresh_for_keys(&vec![key.to_string().as_ref()]);

        if key == 'C' {
            self.calculator.reset();
        } else {
            self.calculator.push(key.to_string().as_str()).unwrap();
        }
        self.refresh_state.display = self.calculator.get_display_sized(RESULT_WIDTH as usize);
        self._refresh_for_keys(&vec!["display"]);
    }
    fn _select_and_enter_key(&mut self, key: char) {
        let key = key.to_ascii_uppercase();
        let key_coor = CalculatorUI::_get_key_coor(key, &self.key_map);
        if let Some((row_idx, col_idx)) = key_coor {
            let key = self.key_map[self.selected_key_rc.0][self.selected_key_rc.1];
            self.refresh_state.selected_key = None;
            self._refresh_for_keys(&vec![key.to_string().as_ref()]);

            self.selected_key_rc = (row_idx, col_idx);
            let key = self.key_map[self.selected_key_rc.0][self.selected_key_rc.1];
            self.refresh_state.selected_key = Some(key);
            self._refresh_for_keys(&vec![key.to_string().as_ref()]);

            self._commit_key_selected();
        }
    }
    fn _move_key_selected(&mut self, move_dir: MoveDir) {
        let key = self.key_map[self.selected_key_rc.0][self.selected_key_rc.1];
        self.refresh_state.selected_key = None;
        self._refresh_for_keys(&vec![key.to_string().as_ref()]);

        let key = self._adjust_key_selected(move_dir);
        self.refresh_state.selected_key = Some(key);
        self._refresh_for_keys(&vec![key.to_string().as_ref()]);
    }
    fn _adjust_key_selected(&mut self, move_dir: MoveDir) -> char {
        let row_count = self.key_map.len();
        let col_count = self.key_map[0].len();
        let (row_idx, col_idx) = self.selected_key_rc;
        let old_key = self.key_map[row_idx][col_idx];
        loop {
            let (row_idx, col_idx) = self.selected_key_rc;
            self.selected_key_rc = match move_dir {
                MoveDir::Up => {
                    if row_idx > 0 {
                        (row_idx - 1, col_idx)
                    } else {
                        (row_count - 1, col_idx)
                    }
                }
                MoveDir::Down => {
                    if row_idx < row_count - 1 {
                        (row_idx + 1, col_idx)
                    } else {
                        (0, col_idx)
                    }
                }
                MoveDir::Left => {
                    if col_idx > 0 {
                        (row_idx, col_idx - 1)
                    } else {
                        (row_idx, col_count - 1)
                    }
                }
                MoveDir::Right => {
                    if col_idx < col_count - 1 {
                        (row_idx, col_idx + 1)
                    } else {
                        (row_idx, 0)
                    }
                }
            };
            let (row_idx, col_idx) = self.selected_key_rc;
            let new_key = self.key_map[row_idx][col_idx];
            if new_key != old_key {
                break new_key;
            }
        }
    }
}

enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

struct RefreshState {
    display: String,
    selected_key: Option<char>,
    highlight_selected: bool,
}

impl LBLScreenMapValueTrait for RefreshState {
    type VALUE = String;
    fn map_value(&self, key: &str) -> Option<(String, u16)> {
        if key.len() == 1 {
            let current_key = self.selected_key;
            let key = key.chars().next().unwrap();
            let key_value = key.to_string();
            let key_value = match current_key {
                Some(current_key) if current_key == key => {
                    if self.highlight_selected {
                        format!("\x1B[7m{}\x1B[0m", key_value) // invert color
                    } else {
                        if true {
                            format!("\x1B[31m{}\x1B[0m", key_value) // underline
                        } else {
                            format!("\x1B[4m{}\x1B[0m", key_value) // underline
                        }
                    }
                }
                _ => key_value,
            };
            Some((key_value, 1))
        } else if key == "display" {
            let mut display_result = self.display.clone();
            if display_result.len() < RESULT_WIDTH as usize {
                let room = RESULT_WIDTH - display_result.len() as u16;
                display_result = format!("{}{}", " ".repeat(room as usize), display_result);
            }
            let display_result = format!("\x1B[7m {} \x1B[0m", display_result);
            Some((display_result, DISPLAY_WIDTH))
        } else {
            None
        }
    }
}
