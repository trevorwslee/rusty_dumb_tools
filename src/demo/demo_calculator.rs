//! core [`crate::calculator`] demo code
//!
//! ***work in progress***

#![deny(warnings)]
#![allow(unused)]

use std::{collections::HashMap, thread, time::Duration};

use crate::{
    dlt_comps, dltc,
    lblscreen::{DumbLineByLineScreen, LBLScreenMapValueTrait, LBLScreenSettings},
    ltemp::{
        DumbLineTemplate, EscapedLineTempComp, LineTempComp, LineTempCompTrait, MappedLineTempComp,
        MappedLineTempCompBuilder, MappedLineTempCompSettings, FLEXIBLE_WIDTH_EX,
    },
};

pub fn handle_demo_calculator() {
    let mut ui = CalculatorUI::new_and_init();
    //ui.refresh();

    let keys = vec!["7"];
    ui.state.pressed_key = Some('7');
    ui.state.highlight_pressed_key = true;
    ui.refresh_for_keys(&keys);
    thread::sleep(Duration::from_millis(500));
    ui.state.highlight_pressed_key = false;
    ui.refresh_for_keys(&keys)
}

const RESULT_WIDTH: u16 = 11;
const DISPLAY_WIDTH: u16 = RESULT_WIDTH + 2;
const FIXED_WIDTH: u16 = DISPLAY_WIDTH;

struct CalculatorUI {
    screen: DumbLineByLineScreen,
    state: CalculatorUIState,
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
        screen.init();

        Self {
            screen,
            state: CalculatorUIState {
                pressed_key: None,
                highlight_pressed_key: false,
            },
        }
    }
    fn _create_key(key: char, fixed_width: u16) -> MappedLineTempCompBuilder {
        dltc!(&key.to_string(), fixed_width = fixed_width, align = 'C')
    }
    fn refresh(&mut self) {
        self.screen.refresh(&self.state);
    }
    fn refresh_for_keys(&mut self, keys: &Vec<&str>) {
        self.screen.refresh_for_keys(keys, &self.state);
    }
}

struct CalculatorUIState {
    pressed_key: Option<char>,
    highlight_pressed_key: bool,
}

impl LBLScreenMapValueTrait for CalculatorUIState {
    type VALUE = String;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, u16)> {
        if key.len() == 1 {
            let pressed_key = self.pressed_key;
            let key_char = key.chars().next().unwrap();
            let mut key_value = key_char.to_string();
            if let Some(pressed_key) = pressed_key {
                if pressed_key == key_char {
                    if self.highlight_pressed_key {
                        //key_value = format!("\x1B[1D\x1B[7m {} \x1B[0m", key_value);
                        key_value = format!("\x1B[7m{}\x1B[0m", key_value);
                    } else {
                        key_value = format!("\x1B[4m{}\x1B[0m", key_value);
                    }
                    //print("\x1b[4mThis will be underlined.\x1b[0m This will not.")
                }
            };
            Some((key_value, 1))
        } else if key == "display" {
            let result = 0.0;
            //let result = -21.2345;
            //let result = -0.123456789123456789;
            //let result = -1234567891234.0;
            let mut display_result = format!("{}", result);
            if display_result.len() < RESULT_WIDTH as usize {
                let room = RESULT_WIDTH - display_result.len() as u16;
                display_result = format!("{}{}", " ".repeat(room as usize), display_result);
            } else {
                let room = DISPLAY_WIDTH - (if result < 0.0 { 5 } else { 4 });
                display_result = format!("{:.*}", room as usize, result);
                if display_result.len() > DISPLAY_WIDTH as usize {
                    let room = DISPLAY_WIDTH - (if result < 0.0 { 8 } else { 7 });
                    display_result = format!("{:.*e}", room as usize, result);
                }
            }
            let display_result = format!("\x1B[7m {} \x1B[0m", display_result);
            Some((display_result, DISPLAY_WIDTH))
        } else {
            None
        }
    }
}
