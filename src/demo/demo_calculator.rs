//! core [`crate::calculator`] demo code
//!
//! ***work in progress***

#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::{
    dlt_comps, dltc,
    lblscreen::{DumbLineByLineScreen, LBLScreenMapValueTrait, LBLScreenSettings},
    ltemp::{
        DumbLineTemplate, EscapedLineTempComp, LineTempComp, LineTempCompTrait, MappedLineTempComp,
        MappedLineTempCompBuilder, MappedLineTempCompSettings, FLEXIBLE_WIDTH_EX,
    },
};

pub fn handle_demo_calculator() {
    let mut ui = CalculatorUI::new();
    ui.refresh();
    ui.refresh();
}

const RESULT_WIDTH: u16 = 11;
const DISPLAY_WIDTH: u16 = RESULT_WIDTH + 2;
const FIXED_WIDTH: u16 = DISPLAY_WIDTH;

struct CalculatorUI {
    screen: DumbLineByLineScreen,
    state: CalculatorUIState,
}
impl CalculatorUI {
    fn new() -> Self {
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

        // let map_value_fn = |key: &str, state: &()| -> Option<(String, u16)> {
        //     if key.len() == 1 {
        //         let value = key.to_string();
        //         let value_len = value.len();
        //         Some((value, value_len as u16))
        //     } else if key == "display" {
        //         let result = 0.0;
        //         //let result = -21.2345;
        //         //let result = -0.123456789123456789;
        //         //let result = -1234567891234.0;
        //         let mut display_result = format!("{}", result);
        //         if display_result.len() < RESULT_WIDTH as usize {
        //             let room = RESULT_WIDTH - display_result.len() as u16;
        //             display_result = format!("{}{}", " ".repeat(room as usize), display_result);
        //         } else {
        //             let room = DISPLAY_WIDTH - (if result < 0.0 { 5 } else { 4 });
        //             display_result = format!("{:.*}", room as usize, result);
        //             if display_result.len() > DISPLAY_WIDTH as usize {
        //                 let room = DISPLAY_WIDTH - (if result < 0.0 { 8 } else { 7 });
        //                 display_result = format!("{:.*e}", room as usize, result);
        //             }
        //         }
        //         let display_result = format!("\x1B[7m {} \x1B[0m", display_result);
        //         Some((display_result, DISPLAY_WIDTH))
        //     } else {
        //         None
        //     }
        // };

        let settings = LBLScreenSettings {
            line_prefix: Some("\t|".to_string()),
            line_suffix: Some("|".to_string()),
            top_line: Some(format!("\n\t{}", "=".repeat(FIXED_WIDTH as usize + 2))),
            bottom_line: Some(format!("\t{}\n", "=".repeat(FIXED_WIDTH as usize + 2))),
            screen_height_adjustment: 2,
            ..LBLScreenSettings::default()
        };
        let screen = DumbLineByLineScreen::new(line_temps, settings);

        Self { screen, state: CalculatorUIState{} }
    }
    fn _create_key(key: char, fixed_width: u16) -> MappedLineTempCompBuilder {
        dltc!(&key.to_string(), fixed_width = fixed_width, align = 'C')
    }
    fn refresh(&mut self) {
        self.screen.refresh(&self.state);
    }
}

struct CalculatorUIState {

} 

impl LBLScreenMapValueTrait for CalculatorUIState {
    type VALUE = String;
    fn map_value(&self, key: &str) -> Option<(Self::VALUE, u16)> {
        if key.len() == 1 {
            let value = key.to_string();
            let value_len = value.len();
            Some((value, value_len as u16))
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

// struct CalculatorUI2 {
//     line_temps: Vec<DumbLineTemplate>,
// }
// impl CalculatorUI2 {
//     fn update(&self) {
//         let map_value_fn = |key: &str| -> Option<(String, u16)> {
//             if key.len() == 1 {
//                 let value = key.to_string();
//                 let value_len = value.len();
//                 Some((value, value_len as u16))
//             } else if key == "display" {
//                 let result = 0.0;
//                 //let result = -21.2345;
//                 //let result = -0.123456789123456789;
//                 //let result = -1234567891234.0;
//                 let mut display_result = format!("{}", result);
//                 if display_result.len() < RESULT_WIDTH as usize {
//                     let room = RESULT_WIDTH - display_result.len() as u16;
//                     display_result = format!("{}{}", " ".repeat(room as usize), display_result);
//                 } else {
//                     let room = DISPLAY_WIDTH - (if result < 0.0 { 5 } else { 4 });
//                     display_result = format!("{:.*}", room as usize, result);
//                     if display_result.len() > DISPLAY_WIDTH as usize {
//                         let room = DISPLAY_WIDTH - (if result < 0.0 { 8 } else { 7 });
//                         display_result = format!("{:.*e}", room as usize, result);
//                     }
//                 }
//                 let display_result = format!("\x1B[7m {} \x1B[0m", display_result);
//                 Some((display_result, DISPLAY_WIDTH))
//             } else {
//                 None
//             }
//         };
//         println!("{}", "=".repeat(FIXED_WIDTH as usize + 2)); // top line
//         for line_temp in &self.line_temps {
//             let line = line_temp.format_ex(map_value_fn).unwrap();
//             println!("|{}|", line); // | is the line prefix and suffix
//         }
//         println!("{}", "=".repeat(FIXED_WIDTH as usize + 2)); // bottom line
//     }
// }

// impl CalculatorUI2 {
//     fn new() -> Self {
//         let mut line_temps = Vec::<DumbLineTemplate>::new();

//         let mut comps = dlt_comps![dltc!("display", fixed_width = DISPLAY_WIDTH)];
//         let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
//         line_temps.push(temp);

//         let mut comps = dlt_comps![
//             " ",
//             CalculatorUI2::_create_key('7', 1),
//             " ",
//             CalculatorUI2::_create_key('8', 1),
//             " ",
//             CalculatorUI2::_create_key('9', 1),
//             " | ",
//             CalculatorUI2::_create_key('C', 3),
//             " "
//         ];
//         let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
//         line_temps.push(temp);

//         let mut comps = dlt_comps![
//             " ",
//             CalculatorUI2::_create_key('4', 1),
//             " ",
//             CalculatorUI2::_create_key('5', 1),
//             " ",
//             CalculatorUI2::_create_key('6', 1),
//             " | ",
//             CalculatorUI2::_create_key('*', 1),
//             " ",
//             CalculatorUI2::_create_key('/', 1),
//             " "
//         ];
//         let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
//         line_temps.push(temp);

//         let mut comps = dlt_comps![
//             " ",
//             CalculatorUI2::_create_key('1', 1),
//             " ",
//             CalculatorUI2::_create_key('2', 1),
//             " ",
//             CalculatorUI2::_create_key('3', 1),
//             " | ",
//             CalculatorUI2::_create_key('+', 1),
//             " ",
//             CalculatorUI2::_create_key('-', 1),
//             " "
//         ];
//         let temp = DumbLineTemplate::new_fixed_width(FIXED_WIDTH as u16, &comps);
//         line_temps.push(temp);

//         let mut comps = dlt_comps![
//             " ",
//             CalculatorUI2::_create_key('%', 1),
//             " ",
//             CalculatorUI2::_create_key('0', 1),
//             " ",
//             CalculatorUI2::_create_key('.', 1),
//             " | ",
//             CalculatorUI2::_create_key('=', 3),
//             " "
//         ];
//         let temp = DumbLineTemplate::new_fixed_width(DISPLAY_WIDTH as u16, &comps);
//         line_temps.push(temp);

//         Self { line_temps }
//     }
//     fn _create_key_comp(key: char, width: u16) -> LineTempComp {
//         let settings = MappedLineTempCompSettings {
//             min_width: width,
//             max_width: width,
//             ..Default::default()
//         };
//         let comp = MappedLineTempComp::new(&key.to_string(), &settings);
//         LineTempComp::Mapped(comp)
//     }
//     // fn _create_key(key: char, fixed_width: u16) -> MappedLineTempCompBuilder {
//     //     dltc!(&key.to_string(), fixed_width = fixed_width, align = 'C')
//     // }
// }
