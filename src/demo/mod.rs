#![deny(warnings)]

//! Home for `rusty_dumb_tools` demo.

pub mod demo_arg;
pub mod demo_calc;
pub mod demo_calculator;
pub mod demo_calculator_gui;
pub mod demo_ltemp;

pub mod test_arg;
pub mod test_calc;
pub mod test_calculator;
pub mod test_ltemp;

use crate::{
    arg::{DumbArgBuilder, DumbArgParser},
    dap_arg,
};

use crate::demo::{
    demo_arg::handle_demo_arg,
    demo_calc::{create_demo_parser_calc, handle_demo_calc, handle_demo_calc_repl},
};

use self::{
    demo_calculator::handle_demo_calculator, demo_calculator_gui::handle_demo_calculator_gui,
    demo_ltemp::handle_demo_ltemp,
};

///
/// run the demo, which is a command-line program that allows you to choose from a list of sub-demos
/// * `in_args` - if None, parse arguments from command-line; otherwise, parse from `in_args`.
///
/// sub-demos:
/// * `calc`: see [`crate::demo::demo_calc::handle_demo_calc`]
/// * `calc-repl`: see [`crate::demo::demo_calc::handle_demo_calc_repl`]
/// * `arg`: see [`crate::demo::demo_arg::handle_demo_arg`]
pub fn run_demo(in_args: Option<Vec<&str>>) {
    let mut parser = create_demo_parser();
    if in_args.is_some() {
        let in_args = in_args.unwrap();
        parser.process_args(in_args);
    } else {
        parser.parse_args();
    }
    handle_sub_demo(parser);
}

/// create a [`DumbArgParser`] for the demo; it is supposed to be called by [`run_demo`]
pub fn create_demo_parser() -> DumbArgParser {
    let mut parser = DumbArgParser::new();
    parser.set_description("Demos of rusty_dumb_tools.");
    dap_arg!("demo", value = "calc")
        .set_description("a demo")
        .set_with_desc_enums(vec![
            "calc:DumbCalcProcessor command-line input demo",
            "calc-repl:DumbCalcProcessor REPL demo",
            "calculator:DumbCalculator text-based UI demo",
            "calculator-gui:DumbCalculator GUI demo",
            "ltemp:DumbLineTemplate demo",
            "arg:DumbArgParser demo (more like debugging)",
        ])
        .set_rest()
        .add_to(&mut parser)
        .unwrap();
    parser
}

/// handle running a sub-demo; to be called by [`run_demo`]
pub fn handle_sub_demo(parser: DumbArgParser) {
    let demo = match parser.get::<String>("demo") {
        Some(t) => t,
        None => {
            panic!("No demo specified.");
        }
    };
    match demo.as_str() {
        "calc" => {
            let mut demo_parser = create_demo_parser_calc();
            parser.process_rest_args("demo", &mut demo_parser);
            handle_demo_calc(demo_parser);
        }
        "calc-repl" => {
            handle_demo_calc_repl();
        }
        "calculator" => {
            handle_demo_calculator();
        }
        "calculator-gui" => {
            handle_demo_calculator_gui();
        }
        "ltemp" => {
            let mut sub_demo_parser = demo_ltemp::create_demo_ltemp_parser();
            parser.process_rest_args("demo", &mut sub_demo_parser);
            handle_demo_ltemp(sub_demo_parser);
        }
        "arg" => {
            let mut sub_demo_parser = demo_arg::create_debug_arg_parser();
            parser.process_rest_args("demo", &mut sub_demo_parser);
            handle_demo_arg(sub_demo_parser);
        }
        _ => panic!("Unknown sub-demo: {}", demo),
    };
}
