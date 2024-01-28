//! core [`crate::calculator`] demo code
//!
//! ***work in progress***

#![deny(warnings)]
#![allow(unused)]

use std::collections::HashMap;

use crate::{
    dlt_comps, dltc,
    ltemp::{
        DumbLineTemplate, EscapedLineTempComp, LineTempComp, LineTempCompTrait, MappedLineTempComp,
        MappedLineTempCompBuilder, MappedLineTempCompSettings, FLEXIBLE_WIDTH_EX,
    },
};

pub fn handle_demo_calculator() {
    let ui = CalculatorUI::new();
    ui.update();
}


const FIXED_WIDTH: u16 = 15;


struct CalculatorUI {
    line_temps: Vec<DumbLineTemplate>,
}
impl CalculatorUI {
    fn update(&self) {
        let map_value_provide_fn = |key: &str| -> Option<(String, u16)> {
            if key.len() == 1 {
                let value = key.to_string();
                let value_len = value.len();
                Some((value, value_len as u16))
            } else if key == "display" {
                Some(("\x1B[7m           0 \x1B[0m".to_string(), 13))
            } else {
                None
            }
        };
        println!("{}", "=".repeat(FIXED_WIDTH as usize));
        for line_temp in &self.line_temps {
            let line = line_temp.format_ex(map_value_provide_fn).unwrap();
            println!("{}", line);
        }
        println!("{}", "=".repeat(FIXED_WIDTH as usize));
    }
}

impl CalculatorUI {
    fn new() -> Self {
        let mut line_temps = Vec::<DumbLineTemplate>::new();

        let mut comps = dlt_comps!["|", dltc!("display", fixed_width = 13), "|"];
        let temp = DumbLineTemplate::new_fixed(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            "| ",
            CalculatorUI::_create_key('7', 1),
            " ",
            CalculatorUI::_create_key('8', 1),
            " ",
            CalculatorUI::_create_key('9', 1),
            " | ",
            CalculatorUI::_create_key('C', 3),
            " |"
        ];
        let temp = DumbLineTemplate::new_fixed(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            "| ",
            CalculatorUI::_create_key('4', 1),
            " ",
            CalculatorUI::_create_key('5', 1),
            " ",
            CalculatorUI::_create_key('6', 1),
            " | ",
            CalculatorUI::_create_key('*', 1),
            " ",
            CalculatorUI::_create_key('/', 1),
            " |"
        ];
        let temp = DumbLineTemplate::new_fixed(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            "| ",
            CalculatorUI::_create_key('1', 1),
            " ",
            CalculatorUI::_create_key('2', 1),
            " ",
            CalculatorUI::_create_key('3', 1),
            " | ",
            CalculatorUI::_create_key('+', 1),
            " ",
            CalculatorUI::_create_key('1', 1),
            " |"
        ];
        let temp = DumbLineTemplate::new_fixed(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        let mut comps = dlt_comps![
            "| ",
            CalculatorUI::_create_key('%', 1),
            " ",
            CalculatorUI::_create_key('0', 1),
            " ",
            CalculatorUI::_create_key('.', 1),
            " | ",
            CalculatorUI::_create_key('=', 3),
            " |"
        ];
        let temp = DumbLineTemplate::new_fixed(FIXED_WIDTH as u16, &comps);
        line_temps.push(temp);

        Self { line_temps }
    }
    // fn _create_key_comp(key: char, width: u16) -> LineTempComp {
    //     let settings = MappedLineTempCompSettings {
    //         min_width: width,
    //         max_width: width,
    //         ..Default::default()
    //     };
    //     let comp = MappedLineTempComp::new(&key.to_string(), &settings);
    //     LineTempComp::Mapped(comp)
    // }
    fn _create_key(key: char, fixed_width: u16) -> MappedLineTempCompBuilder {
        dltc!(&key.to_string(), fixed_width = fixed_width, align = 'C')
    }
}
