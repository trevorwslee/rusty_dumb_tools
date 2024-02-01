//! core [`crate::calc`] sub-demo code

#![deny(warnings)]

use std::io;
use std::io::Write;

use crate::{
    arg::{DumbArgBuilder, DumbArgParser},
    calc::{self, CalcResult},
    dap_arg,
};

pub fn create_demo_parser_calc() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("DumbCalcProcessor command-line input demo.");
    dap_arg!("input", value = "123")
        .set_multi()
        .set_description("infix expression")
        .add_to(&mut parser)
        .unwrap();
    parser
}

pub fn handle_demo_calc(parser: DumbArgParser) {
    let input = parser.get_multi::<String>("input").unwrap();
    let mut calc = calc::DumbCalcProcessor::new();
    for i in input {
        calc.parse_and_push(&i).unwrap()
    }
    println!("|");
    match calc.eval() {
        Ok(_) => {
            println!("| = {}", calc.get_result());
        }
        Err(e) => {
            println!("| Error: {}", e);
        }
    }
    println!("|");
}

pub fn handle_demo_calc_repl() {
    println!();
    println!("* enter an infix expression");
    println!("* can split the infix expression into multiple lines; e.g. a \"unit\" a line");
    println!("* finally, enter \"=\" (or an empty line) to evaluate it");
    println!("* can then continue to enter another infix expression ...");
    println!();
    let mut calc = calc::DumbCalcProcessor::new();
    let mut units = String::new();

    loop {
        print!("> {}", units);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut unit = input.trim();
        if unit == "" {
            unit = "=";
        }

        if unit.to_lowercase() == "c" {
            units.clear();
            calc.reset();
            continue;
        }

        if unit == "=" {
            calc.evaluate()
        } else {
            let push_res = calc.parse_and_push(unit);
            match push_res {
                Ok(_) => {}
                Err(e) => {
                    println!("| Error: {}", e);
                    continue;
                }
            }
        }

        units.push_str(unit);
        units.push(' ');

        let result = calc.get_result();
        let sep = match result {
            CalcResult::Final(_) => {
                units.clear();
                "="
            }
            CalcResult::Intermediate(_) => ":",
            CalcResult::Error(_) => {
                units.clear();
                calc.reset();
                "!"
            }
        };
        println!("| {} {}", sep, result);
    }
}
