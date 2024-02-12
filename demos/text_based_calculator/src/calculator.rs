#![deny(warnings)]
#![allow(unused)]

use std::{collections::HashMap, fmt, io, thread, time::Duration};

use crossterm::{
    event::{read, Event, KeyCode},
    style::Colorize,
    terminal::{disable_raw_mode, enable_raw_mode},
};

use rusty_dumb_tools::prelude::*;

// pub fn create_parser_calculator() -> DumbArgParser {
//     let mut parser = DumbArgParser::new();
//     parser.set_description("DumbCalculator demo.");
//     dap_arg!("mode", default = "text")
//         .set_description("calculator mode")
//         .set_with_desc_enums(vec![
//             "text:text based",
//             "rich:richer text-based",
//             //"gui: graphical",
//         ])
//         .add_to(&mut parser)
//         .unwrap();
//     parser
// }

// pub fn handle_calculator(parser: DumbArgParser) {
//     let mode = parser.get::<String>("mode").unwrap();
//     let richer = mode == "rich";
//     if richer {
//         CalculatorUI::<true>::new_and_init().run()
//     } else {
//         CalculatorUI::<false>::new_and_init().run()
//     };
// }

const RESULT_WIDTH: u16 = 11;
const DISPLAY_WIDTH: u16 = RESULT_WIDTH + 2;
const FIXED_WIDTH: u16 = DISPLAY_WIDTH;
const ENTER_DELAY_MILLIS: u64 = 100;

const RICH_WIDTH_ADJUST: u16 = 6;

const RICHER_TEXT_RESULT_WIDTH: u16 = RESULT_WIDTH + RICH_WIDTH_ADJUST;
const RICHER_TEXT_DISPLAY_WIDTH: u16 = DISPLAY_WIDTH + RICH_WIDTH_ADJUST;
const RICHER_TEXT_FIXED_WIDTH: u16 = FIXED_WIDTH + RICH_WIDTH_ADJUST;

const INDICATORS_WIDTH: u16 = 4;

pub struct Calculator<const RICHER: bool> {
    calculator: DumbCalculator,
    screen: DumbLineByLineScreen,
    key_map: Vec<Vec<char>>,
    selected_key_rc: (usize, usize),
    refresh_state: RefreshState<RICHER>,
}
impl<const RICHER: bool> Calculator<RICHER> {
    pub fn new_and_init() -> Self {
        let result_fixed_width = if RICHER {
            RICHER_TEXT_RESULT_WIDTH
        } else {
            RESULT_WIDTH
        };
        let display_fixed_width = if RICHER {
            RICHER_TEXT_DISPLAY_WIDTH
        } else {
            DISPLAY_WIDTH
        };
        let template_fixed_width = if RICHER {
            RICHER_TEXT_FIXED_WIDTH
        } else {
            FIXED_WIDTH
        };

        let mut line_temps = Vec::<DumbLineTemplate>::new();

        let mut comps = dlt_comps![dltc!("display", fixed_width = display_fixed_width)];
        let temp = DumbLineTemplate::new_fixed_width(template_fixed_width, &comps);
        line_temps.push(temp);

        if RICHER {
            let mut comps = dlt_comps![
                dltc!("indicators", fixed_width = INDICATORS_WIDTH),
                (
                    "〰️".repeat(7 /*9*/),
                    (template_fixed_width - INDICATORS_WIDTH) as usize
                ),
                " "
            ];
            let temp = DumbLineTemplate::new_fixed_width(template_fixed_width + 1, &comps);
            line_temps.push(temp);
        }

        let mut comps = dlt_comps![
            " ",
            Calculator::<RICHER>::_create_key('7', false),
            " ",
            Calculator::<RICHER>::_create_key('8', false),
            " ",
            Calculator::<RICHER>::_create_key('9', false),
            if RICHER { (" 🚪 ", 4) } else { (" | ", 3) },
            Calculator::<RICHER>::_create_key('C', true),
            " ",
        ];
        let keys_8 = Calculator::<RICHER>::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(template_fixed_width, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            " ",
            Calculator::<RICHER>::_create_key('4', false),
            " ",
            Calculator::<RICHER>::_create_key('5', false),
            " ",
            Calculator::<RICHER>::_create_key('6', false),
            if RICHER { (" 🚪 ", 4) } else { (" | ", 3) },
            Calculator::<RICHER>::_create_key('*', false),
            " ",
            Calculator::<RICHER>::_create_key('/', false),
            " ",
        ];
        let keys_5 = Calculator::<RICHER>::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(template_fixed_width, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            " ",
            Calculator::<RICHER>::_create_key('1', false),
            " ",
            Calculator::<RICHER>::_create_key('2', false),
            " ",
            Calculator::<RICHER>::_create_key('3', false),
            if RICHER { (" 🚪 ", 4) } else { (" | ", 3) },
            Calculator::<RICHER>::_create_key('+', false),
            " ",
            Calculator::<RICHER>::_create_key('-', false),
            " ",
        ];
        let keys_2 = Calculator::<RICHER>::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(template_fixed_width, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            " ",
            Calculator::<RICHER>::_create_key('%', false),
            " ",
            Calculator::<RICHER>::_create_key('0', false),
            " ",
            Calculator::<RICHER>::_create_key('.', false),
            if RICHER { ("  🚪 ", 4) } else { (" | ", 3) },
            Calculator::<RICHER>::_create_key('=', true),
            " ",
        ];
        let keys_0 = Calculator::<RICHER>::_scan_for_keys(&comps);
        let temp = DumbLineTemplate::new_fixed_width(template_fixed_width, &comps);
        line_temps.push(temp);

        if RICHER {
            let mut comps = dlt_comps![("〰️".repeat(9), (template_fixed_width) as usize), " "];
            let temp = DumbLineTemplate::new_fixed_width(template_fixed_width + 1, &comps);
            line_temps.push(temp);
            let mut comps =
                dlt_comps![dltc!("history", fixed_width = display_fixed_width)
                    .set_truncate_indicator("…:<<")];
            let temp = DumbLineTemplate::new_fixed_width(template_fixed_width, &comps);
            line_temps.push(temp);
        }

        let settings = if RICHER {
            LBLScreenSettings {
                line_prefix: Some("\t🧱 ".to_string()),
                line_suffix: Some("🧱 ".to_string()),
                top_line: Some(format!("\n\t{}", "🧱".repeat(FIXED_WIDTH as usize - 1))),
                bottom_line: Some(format!("\t{}\n", "🧱".repeat(FIXED_WIDTH as usize - 1))),
                ..LBLScreenSettings::default()
            }
        } else {
            LBLScreenSettings {
                line_prefix: Some("\t|".to_string()),
                line_suffix: Some("|".to_string()),
                top_line: Some(format!("\n\t{}", "=".repeat(FIXED_WIDTH as usize + 2))),
                bottom_line: Some(format!("\t{}\n", "=".repeat(FIXED_WIDTH as usize + 2))),
                ..LBLScreenSettings::default()
            }
        };
        let mut screen = DumbLineByLineScreen::new(line_temps, settings);
        println!();
        println!("* arrow keys to move selected key; space key to commit selected key");
        println!("* can press corresponding keys directly");
        if RICHER {
            println!("* can input brackets '(' and ')'; backspace to undo last calculator key");
            println!(
                "* note that 'c', '*', '/' and 'enter' are for 'C', 'x', '÷' and '=' respectively"
            );
        } else {
            println!("* note that 'c' is the same as 'C' and the enter key is the same as '='");
        }
        println!("* === to end, press ESC ===");

        screen.init();

        let key_map = vec![keys_8, keys_5, keys_2, keys_0];
        let key = '0';
        let key_pressed_coor = Calculator::<RICHER>::_get_key_coor(key, &key_map).unwrap();
        let calculator = if RICHER {
            DumbCalculator::new_ex(DumbCalculatorSettings {
                enable_undo: true,
                enable_history: true,
            })
        } else {
            DumbCalculator::new()
        };
        Self {
            calculator: calculator,
            screen: screen,
            key_map: key_map,
            selected_key_rc: key_pressed_coor,
            refresh_state: RefreshState {
                //display: String::from("0"),
                //indicators: None,
                selected_key: Some(key),
                highlight_selected: false,
                //result_fixed_width: result_fixed_width,
                //display_fixed_width: display_fixed_width,
            },
        }
    }
    fn _create_key(key: char, span_two: bool) -> MappedLineTempCompBuilder {
        let fixed_width = if RICHER {
            if span_two {
                5
            } else {
                2
            }
        } else {
            if span_two {
                3
            } else {
                1
            }
        };
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
    pub fn run(mut self) {
        self._refresh();
        let key = self.refresh_state.selected_key.unwrap();
        self.refresh_state.selected_key = Some(key);
        self._refresh_for_keys(&vec![key.to_string().as_ref()]);
        enable_raw_mode().unwrap();
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
                    KeyCode::Backspace => {
                        self._undo_commit();
                    }
                    KeyCode::Esc => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        disable_raw_mode().unwrap();
    }
    fn _refresh(&mut self) {
        let map_value_fn = |key: &str| -> Option<(String, u16)> {
            self.refresh_state.map_value(key, &self.calculator)
        };
        self.screen.refresh_ex(map_value_fn);
    }
    fn _refresh_for_keys(&mut self, keys: &Vec<&str>) {
        let map_value_fn = |key: &str| -> Option<(String, u16)> {
            self.refresh_state.map_value(key, &self.calculator)
        };
        self.screen.refresh_for_keys_ex(keys, map_value_fn);
    }
    fn _get_key_coor(
        key: char,
        key_map: &[Vec<char>], /*&Vec<Vec<char>>*/
    ) -> Option<(usize, usize)> {
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
        self._update_display();
    }
    fn _undo_commit(&mut self) {
        self.calculator.undo();
        self._update_display();
    }
    fn _update_display(&mut self) {
        if RICHER {
            self._refresh_for_keys(&vec!["display", "indicators", "history"]);
        } else {
            self._refresh_for_keys(&vec!["display"]);
        }
    }
    fn _select_and_enter_key(&mut self, key: char) {
        let key: char = key.to_ascii_uppercase();
        let key = if key == 'X' { '*' } else { key };
        let key_coor = Calculator::<RICHER>::_get_key_coor(key, &self.key_map);
        if let Some((row_idx, col_idx)) = key_coor {
            let key = self.key_map[self.selected_key_rc.0][self.selected_key_rc.1];
            self.refresh_state.selected_key = None;
            self._refresh_for_keys(&vec![key.to_string().as_ref()]);

            self.selected_key_rc = (row_idx, col_idx);
            let key = self.key_map[self.selected_key_rc.0][self.selected_key_rc.1];
            self.refresh_state.selected_key = Some(key);
            self._refresh_for_keys(&vec![key.to_string().as_ref()]);

            self._commit_key_selected();
        } else {
            // no key on calculator UI
            if RICHER {
                let mut need_update_display = false;
                if key == '(' || key == ')' {
                    self.calculator.push(key.to_string().as_str()).unwrap();
                    need_update_display = true;
                } else if key == 'N' {
                    self.calculator.push("neg");
                    need_update_display = true;
                }
                if need_update_display {
                    self._update_display();
                }
            }
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

struct RefreshState<const RICHER: bool> {
    // //result_fixed_width: u16,
    // //display_fixed_width: u16,
    // display: String,
    // indicators: Option<String>,
    selected_key: Option<char>,
    highlight_selected: bool,
}

impl<const RICHER: bool> RefreshState<RICHER> {
    //type VALUE = String;
    fn map_value(&self, key: &str, calculator: &DumbCalculator) -> Option<(String, u16)> {
        let result_fixed_width = if RICHER {
            RICHER_TEXT_RESULT_WIDTH
        } else {
            RESULT_WIDTH
        };
        let display_fixed_width = if RICHER {
            RICHER_TEXT_DISPLAY_WIDTH
        } else {
            DISPLAY_WIDTH
        };
        if key.len() == 1 {
            let current_key = self.selected_key;
            let key = key.chars().next().unwrap();
            let mut key_value = key.to_string();
            let mut key_width = 1;
            if RICHER {
                if key_value == "*" {
                    key_value = /*'✖'*//*'✱'*/'x'.to_string();
                    key_width = 1;
                } else if key_value == "/" {
                    key_value = "÷" /*'⟋'*/
                        .to_string();
                    key_width = 1;
                } else if key_value == "+" {
                    key_value = /*"✚"*/'+'.to_string();
                    key_width = 1;
                } else if key_value == "-" {
                    key_value = /*"⚊"*/'-'.to_string();
                    key_width = 1;
                } else if key_value == "=" {
                    key_value = "⚌".to_string();
                    key_width = 1;
                } else if key_value == "C" {
                    key_value = /*"🇦🇨"*/"Ｃ".to_string();
                    key_width = 2;
                } else if key_value == "%" {
                    key_value = "%".to_string();
                    key_width = 1;
                } else if key_value == "." {
                    key_value = "." /*"・"*/
                        .to_string();
                    key_width = 2;
                } else if key_value == "0" {
                    key_value = "０".to_string();
                    key_width = 2
                } else if key_value == "1" {
                    key_value = "１".to_string();
                    key_width = 2
                } else if key_value == "2" {
                    key_value = "２".to_string();
                    key_width = 2
                } else if key_value == "3" {
                    key_value = "３".to_string();
                    key_width = 2
                } else if key_value == "4" {
                    key_value = "４".to_string();
                    key_width = 2
                } else if key_value == "5" {
                    key_value = "５".to_string();
                    key_width = 2
                } else if key_value == "6" {
                    key_value = "６".to_string();
                    key_width = 2
                } else if key_value == "7" {
                    key_value = "７".to_string();
                    key_width = 2
                } else if key_value == "8" {
                    key_value = "８".to_string();
                    key_width = 2
                } else if key_value == "9" {
                    key_value = "９".to_string();
                    key_width = 2
                }
            }
            let key_value = match current_key {
                Some(current_key) if current_key == key => {
                    if self.highlight_selected {
                        format!("\x1B[7m{}\x1B[0m", key_value) // invert color
                    } else {
                        if true {
                            format!("\x1B[31m{}\x1B[0m", key_value) // red
                        } else {
                            format!("\x1B[4m{}\x1B[0m", key_value) // underline
                        }
                    }
                }
                _ => key_value,
            };
            Some((key_value, key_width))
        } else if key == "display" {
            let result_fixed_width = if RICHER {
                RICHER_TEXT_RESULT_WIDTH
            } else {
                RESULT_WIDTH
            };
            let mut display_result = calculator.get_display_sized(result_fixed_width as usize);
            //let mut display_result = self.display.clone();
            if display_result.len() < result_fixed_width as usize {
                let room = result_fixed_width - display_result.len() as u16;
                display_result = format!("{}{}", " ".repeat(room as usize), display_result);
            }
            let display_result = format!("\x1B[7m {} \x1B[0m", display_result);
            Some((display_result, display_fixed_width))
        } else if RICHER && key == "indicators" {
            let operator = calculator.get_last_operator();
            let mut ind1 = match operator {
                Some(operator) => match operator.as_str() {
                    "+" => "+",
                    "-" => "-",
                    "*" => "x",
                    "/" => "÷",
                    _ => "",
                },
                None => "",
            };
            let mut ind2 = match calculator.count_opened_brackets() {
                // actually, no way to input bracket yet
                1 => "⑴", // ⑴ ⑵ ⑶ ⑷ ⑸ ⑹ ⑺ ⑻ ⑼ ⑽ ⑾ ⑿ ⒀ ⒁ ⒂ ⒃ ⒄ ⒅ ⒆ ⒇
                2 => "⑵",
                3 => "⑶",
                4 => "⑷",
                5 => "⑸",
                6 => "⑹",
                7 => "⑺",
                8 => "⑻",
                9 => "⑼",
                10 => "⑽",
                _ => "",
            };
            let indicators = if !ind1.is_empty() || !ind2.is_empty() {
                if ind1.is_empty() {
                    ind1 = " "
                }
                if ind2.is_empty() {
                    ind2 = " "
                }
                Some(format!("{} {} ", ind1, ind2))
            } else {
                None
            };
            // if let Some(indicators) = indicators {
            //     self.refresh_state.indicators = Some(indicators.to_string());
            // } else {
            //     self.refresh_state.indicators = None;
            // }
            let indicators = match indicators {
                Some(indicators) => indicators.clone(),
                None => "〰️〰️".to_string(),
            };
            Some((indicators, INDICATORS_WIDTH))
        } else if RICHER && key == "history" {
            let history = calculator.get_history();
            if let Some(history) = history {
                if true {
                    let mut hist = String::new();
                    let mut hist_len = 0;
                    for h in history {
                        if DumbCalcProcessor::is_unary_operator(h) {
                            hist_len += h.len() + 2;
                            hist.push_str(format!("_{}_", h).as_str());
                        } else {
                            hist.push_str(h.as_str());
                            hist_len += h.len();
                        }
                    }
                    Some((hist, hist_len as u16))
                } else {
                    let history = history.join("");
                    let history_len = history.len();
                    Some((history, history_len as u16))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
